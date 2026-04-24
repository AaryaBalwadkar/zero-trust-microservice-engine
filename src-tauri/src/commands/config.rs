//! Configuration management Tauri commands

use serde::{Deserialize, Serialize};
use tauri::command;
use tracing::info;

use crate::get_app_state;

/// Configuration response
#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub general: GeneralConfigResponse,
    pub identity: IdentityConfigResponse,
    pub policy: PolicyConfigResponse,
    pub wireguard: WireGuardConfigResponse,
    pub ebpf: EbpfConfigResponse,
    pub attestation: AttestationConfigResponse,
    pub storage: StorageConfigResponse,
    pub logging: LoggingConfigResponse,
}

#[derive(Debug, Serialize)]
pub struct GeneralConfigResponse {
    pub autostart: bool,
    pub theme: String,
    pub notifications_enabled: bool,
    pub inactivity_lock_minutes: u32,
}

#[derive(Debug, Serialize)]
pub struct IdentityConfigResponse {
    pub trust_domain: String,
    pub jwt_expiration_seconds: u32,
}

#[derive(Debug, Serialize)]
pub struct PolicyConfigResponse {
    pub cache_ttl_seconds: u32,
    pub default_action: String,
}

#[derive(Debug, Serialize)]
pub struct WireGuardConfigResponse {
    pub listen_port: u16,
    pub virtual_subnet: String,
    pub keepalive_seconds: u32,
    pub mtu: u16,
}

#[derive(Debug, Serialize)]
pub struct EbpfConfigResponse {
    pub enabled: bool,
    pub interface: String,
    pub syn_flood_threshold: u32,
    pub port_scan_threshold: u32,
    pub http_flood_threshold: u32,
}

#[derive(Debug, Serialize)]
pub struct AttestationConfigResponse {
    pub tpm_enabled: bool,
    pub recalculation_interval_seconds: u32,
    pub full_access_threshold: f32,
    pub limited_access_threshold: f32,
    pub isolation_threshold: f32,
}

#[derive(Debug, Serialize)]
pub struct StorageConfigResponse {
    pub database_path: String,
    pub log_retention_days: u32,
    pub max_alerts: u32,
}

#[derive(Debug, Serialize)]
pub struct LoggingConfigResponse {
    pub level: String,
    pub file_enabled: bool,
    pub file_path: String,
    pub max_file_size_mb: u32,
}

#[derive(Debug, Serialize)]
pub struct DatabaseStatsResponse {
    pub service_count: u64,
    pub policy_count: u64,
    pub tunnel_count: u64,
    pub attack_count_24h: u64,
    pub unacknowledged_alerts: u64,
    pub blacklist_count: u64,
}

/// Get current configuration
#[command]
pub async fn get_config() -> Result<ConfigResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let config = state.config.read();
    
    Ok(ConfigResponse {
        general: GeneralConfigResponse {
            autostart: config.general.autostart,
            theme: config.general.theme.clone(),
            notifications_enabled: config.general.notifications_enabled,
            inactivity_lock_minutes: config.general.inactivity_lock_minutes,
        },
        identity: IdentityConfigResponse {
            trust_domain: config.identity.trust_domain.clone(),
            jwt_expiration_seconds: config.identity.jwt_expiration_seconds,
        },
        policy: PolicyConfigResponse {
            cache_ttl_seconds: config.policy.cache_ttl_seconds,
            default_action: config.policy.default_action.clone(),
        },
        wireguard: WireGuardConfigResponse {
            listen_port: config.wireguard.listen_port,
            virtual_subnet: config.wireguard.virtual_subnet.clone(),
            keepalive_seconds: config.wireguard.keepalive_seconds,
            mtu: config.wireguard.mtu,
        },
        ebpf: EbpfConfigResponse {
            enabled: config.ebpf.enabled,
            interface: config.ebpf.interface.clone(),
            syn_flood_threshold: config.ebpf.syn_flood_threshold,
            port_scan_threshold: config.ebpf.port_scan_threshold,
            http_flood_threshold: config.ebpf.http_flood_threshold,
        },
        attestation: AttestationConfigResponse {
            tpm_enabled: config.attestation.tpm_enabled,
            recalculation_interval_seconds: config.attestation.recalculation_interval_seconds,
            full_access_threshold: config.attestation.full_access_threshold,
            limited_access_threshold: config.attestation.limited_access_threshold,
            isolation_threshold: config.attestation.isolation_threshold,
        },
        storage: StorageConfigResponse {
            database_path: config.storage.database_path.display().to_string(),
            log_retention_days: config.storage.log_retention_days,
            max_alerts: config.storage.max_alerts,
        },
        logging: LoggingConfigResponse {
            level: config.logging.level.clone(),
            file_enabled: config.logging.file_enabled,
            file_path: config.logging.file_path.display().to_string(),
            max_file_size_mb: config.logging.max_file_size_mb,
        },
    })
}

/// Get current database statistics
#[command]
pub async fn get_database_stats() -> Result<DatabaseStatsResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let stats = state.db.get_stats().map_err(|e| e.to_string())?;

    Ok(DatabaseStatsResponse {
        service_count: stats.service_count,
        policy_count: stats.policy_count,
        tunnel_count: stats.tunnel_count,
        attack_count_24h: stats.attack_count_24h,
        unacknowledged_alerts: stats.unacknowledged_alerts,
        blacklist_count: stats.blacklist_count,
    })
}

/// Update configuration request
#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub section: String,
    pub key: String,
    pub value: serde_json::Value,
}

/// Update configuration
#[command]
pub async fn update_config(request: UpdateConfigRequest) -> Result<(), String> {
    let state = get_app_state().ok_or("Application not initialized")?;
    let mut config = state.config.write();
    
    // Log configuration change (G1.3)
    let old_value = format!("{}::{}", request.section, request.key);
    let new_value = request.value.to_string();
    
    state.db.execute(
        "INSERT INTO config_changes (config_key, old_value, new_value, created_at)
         VALUES (?1, ?2, ?3, ?4)",
        &[&old_value, &"(previous)".to_string(), &new_value, &chrono::Utc::now().to_rfc3339()],
    ).ok();
    
    // Apply update based on section and key
    match request.section.as_str() {
        "general" => {
            match request.key.as_str() {
                "theme" => {
                    config.general.theme = request.value.as_str()
                        .ok_or("Invalid theme value")?.to_string();
                }
                "notifications_enabled" => {
                    config.general.notifications_enabled = request.value.as_bool()
                        .ok_or("Invalid notifications_enabled value")?;
                }
                "autostart" => {
                    config.general.autostart = request.value.as_bool()
                        .ok_or("Invalid autostart value")?;
                }
                _ => return Err(format!("Unknown key: {}", request.key)),
            }
        }
        "ebpf" => {
            match request.key.as_str() {
                "enabled" => {
                    config.ebpf.enabled = request.value.as_bool()
                        .ok_or("Invalid enabled value")?;
                }
                "syn_flood_threshold" => {
                    config.ebpf.syn_flood_threshold = request.value.as_u64()
                        .ok_or("Invalid threshold value")? as u32;
                }
                _ => return Err(format!("Unknown key: {}", request.key)),
            }
        }
        "attestation" => {
            match request.key.as_str() {
                "tpm_enabled" => {
                    config.attestation.tpm_enabled = request.value.as_bool()
                        .ok_or("Invalid tpm_enabled value")?;
                }
                _ => return Err(format!("Unknown key: {}", request.key)),
            }
        }
        _ => return Err(format!("Unknown section: {}", request.section)),
    }
    
    info!("Updated config: {}.{} = {}", request.section, request.key, new_value);
    Ok(())
}
