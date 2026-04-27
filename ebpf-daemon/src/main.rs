use anyhow::{Context, Result};
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use libbpf_rs::{MapFlags, RingBufferBuilder};
use rusqlite::Connection;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::net::Ipv4Addr;
use std::os::fd::{AsFd, AsRawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

// Include the generated skeleton
mod xdp_filter {
    include!(concat!(env!("OUT_DIR"), "/xdp_filter.skel.rs"));
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AttackEvent {
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    attack_type: u8,
    packet_count: u64,
    timestamp: u64,
}

/// rate_config must match the C struct in xdp_filter.bpf.c exactly
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RateConfig {
    syn_flood_threshold: u32,
    port_scan_threshold: u32,
    icmp_flood_threshold: u32,
    http_flood_threshold: u32,
}

/// Log deduplication: track last report time per (src_ip, attack_type)
struct EventDeduplicator {
    last_seen: HashMap<(u32, u8), Instant>,
    cooldown: Duration,
}

impl EventDeduplicator {
    fn new(cooldown_secs: u64) -> Self {
        Self {
            last_seen: HashMap::new(),
            cooldown: Duration::from_secs(cooldown_secs),
        }
    }

    /// Returns true if this event should be logged (not a duplicate)
    fn should_log(&mut self, src_ip: u32, attack_type: u8) -> bool {
        let key = (src_ip, attack_type);
        let now = Instant::now();

        if let Some(last) = self.last_seen.get(&key) {
            if now.duration_since(*last) < self.cooldown {
                return false; // Still within cooldown — suppress
            }
        }

        self.last_seen.insert(key, now);
        true
    }
}

fn int_to_ip(ip: u32) -> String {
    let bytes = ip.to_be_bytes();
    // ip->saddr is in network byte order (big-endian), stored as u32
    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
}

fn protocol_name(proto: u8) -> &'static str {
    match proto {
        6 => "TCP",
        17 => "UDP",
        1 => "ICMP",
        _ => "Unknown",
    }
}

fn handle_event(
    data: &[u8],
    db_conn: &Connection,
    dedup: &Mutex<EventDeduplicator>,
) -> i32 {
    if data.len() != std::mem::size_of::<AttackEvent>() {
        error!(
            "Invalid event size: {} (expected {})",
            data.len(),
            std::mem::size_of::<AttackEvent>()
        );
        return 0;
    }

    // Safety: we checked the length
    let event: AttackEvent =
        unsafe { std::ptr::read_unaligned(data.as_ptr() as *const AttackEvent) };

    // Deduplicate log messages — only log 1 event per IP per attack type per cooldown window
    {
        let mut dedup = dedup.lock().unwrap();
        if !dedup.should_log(event.src_ip, event.attack_type) {
            return 0; // Suppress duplicate — already logged recently
        }
    }

    let src_ip = int_to_ip(event.src_ip);
    let dst_ip = int_to_ip(event.dst_ip);
    let proto = protocol_name(event.protocol);

    let attack_name = match event.attack_type {
        1 => "SYN Flood",
        2 => "Port Scan",
        3 => "ICMP Flood",
        _ => "Unknown Attack",
    };

    let severity = match event.attack_type {
        1 => "Critical",
        2 => "High",
        3 => "Medium",
        _ => "Medium",
    };

    info!(
        "Detected {} from {} to {}:{} (protocol: {}, distinct_count: {}) — IP auto-blacklisted for 1 hour",
        attack_name, src_ip, dst_ip, event.dst_port, proto, event.packet_count
    );

    // Insert into DB with all columns populated
    let now = chrono::Utc::now().to_rfc3339();
    let res = db_conn.execute(
        "INSERT INTO attacks (attack_type, source_ip, source_port, destination_ip, destination_port, protocol, severity, packet_count, details, blocked, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            attack_name,
            src_ip,
            event.src_port as i32,
            dst_ip,
            event.dst_port as i32,
            proto,
            severity,
            event.packet_count as i64,
            format!(
                "XDP_DROP: {} SYNs detected, attacker blacklisted for 1 hour",
                event.packet_count
            ),
            true, // XDP dropped + auto-blacklisted
            now,
        ],
    );

    if let Err(e) = res {
        error!("Failed to log attack to DB: {}", e);
    }

    // Also insert into the blacklist table so the UI can see it
    let expires_at = (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
    let bl_res = db_conn.execute(
        "INSERT OR REPLACE INTO blacklist (ip, reason, auto_generated, expires_at, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            src_ip,
            format!("Auto-blocked: {} detected", attack_name),
            true,
            expires_at,
            now,
        ],
    );

    if let Err(e) = bl_res {
        error!("Failed to update blacklist in DB: {}", e);
    }

    0
}

