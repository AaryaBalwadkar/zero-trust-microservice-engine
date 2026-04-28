//! Identity management Tauri commands

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;
use tracing::{info, error};

use crate::{get_app_state};

/// Service registration request
#[derive(Debug, Deserialize)]
pub struct RegisterServiceRequest {
    pub name: String,
    pub port: u16,
    pub description: Option<String>,
    pub binary_path: Option<String>,
}

/// Service response
#[derive(Debug, Serialize)]
pub struct ServiceResponse {
    pub id: String,
    pub spiffe_id: String,
    pub name: String,
    pub description: Option<String>,
    pub binary_path: Option<String>,
    pub port: u16,
    pub status: String,
    pub trust_score: f64,
}

#[command]
pub async fn register_service(request: RegisterServiceRequest) -> Result<ServiceResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    
    // --- Duplicate check BEFORE acquiring the identity_provider lock ---
    // Only check port conflicts. Only check binary_path when one is actually provided.
    let has_binary_path = request.binary_path.as_ref().map_or(false, |p| !p.trim().is_empty());
    
    let duplicates: Vec<String> = if has_binary_path {
        let bp = request.binary_path.as_deref().unwrap();
        state.db.query_map(
            "SELECT name FROM services WHERE status != 'inactive' AND (port = ?1 OR (binary_path IS NOT NULL AND binary_path != '' AND binary_path = ?2))",
            &[&(request.port as i32), &bp],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?
    } else {
        state.db.query_map(
            "SELECT name FROM services WHERE status != 'inactive' AND port = ?1",
            &[&(request.port as i32)],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?
    };

    if !duplicates.is_empty() {
        let detail = if has_binary_path {
            format!(
                "Conflict: Service '{}' already uses port {} or binary path '{}'.",
                duplicates[0], request.port, request.binary_path.as_deref().unwrap_or("")
            )
        } else {
            format!(
                "Conflict: Service '{}' already uses port {}.",
                duplicates[0], request.port
            )
        };
        return Err(detail);
    }

    // Now acquire the provider lock for certificate generation
    let provider = state.identity_provider.write();
    let binary_path = request.binary_path.as_ref().map(std::path::Path::new);
    
    let (service, _cert) = provider
        .register_service(
            &request.name,
            request.port,
            request.description.as_deref(),
            binary_path,
        )
        .map_err(|e| e.to_string())?;

    let binary_path_for_db = service
        .binary_path
        .as_ref()
        .map(pathbuf_to_string);
    
    // Store in database
    state.db.execute(
        "INSERT INTO services (id, spiffe_id, name, description, port, binary_path, binary_hash, status, trust_score)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        &[
            &service.id,
            &service.spiffe_id.to_uri(),
            &service.name,
            &service.description,
            &(service.port as i32),
            &binary_path_for_db,
            &service.binary_hash,
            &service.status.to_string(),
            &service.trust_score,
        ],
    ).map_err(|e| e.to_string())?;
    
    // Log audit event (G1.2)
    state.db.execute(
        "INSERT INTO audit_logs (event_type, action, subject, details, success)
         VALUES ('identity', 'register_service', ?1, ?2, 1)",
        &[&service.id, &format!("Registered service: {}", service.name)],
    ).ok();
    
    info!("Registered service: {} ({})", service.name, service.id);
    
    Ok(ServiceResponse {
        id: service.id,
        spiffe_id: service.spiffe_id.to_uri(),
        name: service.name,
        description: service.description,
        binary_path: binary_path_for_db,
        port: service.port,
        status: service.status.to_string(),
        trust_score: service.trust_score,
    })
}

fn pathbuf_to_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

/// Deregister a service (A2.6)
#[command]
pub async fn deregister_service(service_id: String) -> Result<(), String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    
    state.db.execute(
        "UPDATE services SET status = 'inactive' WHERE id = ?1",
        &[&service_id],
    ).map_err(|e| e.to_string())?;
    
    // Log audit event
    state.db.execute(
        "INSERT INTO audit_logs (event_type, action, subject, success)
         VALUES ('identity', 'deregister_service', ?1, 1)",
        &[&service_id],
    ).ok();
    
    info!("Deregistered service: {}", service_id);
    Ok(())
}

/// List all services
#[command]
pub async fn list_services() -> Result<Vec<ServiceResponse>, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    
    let services: Vec<ServiceResponse> = state.db.query_map(
        "SELECT id, spiffe_id, name, description, binary_path, port, status, trust_score 
         FROM services WHERE status != 'inactive' ORDER BY name",
        &[],
        |row| {
            Ok(ServiceResponse {
                id: row.get(0)?,
                spiffe_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                binary_path: row.get(4)?,
                port: row.get::<_, i32>(5)? as u16,
                status: row.get(6)?,
                trust_score: row.get(7)?,
            })
        },
    ).map_err(|e| e.to_string())?;
    
    Ok(services)
}

/// Get a single service
#[command]
pub async fn get_service(service_id: String) -> Result<ServiceResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    
    let services: Vec<ServiceResponse> = state.db.query_map(
        "SELECT id, spiffe_id, name, description, binary_path, port, status, trust_score 
         FROM services WHERE id = ?1",
        &[&service_id],
        |row| {
            Ok(ServiceResponse {
                id: row.get(0)?,
                spiffe_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                binary_path: row.get(4)?,
                port: row.get::<_, i32>(5)? as u16,
                status: row.get(6)?,
                trust_score: row.get(7)?,
            })
        },
    ).map_err(|e| e.to_string())?;
    
    services.into_iter().next()
        .ok_or_else(|| format!("Service not found: {}", service_id))
}

/// Issue JWT-SVID for a service (A1.2)
#[command]
pub async fn issue_jwt_svid(service_id: String, audience: Vec<String>) -> Result<String, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    
    // Get service from database
    let services: Vec<(String, String, f64)> = state.db.query_map(
        "SELECT spiffe_id, name, trust_score FROM services WHERE id = ?1",
        &[&service_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    ).map_err(|e| e.to_string())?;
    
    let (spiffe_id_str, name, trust_score) = services.into_iter().next()
        .ok_or_else(|| format!("Service not found: {}", service_id))?;
    
    let spiffe_id = zerotrust_mesh_lib::identity::SpiffeId::from_uri(&spiffe_id_str)
        .map_err(|e| e.to_string())?;
    
    let service = zerotrust_mesh_lib::identity::Service {
        id: service_id.clone(),
        spiffe_id,
        name,
        description: None,
        port: 0,
        binary_path: None,
        binary_hash: None,
        user: None,
        pid: None,
        status: zerotrust_mesh_lib::identity::ServiceStatus::Active,
        trust_score,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let provider = state.identity_provider.read();
    let token = provider.issue_jwt_svid(&service, audience)
        .map_err(|e| e.to_string())?;
    
    // Log audit event (A1.8)
    state.db.execute(
        "INSERT INTO audit_logs (event_type, action, subject, success)
         VALUES ('identity', 'issue_jwt', ?1, 1)",
        &[&service_id],
    ).ok();
    
    Ok(token)
}

/// Verify JWT-SVID (A3.2)
#[command]
pub async fn verify_svid(token: String) -> Result<bool, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let provider = state.identity_provider.read();
    
    match provider.verify_jwt_svid(&token) {
        Ok(_claims) => Ok(true),
        Err(e) => {
            error!("SVID verification failed: {}", e);
            Ok(false)
        }
    }
}
