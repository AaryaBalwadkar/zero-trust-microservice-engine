//! Attestation and trust scoring Tauri commands

use serde::Serialize;
use std::path::Path;
use tauri::command;
use tracing::info;

use crate::get_app_state;

/// Trust score response
#[derive(Debug, Serialize)]
pub struct TrustScoreResponse {
    pub service_id: String,
    pub score: f64,
    pub level: String,
    pub tpm_score: f64,
    pub process_score: f64,
    pub behavioral_score: f64,
    pub resource_score: f64,
    pub reason: Option<String>,
    pub calculated_at: String,
}

/// TPM status response
#[derive(Debug, Serialize)]
pub struct TpmStatusResponse {
    pub available: bool,
    pub version: Option<String>,
    pub manufacturer: Option<String>,
    pub last_check: String,
}

/// Binary measurement response
#[derive(Debug, Serialize)]
pub struct BinaryMeasurementResponse {
    pub path: String,
    pub sha256_hash: String,
    pub size_bytes: u64,
    pub measured_at: String,
}

#[derive(Debug, Serialize)]
pub struct ServiceScanResultResponse {
    pub service_id: String,
    pub service_name: String,
    pub binary_path: Option<String>,
    pub expected_sha256: Option<String>,
    pub measured_sha256: Option<String>,
    pub measured_at: Option<String>,
    pub trust_score: f64,
    pub trust_level: String,
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServiceScanSummaryResponse {
    pub started_at: String,
    pub completed_at: String,
    pub scanned: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub results: Vec<ServiceScanResultResponse>,
}

/// Get trust score for a service (E3.1)
#[command]
pub async fn get_trust_score(service_id: String) -> Result<TrustScoreResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let trust_manager = state.trust_manager.read();
    
    let score = trust_manager.calculate_trust_score(&service_id);
    
    Ok(TrustScoreResponse {
        service_id: score.service_id,
        score: score.score,
        level: format!("{}", score.level),
        tpm_score: score.components.tpm_score,
        process_score: score.components.process_score,
        behavioral_score: score.components.behavioral_score,
        resource_score: score.components.resource_score,
        reason: score.reason,
        calculated_at: score.calculated_at.to_rfc3339(),
    })
}

/// List all trust scores
#[command]
pub async fn list_trust_scores() -> Result<Vec<TrustScoreResponse>, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let trust_manager = state.trust_manager.read();
    
    let scores = trust_manager.get_all_trust_scores();
    
    Ok(scores.into_iter().map(|score| TrustScoreResponse {
        service_id: score.service_id,
        score: score.score,
        level: format!("{}", score.level),
        tpm_score: score.components.tpm_score,
        process_score: score.components.process_score,
        behavioral_score: score.components.behavioral_score,
        resource_score: score.components.resource_score,
        reason: score.reason,
        calculated_at: score.calculated_at.to_rfc3339(),
    }).collect())
}

/// Measure binary hash (E1.1)
#[command]
pub async fn measure_binary(path: String) -> Result<BinaryMeasurementResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let trust_manager = state.trust_manager.read();
    
    let measurement = trust_manager
        .measure_binary(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    info!("Measured binary {}: {}", path, measurement.sha256_hash);
    
    Ok(BinaryMeasurementResponse {
        path: measurement.path.to_string_lossy().to_string(),
        sha256_hash: measurement.sha256_hash,
        size_bytes: measurement.size_bytes,
        measured_at: measurement.measured_at.to_rfc3339(),
    })
}

/// Get TPM status (E2.1)
#[command]
pub async fn get_tpm_status() -> Result<TpmStatusResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let trust_manager = state.trust_manager.read();
    
    if let Some(status) = trust_manager.get_tpm_status() {
        Ok(TpmStatusResponse {
            available: status.available,
            version: status.version,
            manufacturer: status.manufacturer,
            last_check: status.last_check.to_rfc3339(),
        })
    } else {
        Ok(TpmStatusResponse {
            available: false,
            version: None,
            manufacturer: None,
            last_check: chrono::Utc::now().to_rfc3339(),
        })
    }
}

#[derive(Debug)]
struct ServiceScanRow {
    id: String,
    name: String,
    binary_path: Option<String>,
    binary_hash: Option<String>,
    pid: Option<i64>,
}

