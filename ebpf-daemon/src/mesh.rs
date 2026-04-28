//! Live WireGuard Mesh Controller
//!
//! Watches zerotrust.db for tunnels with status='connecting', provisions real
//! WireGuard kernel interfaces on the host using `wg` + `ip` CLI tools, and
//! keeps stats (bytes_sent, bytes_received, last_handshake) in sync with the DB.
//!
//! Architecture (single-host loopback mesh):
//!   For each tunnel between ServiceA and ServiceB on the same machine:
//!     - wg<2n>   = ServiceA endpoint  (listen port WG_BASE_PORT + 2n)
//!     - wg<2n+1> = ServiceB endpoint  (listen port WG_BASE_PORT + 2n + 1)
//!   Both endpoints are peered via 127.0.0.1, forming a real encrypted channel.

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// First port allocated to WireGuard interfaces (subsequent ones increment by 1)
const WG_BASE_PORT: u16 = 51820;

/// 10.128.x.x base for virtual IPs
const VIP_BASE: [u8; 2] = [10, 128];

/// State for one fully-provisioned tunnel (both interface halves managed here)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ManagedTunnel {
    pub tunnel_id: String,
    pub iface_a: String,
    pub iface_b: String,
    pub port_a: u16,
    pub port_b: u16,
    pub vip_a: String,
    pub vip_b: String,
}

// ── low-level helpers ─────────────────────────────────────────────────────────

/// Generate a real Curve25519 keypair using the system `wg` binary.
/// Returns (private_key_b64, public_key_b64).
fn generate_wg_keypair() -> Result<(String, String)> {
    let priv_out = Command::new("wg")
        .arg("genkey")
        .output()
        .context("Failed to run 'wg genkey' — is wireguard-tools installed?")?;

    if !priv_out.status.success() {
        anyhow::bail!("wg genkey failed: {}", String::from_utf8_lossy(&priv_out.stderr));
    }

    let private_key = String::from_utf8(priv_out.stdout)
        .context("wg genkey output is not valid UTF-8")?
        .trim()
        .to_string();

    let mut pubkey_child = Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn 'wg pubkey'")?;

    if let Some(mut stdin) = pubkey_child.stdin.take() {
        stdin
            .write_all(private_key.as_bytes())
            .context("Failed to write private key to wg pubkey stdin")?;
    }

    let pub_out = pubkey_child
        .wait_with_output()
        .context("wg pubkey process failed")?;

    if !pub_out.status.success() {
        anyhow::bail!("wg pubkey failed: {}", String::from_utf8_lossy(&pub_out.stderr));
    }

    let public_key = String::from_utf8(pub_out.stdout)
        .context("wg pubkey output is not valid UTF-8")?
        .trim()
        .to_string();

    Ok((private_key, public_key))
}

/// Write a private key to a mode-600 temp file.
fn write_key_file(key: &str, label: &str) -> Result<PathBuf> {
    let path = PathBuf::from(format!("/tmp/zt_{}.key", label));
    fs::write(&path, key).with_context(|| format!("Failed to write key file {}", path.display()))?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .context("Failed to chmod 600 key file")?;
    Ok(path)
}