fn init_config_map(skel: &xdp_filter::XdpFilterSkel) -> Result<()> {
    let cfg = RateConfig {
        syn_flood_threshold: 100,   // D2.1: >100 SYNs/sec from single IP
        port_scan_threshold: 200,   // D2.2: >200 DISTINCT ports in 10 seconds (raised: browsers use many ephemeral ports)
        icmp_flood_threshold: 500,  // D2.4: >500 pings/sec
        http_flood_threshold: 1000, // D2.3: >1000 req/sec
    };

    let key = 0u32.to_ne_bytes();
    let value = unsafe {
        std::slice::from_raw_parts(
            &cfg as *const RateConfig as *const u8,
            std::mem::size_of::<RateConfig>(),
        )
    };

    skel.maps()
        .config()
        .update(&key, value, MapFlags::ANY)
        .context("Failed to initialize config map")?;

    info!(
        "Config map initialized: syn_flood={}, port_scan={}, icmp_flood={}, http_flood={}",
        cfg.syn_flood_threshold,
        cfg.port_scan_threshold,
        cfg.icmp_flood_threshold,
        cfg.http_flood_threshold
    );

    Ok(())
}

/// Convert an IPv4 address to the u32 representation used by BPF maps (network byte order)
fn ip_to_bpf_u32(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

/// Whitelist essential IPs in the BPF whitelist map to prevent self-blocking.
/// This adds:
/// - Loopback (127.0.0.1)
/// - All local interface IPs
/// - Default gateway IP
/// - DNS server IPs from /etc/resolv.conf
fn whitelist_essential_ips(skel: &xdp_filter::XdpFilterSkel) -> Result<()> {
    let maps = skel.maps();
    let whitelist_map = maps.whitelist();
    let flag: u8 = 1;
    let flag_bytes = [flag];

    // Helper closure to whitelist a single IP
    let whitelist_ip = |ip: Ipv4Addr, reason: &str| {
        let key = ip_to_bpf_u32(ip).to_ne_bytes();
        match whitelist_map.update(&key, &flag_bytes, MapFlags::ANY) {
            Ok(()) => info!("Whitelisted {} ({})", ip, reason),
            Err(e) => warn!("Failed to whitelist {}: {}", ip, e),
        }
    };

    // 1. Always whitelist loopback
    whitelist_ip(Ipv4Addr::new(127, 0, 0, 1), "loopback");

    // 2. Whitelist all local interface IPs
    // Parse from /proc/net/fib_trie or use simpler approach via ip command output
    if let Ok(output) = std::process::Command::new("hostname").arg("-I").output() {
        if let Ok(ips_str) = String::from_utf8(output.stdout) {
            for ip_str in ips_str.split_whitespace() {
                if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                    whitelist_ip(ip, "local interface");
                }
            }
        }
    }

    // 3. Whitelist default gateway from /proc/net/route
    if let Ok(route_content) = fs::read_to_string("/proc/net/route") {
        for line in route_content.lines().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 {
                let dest = fields[1];
                let gateway = fields[2];
                // Default route has destination 00000000
                if dest == "00000000" && gateway != "00000000" {
                    if let Ok(gw_u32) = u32::from_str_radix(gateway, 16) {
                        let gw_bytes = gw_u32.to_le_bytes(); // /proc/net/route is in little-endian hex
                        let gw_ip = Ipv4Addr::new(gw_bytes[0], gw_bytes[1], gw_bytes[2], gw_bytes[3]);
                        whitelist_ip(gw_ip, "default gateway");
                    }
                }
            }
        }
    }

    // 4. Whitelist DNS servers from /etc/resolv.conf
    if let Ok(resolv_content) = fs::read_to_string("/etc/resolv.conf") {
        for line in resolv_content.lines() {
            let line = line.trim();
            if line.starts_with("nameserver") {
                if let Some(ip_str) = line.split_whitespace().nth(1) {
                    if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                        whitelist_ip(ip, "DNS server");
                    }
                }
            }
        }
    }

    // 5. Whitelist common VirtualBox/NAT gateway ranges
    whitelist_ip(Ipv4Addr::new(10, 0, 2, 2), "VirtualBox NAT gateway");
    whitelist_ip(Ipv4Addr::new(10, 0, 2, 3), "VirtualBox NAT DNS");

    info!("Essential IP whitelisting complete");
    Ok(())
}

fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args: Vec<String> = env::args().collect();
    let iface = if args.len() > 1 {
        &args[1]
    } else {
        "eth0"
    };

    info!("Starting ZeroTrust eBPF Daemon on interface {}", iface);

    // Ensure we are root
    if unsafe { libc::geteuid() } != 0 {
        anyhow::bail!("This daemon must be run as root to attach eBPF programs.");
    }

    // Open DB
    // Assuming the daemon is run from the workspace root where `zerotrust_mesh.db` is located (or standard app data path).
    // Let's use the current directory for simplicity in the demo, or an absolute path.
    let cwd = std::env::current_dir()?;
    let mut db_path = if cwd.ends_with("ebpf-daemon") {
        cwd.parent().unwrap().to_path_buf()
    } else {
        cwd
    };
    db_path.push(".dev-data");
    db_path.push("zerotrust-mesh");
    db_path.push("data");
    db_path.push("zerotrust.db");
    
    info!("Connecting to SQLite database at {}", db_path.display());
    let db_conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open DB at {}", db_path.display()))?;

    // Load BPF
    let skel_builder = xdp_filter::XdpFilterSkelBuilder::default();
    let open_skel = skel_builder.open()?;
    let skel = open_skel.load()?;

    // *** FIX BUG 1: Initialize the config map with proper thresholds ***
    init_config_map(&skel)?;

    // *** FIX: Whitelist local IPs, gateway, and DNS to prevent self-blocking ***
    whitelist_essential_ips(&skel)?;

    // Get ifindex
    let c_iface = std::ffi::CString::new(iface)?;
    let ifindex = unsafe { libc::if_nametoindex(c_iface.as_ptr()) };
    if ifindex == 0 {
        anyhow::bail!("Failed to get interface index for {}", iface);
    }

    // Attach XDP in SKB mode explicitly for VirtualBox compatibility
    let fd = skel.progs().xdp_filter().as_fd().as_raw_fd();
    let err = unsafe {
        // 2 is XDP_FLAGS_SKB_MODE
        libbpf_sys::bpf_xdp_attach(ifindex as i32, fd, 2, std::ptr::null())
    };
    
    if err < 0 {
        anyhow::bail!("Failed to attach XDP in SKB mode to interface {}: error {}", iface, err);
    }

    info!("Successfully attached XDP program to {}", iface);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Create event deduplicator — suppress duplicate logs within 5-second window
    let dedup = Arc::new(Mutex::new(EventDeduplicator::new(5)));

    // Open RingBuffer
    let mut builder = RingBufferBuilder::new();
    let maps = skel.maps();
    let dedup_clone = dedup.clone();
    builder.add(maps.events(), move |data| {
        handle_event(data, &db_conn, &dedup_clone)
    })?;
    let ring_buffer = builder.build()?;

    info!("Listening for attack events. Press Ctrl+C to exit.");
    warn!(
        "Thresholds: SYN flood > 100/sec, Port scan > 200 distinct ports/10sec, ICMP > 500/sec"
    );

    while running.load(Ordering::SeqCst) {
        ring_buffer.poll(Duration::from_millis(100))?;
    }

    // Graceful cleanup: detach XDP program so it doesn't keep blocking after daemon exits
    info!("Detaching XDP program from {}...", iface);
    let detach_err = unsafe {
        libbpf_sys::bpf_xdp_detach(ifindex as i32, 2, std::ptr::null()) // 2 = XDP_FLAGS_SKB_MODE
    };
    if detach_err < 0 {
        error!("Failed to detach XDP program: error {}", detach_err);
    } else {
        info!("XDP program detached successfully. Network is back to normal.");
    }

    info!("Exiting...");
    Ok(())
}
