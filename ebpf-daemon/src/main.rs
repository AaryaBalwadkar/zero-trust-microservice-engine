use anyhow::{Context, Result};
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use libbpf_rs::RingBufferBuilder;
use rusqlite::Connection;
use std::env;
use std::os::fd::{AsFd, AsRawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, Level};
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

fn int_to_ip(ip: u32) -> String {
    let bytes = ip.to_be_bytes(); // IP in eBPF is usually network byte order, but let's check
    // Actually, xdp_filter.bpf.c uses ip->saddr which is network byte order.
    // So let's format it. Wait, xdp_filter.bpf.c gets ip->saddr and directly assigns it to event->src_ip.
    // network byte order:
    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
}

fn handle_event(data: &[u8], db_conn: &Connection) -> i32 {
    if data.len() != std::mem::size_of::<AttackEvent>() {
        error!("Invalid event size: {} (expected {})", data.len(), std::mem::size_of::<AttackEvent>());
        return 0;
    }

    // Safety: we checked the length
    let event: AttackEvent = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const AttackEvent) };

    let src_ip = int_to_ip(event.src_ip);
    let dst_ip = int_to_ip(event.dst_ip);
    
    let attack_name = match event.attack_type {
        1 => "SYN Flood",
        2 => "Port Scan",
        3 => "ICMP Flood",
        _ => "Unknown Attack",
    };

    let severity = match event.attack_type {
        1 => "Critical",
        2 => "High",
        _ => "Medium",
    };

    info!(
        "Detected {} from {} to {}:{} (packets: {})",
        attack_name, src_ip, dst_ip, event.dst_port, event.packet_count
    );

    // Insert into DB
    let res = db_conn.execute(
        "INSERT INTO attacks (attack_type, source_ip, destination_ip, severity, packet_count, blocked)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            attack_name,
            src_ip,
            dst_ip,
            severity,
            event.packet_count as i64,
            true, // XDP dropped it, so it's blocked
        ),
    );

    if let Err(e) = res {
        error!("Failed to log attack to DB: {}", e);
    }

    0
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
    let mut skel = open_skel.load()?;

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

    // Open RingBuffer
    let mut builder = RingBufferBuilder::new();
    let maps = skel.maps();
    builder.add(maps.events(), move |data| handle_event(data, &db_conn))?;
    let ring_buffer = builder.build()?;

    info!("Listening for attack events. Press Ctrl+C to exit.");

    while running.load(Ordering::SeqCst) {
        ring_buffer.poll(Duration::from_millis(100))?;
    }

    info!("Exiting...");
    Ok(())
}
