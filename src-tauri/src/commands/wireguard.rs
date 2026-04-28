//! WireGuard tunnel management Tauri commands

use serde::{Deserialize, Serialize};
use tauri::command;
use tracing::info;
use uuid::Uuid;

use crate::get_app_state;
use zerotrust_mesh_lib::wireguard::WgKeyPair;

/// Tunnel response
#[derive(Debug, Serialize)]
pub struct TunnelResponse {
    pub id: String,
    pub service_a_id: String,
    pub service_b_id: String,
    pub interface_name: String,
    pub public_key: String,
    pub virtual_ip: String,
    pub peer_endpoint: Option<String>,
    pub status: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_handshake: Option<String>,
}

/// Create tunnel request
#[derive(Debug, Deserialize)]
pub struct CreateTunnelRequest {
    pub service_a_id: String,
    pub service_b_id: String,
    pub endpoint: Option<String>,
}

/// Create a new WireGuard tunnel (persisted record for the desktop app)
#[command]
pub async fn create_tunnel(request: CreateTunnelRequest) -> Result<TunnelResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;

    // Check if a tunnel already exists between these services (UNIQUE constraint check)
    let existing_tunnel: Vec<String> = state.db.query_map(
        "SELECT id FROM tunnels WHERE (service_a_id = ?1 AND service_b_id = ?2) OR (service_a_id = ?2 AND service_b_id = ?1)",
        &[&request.service_a_id, &request.service_b_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if !existing_tunnel.is_empty() {
        return Err("A secure tunnel already exists between these two services.".to_string());
    }

    // Get the highest current index for interface naming
    let max_idx: i64 = state.db.query_map(
        "SELECT MAX(CAST(SUBSTR(interface_name, 3) AS INTEGER)) FROM tunnels",
        &[],
        |row| Ok(row.get::<_, Option<i64>>(0)?.unwrap_or(0))
    ).map_err(|e| e.to_string())?.first().copied().unwrap_or(0);

    let key_pair = WgKeyPair::generate().map_err(|e| e.to_string())?;
    let tunnel_id = Uuid::new_v4().to_string();
    let interface_name = format!("wg{}", max_idx + 1);
    let virtual_ip = format!("10.128.0.{}", max_idx + 10);
    let now = chrono::Utc::now().to_rfc3339();

    state
        .db
        .execute(
            "INSERT INTO tunnels (
                id, service_a_id, service_b_id, interface_name, private_key_encrypted,
                public_key, virtual_ip, peer_endpoint, status, bytes_sent, bytes_received, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            &[
                &tunnel_id,
                &request.service_a_id,
                &request.service_b_id,
                &interface_name,
                &key_pair.private_key.into_bytes(),
                &key_pair.public_key,
                &virtual_ip,
                &request.endpoint,
                &"connecting".to_string(),
                &0i64,
                &0i64,
                &now,
            ],
        )
        .map_err(|e| e.to_string())?;

    info!(
        "Created tunnel {} between {} and {}",
        tunnel_id, request.service_a_id, request.service_b_id
    );

    Ok(TunnelResponse {
        id: tunnel_id,
        service_a_id: request.service_a_id,
        service_b_id: request.service_b_id,
        interface_name,
        public_key: key_pair.public_key,
        virtual_ip,
        peer_endpoint: request.endpoint,
        status: "connecting".to_string(),
        bytes_sent: 0,
        bytes_received: 0,
        last_handshake: None,
    })
}

/// Destroy a WireGuard tunnel (C2.2)
#[command]
pub async fn destroy_tunnel(tunnel_id: String) -> Result<(), String> {
    let state = get_app_state().ok_or("Application not initialized")?;

    state
        .db
        .execute("DELETE FROM tunnels WHERE id = ?1", &[&tunnel_id])
        .map_err(|e| e.to_string())?;

    info!("Destroyed tunnel: {}", tunnel_id);
    Ok(())
}

/// List all tunnels
#[command]
pub async fn list_tunnels() -> Result<Vec<TunnelResponse>, String> {
    let state = get_app_state().ok_or("Application not initialized")?;

    state
        .db
        .query_map(
            "SELECT id, service_a_id, service_b_id, interface_name, public_key, virtual_ip,
                    peer_endpoint, status, bytes_sent, bytes_received, last_handshake
             FROM tunnels
             ORDER BY created_at DESC",
            &[],
            |row| {
                Ok(TunnelResponse {
                    id: row.get(0)?,
                    service_a_id: row.get(1)?,
                    service_b_id: row.get(2)?,
                    interface_name: row.get(3)?,
                    public_key: row.get(4)?,
                    virtual_ip: row.get(5)?,
                    peer_endpoint: row.get(6)?,
                    status: row.get(7)?,
                    bytes_sent: row.get::<_, i64>(8)? as u64,
                    bytes_received: row.get::<_, i64>(9)? as u64,
                    last_handshake: row.get(10)?,
                })
            },
        )
        .map_err(|e| e.to_string())
}

/// Get tunnel status
#[command]
pub async fn get_tunnel_status(tunnel_id: String) -> Result<TunnelResponse, String> {
    let tunnels = list_tunnels().await?;
    tunnels
        .into_iter()
        .find(|tunnel| tunnel.id == tunnel_id)
        .ok_or_else(|| format!("Tunnel not found: {}", tunnel_id))
}