/// Scan all registered services with configured binaries and persist refreshed trust scores.
#[command]
pub async fn scan_registered_services() -> Result<ServiceScanSummaryResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let started_at = chrono::Utc::now();

    let services: Vec<ServiceScanRow> = state
        .db
        .query_map(
            "SELECT id, name, binary_path, binary_hash, pid
             FROM services
             WHERE status != 'inactive'
             ORDER BY name ASC",
            &[],
            |row| {
                Ok(ServiceScanRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    binary_path: row.get(2)?,
                    binary_hash: row.get(3)?,
                    pid: row.get(4)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    let mut passed = 0u64;
    let mut failed = 0u64;
    let mut skipped = 0u64;
    let mut results = Vec::with_capacity(services.len());

    for service in services {
        let binary_path = service.binary_path.clone();
        let path = binary_path.as_deref().map(Path::new);
        let mut status = "skipped".to_string();
        let mut measured_sha256 = None;
        let mut measured_at = None;
        let mut reason: Option<String> = None;

        let trust_score = {
            let trust_manager = state.trust_manager.write();

            if let (Some(path), Some(expected_hash)) = (path, service.binary_hash.as_deref()) {
                trust_manager.register_known_good(
                    path,
                    expected_hash,
                    Some("Registered service baseline"),
                );
            }

            if let Some(path) = path {
                match trust_manager.measure_binary(path) {
                    Ok(measurement) => {
                        measured_sha256 = Some(measurement.sha256_hash.clone());
                        measured_at = Some(measurement.measured_at.to_rfc3339());

                        status = if let Some(expected_hash) = service.binary_hash.as_deref() {
                            if measurement.sha256_hash == expected_hash {
                                "passed".to_string()
                            } else {
                                "failed".to_string()
                            }
                        } else {
                            "passed".to_string()
                        };
                    }
                    Err(error) => {
                        status = "failed".to_string();
                        reason = Some(error.to_string());
                    }
                }
            }

            trust_manager
                .update_service(&service.id, path, service.pid.map(|pid| pid as u32))
                .map_err(|e| e.to_string())?;
            trust_manager.calculate_trust_score(&service.id)
        };

        if reason.is_none() {
            reason = if binary_path.is_none() {
                Some("No binary path registered, so the service scan was skipped.".to_string())
            } else if service.binary_hash.is_none() && measured_sha256.is_some() {
                Some("Binary measured successfully, and this scan stored the first baseline hash.".to_string())
            } else {
                trust_score.reason.clone()
            };
        }

        match status.as_str() {
            "passed" => passed += 1,
            "failed" => failed += 1,
            _ => skipped += 1,
        }

        state
            .db
            .execute(
                "UPDATE services
                 SET binary_hash = COALESCE(binary_hash, ?2), trust_score = ?3, updated_at = ?4
                 WHERE id = ?1",
                &[
                    &service.id,
                    &measured_sha256,
                    &trust_score.score,
                    &trust_score.calculated_at.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;

        state
            .db
            .execute(
                "INSERT INTO trust_scores (
                    service_id, score, tpm_score, process_score, behavioral_score, resource_score, reason, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                &[
                    &service.id,
                    &trust_score.score,
                    &trust_score.components.tpm_score,
                    &trust_score.components.process_score,
                    &trust_score.components.behavioral_score,
                    &trust_score.components.resource_score,
                    &reason,
                    &trust_score.calculated_at.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;

        results.push(ServiceScanResultResponse {
            service_id: service.id.clone(),
            service_name: service.name.clone(),
            binary_path,
            expected_sha256: service.binary_hash.clone(),
            measured_sha256,
            measured_at,
            trust_score: trust_score.score,
            trust_level: trust_score.level.to_string(),
            status,
            reason,
        });
    }

    state
        .db
        .execute(
            "INSERT INTO audit_logs (event_type, action, details, success, created_at)
             VALUES ('attestation', 'scan_registered_services', ?1, 1, ?2)",
            &[
                &format!(
                    "scanned={}, passed={}, failed={}, skipped={}",
                    results.len(),
                    passed,
                    failed,
                    skipped
                ),
                &chrono::Utc::now().to_rfc3339(),
            ],
        )
        .ok();

    let completed_at = chrono::Utc::now();

    info!(
        "Completed registered service scan: {} services, {} passed, {} failed, {} skipped",
        results.len(),
        passed,
        failed,
        skipped
    );

    Ok(ServiceScanSummaryResponse {
        started_at: started_at.to_rfc3339(),
        completed_at: completed_at.to_rfc3339(),
        scanned: results.len() as u64,
        passed,
        failed,
        skipped,
        results,
    })
}