/// Run a command and return stdout, or bail with a clear message on failure.
fn cmd(args: &[&str]) -> Result<String> {
    let out = Command::new(args[0])
        .args(&args[1..])
        .output()
        .with_context(|| format!("Failed to execute: {}", args.join(" ")))?;

    if !out.status.success() {
        anyhow::bail!(
            "Command `{}` exited {}: {}",
            args.join(" "),
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Return true if a network interface with `name` exists on the host.
fn iface_exists(name: &str) -> bool {
    Command::new("ip")
        .args(["link", "show", name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Bring an interface down and delete it if it exists.
fn teardown_iface(name: &str) {
    if iface_exists(name) {
        let _ = cmd(&["ip", "link", "set", name, "down"]);
        let _ = cmd(&["ip", "link", "delete", name]);
        info!("Removed WireGuard interface {}", name);
    }
}

/// Calculate a pair of /30 virtual IPs for tunnel index `idx`.
///
/// Layout inside 10.128.0.0/16:
///   idx=0 → 10.128.0.1 / 10.128.0.2  (subnet 10.128.0.0/30)
///   idx=1 → 10.128.0.5 / 10.128.0.6  (subnet 10.128.0.4/30)
fn virtual_ips(idx: u32) -> (String, String) {
    let offset = idx * 4; // each /30 occupies 4 addresses
    let b3 = (offset / 256) as u8;
    let b4 = (offset % 256) as u8;
    let vip_a = format!("{}.{}.{}.{}", VIP_BASE[0], VIP_BASE[1], b3, b4 + 1);
    let vip_b = format!("{}.{}.{}.{}", VIP_BASE[0], VIP_BASE[1], b3, b4 + 2);
    (vip_a, vip_b)
}

/// Parse `wg show <iface> dump` and return (bytes_tx, bytes_rx, last_handshake_rfc3339).
///
/// `wg show dump` columns per peer line (tab-separated):
///   pubkey  preshared_key  endpoint  allowed_ips  latest-handshake  transfer-rx  transfer-tx  persistent-keepalive
fn parse_wg_dump(iface: &str) -> (u64, u64, Option<String>) {
    let out = match Command::new("wg").args(["show", iface, "dump"]).output() {
        Ok(o) => o,
        Err(_) => return (0, 0, None),
    };

    if !out.status.success() {
        return (0, 0, None);
    }

    let text = String::from_utf8_lossy(&out.stdout);
    let mut total_tx: u64 = 0;
    let mut total_rx: u64 = 0;
    let mut latest_handshake: Option<String> = None;

    for line in text.lines().skip(1) {
        // first line is the interface row — skip it
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() >= 8 {
            let epoch: u64 = cols[4].parse().unwrap_or(0);
            let rx: u64 = cols[5].parse().unwrap_or(0);
            let tx: u64 = cols[6].parse().unwrap_or(0);
            total_rx += rx;
            total_tx += tx;
            if epoch > 0 {
                if let Some(dt) = chrono::DateTime::from_timestamp(epoch as i64, 0) {
                    latest_handshake = Some(dt.to_rfc3339());
                }
            }
        }
    }

    (total_tx, total_rx, latest_handshake)
}

// ── core provisioning ─────────────────────────────────────────────────────────

/// Create two real WireGuard kernel interfaces and peer them together
/// over the loopback interface (127.0.0.1).
///
/// Returns (pubkey_a, pubkey_b) on success.
fn provision_tunnel(
    tunnel_id: &str,
    iface_a: &str,
    iface_b: &str,
    port_a: u16,
    port_b: u16,
    vip_a: &str,
    vip_b: &str,
) -> Result<(String, String)> {
    info!(
        "Provisioning live tunnel {} — {} (port={}, ip={}) <-> {} (port={}, ip={})",
        &tunnel_id[..8], iface_a, port_a, vip_a, iface_b, port_b, vip_b
    );

    // Defensive cleanup of stale interfaces
    teardown_iface(iface_a);
    teardown_iface(iface_b);

    // 1. Generate real Curve25519 key pairs
    let (priv_a, pub_a) = generate_wg_keypair().context("Keypair gen for side A failed")?;
    let (priv_b, pub_b) = generate_wg_keypair().context("Keypair gen for side B failed")?;
    info!("  pubkey_a={}…  pubkey_b={}…", &pub_a[..12], &pub_b[..12]);

    // 2. Write private keys to secure temp files
    let short = &tunnel_id[..8];
    let key_a = write_key_file(&priv_a, &format!("{}_a", short))?;
    let key_b = write_key_file(&priv_b, &format!("{}_b", short))?;

    // 3. Create kernel WireGuard interfaces
    cmd(&["ip", "link", "add", "dev", iface_a, "type", "wireguard"])
        .with_context(|| format!("Failed to create {}", iface_a))?;
    cmd(&["ip", "link", "add", "dev", iface_b, "type", "wireguard"])
        .with_context(|| format!("Failed to create {}", iface_b))?;

    // 4. Load private keys + listen ports
    cmd(&[
        "wg", "set", iface_a,
        "private-key", key_a.to_str().unwrap(),
        "listen-port", &port_a.to_string(),
    ])
    .with_context(|| format!("wg set private-key on {} failed", iface_a))?;

    cmd(&[
        "wg", "set", iface_b,
        "private-key", key_b.to_str().unwrap(),
        "listen-port", &port_b.to_string(),
    ])
    .with_context(|| format!("wg set private-key on {} failed", iface_b))?;

    // 5. Assign virtual IPs using /30 point-to-point subnets
    cmd(&["ip", "addr", "add", &format!("{}/30", vip_a), "dev", iface_a])
        .with_context(|| format!("Failed to assign {} to {}", vip_a, iface_a))?;
    cmd(&["ip", "addr", "add", &format!("{}/30", vip_b), "dev", iface_b])
        .with_context(|| format!("Failed to assign {} to {}", vip_b, iface_b))?;

    // 6. Cross-peer the two interfaces via loopback
    cmd(&[
        "wg", "set", iface_a,
        "peer", &pub_b,
        "allowed-ips", &format!("{}/32", vip_b),
        "endpoint", &format!("127.0.0.1:{}", port_b),
        "persistent-keepalive", "25",
    ])
    .context("Failed to add peer B → interface A")?;

    cmd(&[
        "wg", "set", iface_b,
        "peer", &pub_a,
        "allowed-ips", &format!("{}/32", vip_a),
        "endpoint", &format!("127.0.0.1:{}", port_a),
        "persistent-keepalive", "25",
    ])
    .context("Failed to add peer A → interface B")?;

    // 7. Bring both interfaces up
    cmd(&["ip", "link", "set", iface_a, "up"])
        .with_context(|| format!("Failed to bring up {}", iface_a))?;
    cmd(&["ip", "link", "set", iface_b, "up"])
        .with_context(|| format!("Failed to bring up {}", iface_b))?;

    // 8. Remove temp key files — the key is now loaded into the kernel
    let _ = fs::remove_file(&key_a);
    let _ = fs::remove_file(&key_b);

    info!("  ✓ {} ↔ {} are UP and ACTIVE", iface_a, iface_b);
    Ok((pub_a, pub_b))
}

// ── controller ────────────────────────────────────────────────────────────────

pub struct WireGuardMeshController {
    db_path: PathBuf,
    running: Arc<AtomicBool>,
    managed: Vec<ManagedTunnel>,
}

impl WireGuardMeshController {
    pub fn new(db_path: PathBuf, running: Arc<AtomicBool>) -> Self {
        Self {
            db_path,
            running,
            managed: Vec::new(),
        }
    }

    /// Entry point — call from a dedicated OS thread.
    pub fn run(&mut self) {
        info!("WireGuard Mesh Controller started. Poll interval = 5 s.");
        let mut tick: u64 = 0;

        while self.running.load(Ordering::SeqCst) {
            if let Err(e) = self.provision_pending() {
                error!("Provisioning error: {}", e);
            }

            // sync real kernel stats to DB every 15 s
            if tick % 3 == 0 {
                self.sync_stats();
            }

            // health check every 60 s
            if tick % 12 == 0 && tick > 0 {
                self.health_check();
            }

            tick += 1;
            std::thread::sleep(Duration::from_secs(5));
        }

        self.teardown_all();
    }

    fn open_db(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
            .with_context(|| format!("Cannot open DB at {}", self.db_path.display()))
    }

    /// Query DB for tunnels in 'connecting' state and provision each one.
    fn provision_pending(&mut self) -> Result<()> {
        let db = self.open_db()?;

        let mut stmt = db.prepare(
            "SELECT id FROM tunnels WHERE status = 'connecting' ORDER BY created_at ASC",
        )?;

        let pending: Vec<String> = stmt
            .query_map([], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        drop(stmt);

        for tunnel_id in pending {
            if self.managed.iter().any(|t| t.tunnel_id == tunnel_id) {
                continue; // already managed
            }

            let idx = self.managed.len() as u32;
            let iface_a = format!("wg{}", idx * 2);
            let iface_b = format!("wg{}", idx * 2 + 1);
            let port_a = WG_BASE_PORT + (idx * 2) as u16;
            let port_b = WG_BASE_PORT + (idx * 2 + 1) as u16;
            let (vip_a, vip_b) = virtual_ips(idx);

            match provision_tunnel(
                &tunnel_id, &iface_a, &iface_b, port_a, port_b, &vip_a, &vip_b,
            ) {
                Ok((pub_a, pub_b)) => {
                    let now = chrono::Utc::now().to_rfc3339();
                    let combined_pubkey = format!("{}|{}", pub_a, pub_b);
                    let combined_vip = format!("{}/{}", vip_a, vip_b);

                    if let Err(e) = db.execute(
                        "UPDATE tunnels SET status='active', public_key=?1, virtual_ip=?2, last_handshake=?3 WHERE id=?4",
                        rusqlite::params![combined_pubkey, combined_vip, now, tunnel_id],
                    ) {
                        error!("DB update failed for {}: {}", &tunnel_id[..8], e);
                        teardown_iface(&iface_a);
                        teardown_iface(&iface_b);
                        continue;
                    }

                    // Write to audit log
                    let _ = db.execute(
                        "INSERT INTO audit_logs (event_type, action, subject, details, success) VALUES (?1,?2,?3,?4,?5)",
                        rusqlite::params![
                            "mesh",
                            "tunnel_provisioned",
                            tunnel_id,
                            format!(
                                "{{\"iface_a\":\"{iface_a}\",\"iface_b\":\"{iface_b}\",\
                                 \"vip_a\":\"{vip_a}\",\"vip_b\":\"{vip_b}\",\
                                 \"port_a\":{port_a},\"port_b\":{port_b}}}"
                            ),
                            true,
                        ],
                    );

                    info!("Tunnel {} committed to DB — status=active.", &tunnel_id[..8]);

                    self.managed.push(ManagedTunnel {
                        tunnel_id,
                        iface_a,
                        iface_b,
                        port_a,
                        port_b,
                        vip_a,
                        vip_b,
                    });
                }

                Err(e) => {
                    error!("Provision failed for tunnel {}: {}", &tunnel_id[..8], e);
                    let _ = db.execute(
                        "UPDATE tunnels SET status='error' WHERE id=?1",
                        rusqlite::params![tunnel_id],
                    );
                }
            }
        }

        Ok(())
    }

    /// Read live stats from the kernel via `wg show dump` and write to DB.
    fn sync_stats(&self) {
        let db = match self.open_db() {
            Ok(d) => d,
            Err(e) => { error!("sync_stats DB open: {}", e); return; }
        };

        for t in &self.managed {
            let (tx_a, rx_a, hs_a) = parse_wg_dump(&t.iface_a);
            let (tx_b, rx_b, hs_b) = parse_wg_dump(&t.iface_b);
            let total_tx = tx_a + tx_b;
            let total_rx = rx_a + rx_b;
            let handshake = hs_a.or(hs_b);

            if let Err(e) = db.execute(
                "UPDATE tunnels SET bytes_sent=?1, bytes_received=?2, last_handshake=?3 WHERE id=?4",
                rusqlite::params![total_tx as i64, total_rx as i64, handshake, t.tunnel_id],
            ) {
                error!("stat sync failed for {}: {}", &t.tunnel_id[..8], e);
            } else {
                info!("Stats synced {}: tx={} bytes, rx={} bytes", &t.tunnel_id[..8], total_tx, total_rx);
            }
        }
    }

    /// Verify both kernel interfaces are still present; mark degraded ones as 'error'.
    fn health_check(&self) {
        let db = match self.open_db() {
            Ok(d) => d,
            Err(e) => { error!("health_check DB open: {}", e); return; }
        };

        for t in &self.managed {
            let a_ok = iface_exists(&t.iface_a);
            let b_ok = iface_exists(&t.iface_b);

            if a_ok && b_ok {
                info!("Health OK: {} ({} ↔ {})", &t.tunnel_id[..8], t.iface_a, t.iface_b);
            } else {
                warn!(
                    "Tunnel {} degraded — {} {} / {} {}",
                    &t.tunnel_id[..8],
                    t.iface_a, if a_ok { "UP" } else { "DOWN" },
                    t.iface_b, if b_ok { "UP" } else { "DOWN" },
                );
                let _ = db.execute(
                    "UPDATE tunnels SET status='error' WHERE id=?1",
                    rusqlite::params![t.tunnel_id],
                );
            }
        }
    }

    /// Graceful shutdown: remove all kernel interfaces and mark tunnels inactive.
    fn teardown_all(&mut self) {
        info!("Shutting down mesh controller — removing {} tunnel(s).", self.managed.len());
        let db = self.open_db().ok();

        for t in &self.managed {
            teardown_iface(&t.iface_a);
            teardown_iface(&t.iface_b);

            if let Some(ref db) = db {
                let _ = db.execute(
                    "UPDATE tunnels SET status='inactive', bytes_sent=0, bytes_received=0 WHERE id=?1",
                    rusqlite::params![t.tunnel_id],
                );
            }
        }

        self.managed.clear();
        info!("All WireGuard interfaces removed cleanly.");
    }
}
