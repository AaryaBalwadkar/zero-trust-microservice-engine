//! Development and demo-data commands

use chrono::{Duration, Utc};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::command;
use tracing::info;
use uuid::Uuid;

use crate::get_app_state;
use zerotrust_mesh_lib::policy::{
    DestinationField, Operator, Policy, PolicyAction, PolicyCondition, SourceField,
};

#[derive(Debug, Serialize)]
pub struct DemoDataResponse {
    pub services: u64,
    pub policies: u64,
    pub tunnels: u64,
    pub attacks: u64,
    pub alerts: u64,
    pub audit_logs: u64,
}

/// Populate the local database with representative demo data.
#[command]
pub async fn seed_demo_data() -> Result<DemoDataResponse, String> {
    let state = get_app_state().ok_or("Application not initialized")?;

    reset_demo_tables(&state)?;

    let services = {
        let provider = state.identity_provider.write();
        let sample_services = [
            ("API Gateway", 8080u16, "Public ingress and service routing"),
            ("Auth Service", 9000u16, "Issues and verifies workload credentials"),
            ("Payments Worker", 9100u16, "Processes internal payment jobs"),
        ];

        for (name, port, description) in sample_services {
            let demo_binary_path = resolve_demo_binary_path(name);
            let (service, _cert) = provider
                .register_service(name, port, Some(description), demo_binary_path.as_deref())
                .map_err(|e| e.to_string())?;

            let binary_path = service
                .binary_path
                .as_ref()
                .map(pathbuf_to_string);

            state
                .db
                .execute(
                    "INSERT INTO services (id, spiffe_id, name, description, port, binary_path, binary_hash, status, trust_score)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    &[
                        &service.id,
                        &service.spiffe_id.to_uri(),
                        &service.name,
                        &service.description,
                        &(service.port as i32),
                        &binary_path,
                        &service.binary_hash,
                        &service.status.to_string(),
                        &service.trust_score,
                    ],
                )
                .map_err(|e| e.to_string())?;
        }

        state
            .db
            .query_map(
                "SELECT id, name FROM services ORDER BY name ASC",
                &[],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .map_err(|e| e.to_string())?
    };

    let find_service_id = |service_name: &str| -> Result<String, String> {
        services
            .iter()
            .find(|(_, name)| name == service_name)
            .map(|(id, _)| id.clone())
            .ok_or_else(|| format!("Missing demo service: {service_name}"))
    };

    {
        let mut engine = state.policy_engine.write();

        let existing_ids: Vec<String> = engine
            .get_policies()
            .iter()
            .map(|policy| policy.id.clone())
            .collect();

        for policy_id in existing_ids {
            engine.remove_policy(&policy_id).map_err(|e| e.to_string())?;
        }

        let mut allow_policy = Policy::new("Allow API to Auth", 10, PolicyAction::Allow);
        allow_policy.description =
            Some("Allows the gateway to call the authentication service".to_string());
        allow_policy.conditions.push(PolicyCondition::And {
            conditions: vec![
                PolicyCondition::Source {
                    field: SourceField::ServiceName,
                    operator: Operator::Equals,
                    value: serde_json::json!("API Gateway"),
                },
                PolicyCondition::Destination {
                    field: DestinationField::ServiceName,
                    operator: Operator::Equals,
                    value: serde_json::json!("Auth Service"),
                },
            ],
        });
        engine
            .add_policy(allow_policy)
            .map_err(|e| e.to_string())?;

        let mut deny_policy = Policy::new("Deny Low Trust Access", 20, PolicyAction::Deny);
        deny_policy.description =
            Some("Blocks requests when trust scores drop below the isolation threshold".to_string());
        deny_policy.conditions.push(PolicyCondition::RiskScore {
            operator: Operator::LessThan,
            threshold: 0.45,
        });
        engine
            .add_policy(deny_policy)
            .map_err(|e| e.to_string())?;
    }

    let now = Utc::now();
    let api_gateway_id = find_service_id("API Gateway")?;
    let auth_service_id = find_service_id("Auth Service")?;

    state
        .db
        .execute(
            "INSERT INTO tunnels (
                id, service_a_id, service_b_id, interface_name, private_key_encrypted,
                public_key, virtual_ip, peer_endpoint, status, last_handshake,
                bytes_sent, bytes_received, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            &[
                &Uuid::new_v4().to_string(),
                &api_gateway_id,
                &auth_service_id,
                &"wg-demo0".to_string(),
                &vec![1u8, 2, 3, 4],
                &"demo-public-key".to_string(),
                &"10.128.0.10".to_string(),
                &Some("127.0.0.1:51820".to_string()),
                &"active".to_string(),
                &Some(now.to_rfc3339()),
                &1048576i64,
                &2097152i64,
                &now.to_rfc3339(),
            ],
        )
        .map_err(|e| e.to_string())?;

    let attack_rows = [
        (
            "SYN Flood",
            "45.123.45.67",
            "10.128.0.10",
            "Critical",
            823i64,
            true,
            now - Duration::minutes(4),
        ),
        (
            "Port Scan",
            "203.0.113.25",
            "10.128.0.20",
            "High",
            58i64,
            true,
            now - Duration::minutes(12),
        ),
        (
            "HTTP Flood",
            "198.51.100.91",
            "10.128.0.10",
            "Medium",
            212i64,
            false,
            now - Duration::minutes(27),
        ),
    ];

    for (attack_type, source_ip, destination_ip, severity, packet_count, blocked, created_at) in
        attack_rows
    {
        state
            .db
            .execute(
                "INSERT INTO attacks (
                    attack_type, source_ip, destination_ip, severity, packet_count, blocked, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                &[
                    &attack_type.to_string(),
                    &source_ip.to_string(),
                    &destination_ip.to_string(),
                    &severity.to_string(),
                    &packet_count,
                    &blocked,
                    &created_at.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;
    }

    let alert_rows = [
        (
            "attack_detected",
            "Critical",
            "SYN Flood blocked",
            "Ingress protections blocked a sustained SYN flood against API Gateway.",
            Some("45.123.45.67"),
            false,
            now - Duration::minutes(3),
        ),
        (
            "trust_score",
            "High",
            "Payments worker trust degraded",
            "Behavioral scoring dropped below the limited-access threshold.",
            Some("Payments Worker"),
            false,
            now - Duration::minutes(9),
        ),
        (
            "policy_update",
            "Info",
            "Baseline policies loaded",
            "Demo bootstrap inserted starter allow/deny policies for the mesh.",
            None,
            true,
            now - Duration::minutes(18),
        ),
    ];

    for (alert_type, severity, title, message, source, acknowledged, created_at) in alert_rows {
        state
            .db
            .execute(
                "INSERT INTO alerts (
                    alert_type, severity, title, message, source, acknowledged, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                &[
                    &alert_type.to_string(),
                    &severity.to_string(),
                    &title.to_string(),
                    &message.to_string(),
                    &source.map(|value| value.to_string()),
                    &acknowledged,
                    &created_at.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;
    }

    state
        .db
        .execute(
            "INSERT INTO blacklist (ip, reason, auto_generated, created_at) VALUES (?1, ?2, ?3, ?4)",
            &[
                &"45.123.45.67".to_string(),
                &"Repeated SYN flood activity".to_string(),
                &true,
                &now.to_rfc3339(),
            ],
        )
        .map_err(|e| e.to_string())?;

    let audit_rows = [
        ("system", "seed_demo_data", Some("demo"), Some("Inserted representative development data")),
        ("identity", "register_service", Some("API Gateway"), Some("Demo service created")),
        ("policy", "create", Some("Allow API to Auth"), Some("Demo policy created")),
        ("attack", "record", Some("SYN Flood"), Some("Attack event inserted for UI verification")),
    ];

    for (event_type, action, subject, details) in audit_rows {
        state
            .db
            .execute(
                "INSERT INTO audit_logs (event_type, action, subject, details, success, created_at)
                 VALUES (?1, ?2, ?3, ?4, 1, ?5)",
                &[
                    &event_type.to_string(),
                    &action.to_string(),
                    &subject.map(|value| value.to_string()),
                    &details.map(|value| value.to_string()),
                    &now.to_rfc3339(),
                ],
            )
            .map_err(|e| e.to_string())?;
    }

    info!("Seeded demonstration data for the desktop UI");

    Ok(DemoDataResponse {
        services: services.len() as u64,
        policies: 2,
        tunnels: 1,
        attacks: 3,
        alerts: 3,
        audit_logs: 4,
    })
}

fn reset_demo_tables(state: &std::sync::Arc<zerotrust_mesh_lib::AppState>) -> Result<(), String> {
    let statements = [
        "DELETE FROM certificates",
        "DELETE FROM jwt_tokens",
        "DELETE FROM trust_scores",
        "DELETE FROM tunnels",
        "DELETE FROM attacks",
        "DELETE FROM alerts",
        "DELETE FROM blacklist",
        "DELETE FROM whitelist",
        "DELETE FROM audit_logs",
        "DELETE FROM config_changes",
        "DELETE FROM services",
    ];

    for statement in statements {
        state.db.execute(statement, &[]).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn pathbuf_to_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn find_first_existing_binary(candidates: &[&str]) -> Option<PathBuf> {
    candidates
        .iter()
        .map(Path::new)
        .find(|path| path.exists())
        .map(PathBuf::from)
}

fn resolve_demo_binary_path(service_name: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let candidates: &[&str] = match service_name {
        "API Gateway" => &[
            "C:\\Windows\\System32\\cmd.exe",
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            "C:\\Windows\\System32\\notepad.exe",
        ],
        "Auth Service" => &[
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
            "C:\\Windows\\System32\\cmd.exe",
            "C:\\Windows\\System32\\notepad.exe",
        ],
        "Payments Worker" => &[
            "C:\\Windows\\System32\\notepad.exe",
            "C:\\Windows\\System32\\cmd.exe",
            "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
        ],
        _ => &[],
    };

    #[cfg(not(target_os = "windows"))]
    let candidates: &[&str] = match service_name {
        "API Gateway" => &["/usr/bin/env", "/bin/sh", "/bin/ls"],
        "Auth Service" => &["/bin/sh", "/usr/bin/env", "/bin/ls"],
        "Payments Worker" => &["/bin/ls", "/usr/bin/env", "/bin/sh"],
        _ => &[],
    };

    find_first_existing_binary(candidates).or_else(|| std::env::current_exe().ok())
}
