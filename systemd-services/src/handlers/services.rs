// Service-related HTTP handlers

use crate::{
    error::{Result, ServiceError},
    systemctl::CommandExecutor,
};
use super::{json_response, error_response, success_response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use toru_plugin_api::{HttpResponse, PluginKvStore};

/// Response format for GET /services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatusResponse {
    pub name: String,
    pub status: String,  // "running", "failed", "inactive"
    pub active_state: String,
    pub sub_state: String,
    pub uptime_seconds: u64,
}

/// Handle GET /services - return watched services with status
pub async fn handle_get_services<E: CommandExecutor>(
    executor: Arc<E>,
    kv_store: &dyn PluginKvStore,
) -> Result<HttpResponse> {
    // Get watched services from KV storage
    let watched_services = get_watched_services(kv_store).await?;

    let mut results = Vec::new();

    // Get status for each watched service
    for service_name in watched_services {
        match crate::systemctl::get_service_status(executor.clone(), &service_name).await {
            Ok(status) => {
                // Map active_state to simple status
                let simple_status = match status.active_state.as_str() {
                    "active" => "running",
                    "failed" => "failed",
                    _ => "inactive",
                };

                results.push(ServiceStatusResponse {
                    name: status.name,
                    status: simple_status.to_string(),
                    active_state: status.active_state,
                    sub_state: status.sub_state,
                    uptime_seconds: status.uptime_seconds,
                });
            }
            Err(e) => {
                // Include services that failed to query but mark them as unavailable
                eprintln!("Failed to get status for {}: {}", service_name, e);
                results.push(ServiceStatusResponse {
                    name: service_name.clone(),
                    status: "unknown".to_string(),
                    active_state: "unknown".to_string(),
                    sub_state: "unknown".to_string(),
                    uptime_seconds: 0,
                });
            }
        }
    }

    json_response(200, results)
}

/// Handle GET /services/available - return all systemd services
pub async fn handle_get_available_services<E: CommandExecutor>(
    executor: Arc<E>,
) -> Result<HttpResponse> {
    let services = crate::systemctl::list_services(executor).await?;
    json_response(200, services)
}

/// Handle POST /services/:name/start|stop|restart
pub async fn handle_service_action<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str,
    action: &str,
) -> Result<HttpResponse> {
    // Validate service name
    crate::systemctl::validate_service_name(service_name)?;

    // Execute action
    let result = match action {
        "start" => crate::systemctl::start_service(executor, service_name).await,
        "stop" => crate::systemctl::stop_service(executor, service_name).await,
        "restart" => crate::systemctl::restart_service(executor, service_name).await,
        _ => {
            return error_response(400, &format!("Invalid action: {}", action));
        }
    };

    match result {
        Ok(_) => success_response(&format!("Service {} successful", action)),
        Err(ServiceError::ServiceNotFound(_)) => {
            error_response(404, "Service not found")
        }
        Err(ServiceError::PermissionDenied(_)) => {
            error_response(403, "Permission denied")
        }
        Err(e) => {
            error_response(500, &format!("Failed to {} service: {}", action, e))
        }
    }
}

/// Handle GET /services/:name/logs?lines=100
pub async fn handle_get_logs<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str,
    query_params: &std::collections::HashMap<String, String>,
) -> Result<HttpResponse> {
    // Validate service name
    crate::systemctl::validate_service_name(service_name)?;

    // Parse lines parameter (default to 100)
    let lines = query_params
        .get("lines")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(100);

    // Get logs
    match crate::systemctl::get_logs(executor, service_name, lines).await {
        Ok(logs) => json_response(200, logs),
        Err(ServiceError::ServiceNotFound(_)) => {
            error_response(404, "Service not found")
        }
        Err(e) => {
            error_response(500, &format!("Failed to get logs: {}", e))
        }
    }
}

/// Helper: Get watched services from KV storage
async fn get_watched_services(kv_store: &dyn PluginKvStore) -> Result<Vec<String>> {
    match kv_store.get("watched_services").await? {
        Some(json_str) => {
            let services: Vec<String> = serde_json::from_str(&json_str)?;
            Ok(services)
        }
        None => Ok(Vec::new()),
    }
}

/// Helper: Save watched services to KV storage
#[allow(dead_code)]
pub async fn save_watched_services(
    kv_store: &dyn PluginKvStore,
    services: &[String],
) -> Result<()> {
    let json_str = serde_json::to_string(services)?;
    kv_store.set("watched_services", &json_str).await?;
    Ok(())
}
