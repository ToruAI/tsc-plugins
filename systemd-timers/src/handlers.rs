//! HTTP handlers module - Phase 9
//! Implements all REST API endpoints for the systemd-timers plugin

use crate::command::CommandExecutor;
use crate::error::{TimerError, TimerResult};
use crate::journal::JournalClient;
use crate::systemctl::SystemctlClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toru_plugin_api::{HttpResponse, PluginKvStore};

/// Response format for GET /timers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerStatusResponse {
    pub name: String,
    pub service: String,
    pub enabled: bool,
    pub schedule: String,
    pub schedule_human: String,
    pub next_run: Option<String>,
    pub last_run: Option<String>,
    pub last_result: Option<String>, // "success", "failed", "running"
}

/// Response format for available timers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTimerResponse {
    pub name: String,
    pub description: String,
}

/// Creates a JSON response with given status and data
pub fn json_response<T: Serialize>(status: u16, data: T) -> TimerResult<HttpResponse> {
    let body = serde_json::to_string(&data)?;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    Ok(HttpResponse {
        status,
        headers,
        body: Some(body),
    })
}

/// Creates an error response
pub fn error_response(status: u16, error: &str) -> TimerResult<HttpResponse> {
    let error_obj = serde_json::json!({
        "success": false,
        "error": error
    });

    json_response(status, error_obj)
}

/// Creates a success response
pub fn success_response(message: &str) -> TimerResult<HttpResponse> {
    let success_obj = serde_json::json!({
        "success": true,
        "message": message
    });

    json_response(200, success_obj)
}

/// Parses query parameters from a path
pub fn parse_query_params(path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(query_start) = path.find('?') {
        let query = &path[query_start + 1..];
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = &pair[..eq_pos];
                let value = &pair[eq_pos + 1..];
                params.insert(key.to_string(), value.to_string());
            }
        }
    }

    params
}

/// Extracts path without query parameters
pub fn path_without_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}

/// Handle GET /timers - return watched timers with status
pub async fn handle_get_timers<E: CommandExecutor + Clone>(
    executor: E,
    kv_store: &dyn PluginKvStore,
) -> TimerResult<HttpResponse> {
    // Get watched timers from KV storage
    let watched_timers = get_watched_timers(kv_store).await?;

    if watched_timers.is_empty() {
        return json_response(200, Vec::<TimerStatusResponse>::new());
    }

    let client = SystemctlClient::new(executor.clone());
    let journal = JournalClient::new(executor);
    let mut results = Vec::new();

    for timer_name in watched_timers {
        match client.get_timer_info(&timer_name).await {
            Ok(info) => {
                // Get last execution result from journal
                let last_result = journal
                    .get_execution_history(&info.service, 1)
                    .await
                    .ok()
                    .and_then(|history| history.first().cloned())
                    .map(|h| format!("{:?}", h.status).to_lowercase());

                // Get schedule (will be parsed in future enhancement)
                let schedule_human = if info.schedule.is_empty() {
                    "Schedule not available".to_string()
                } else {
                    info.schedule.clone()
                };

                results.push(TimerStatusResponse {
                    name: info.name,
                    service: info.service,
                    enabled: info.enabled,
                    schedule: info.schedule,
                    schedule_human,
                    next_run: info.next_run,
                    last_run: info.last_trigger,
                    last_result,
                });
            }
            Err(e) => {
                eprintln!("Failed to get info for timer {}: {}", timer_name, e);
                // Include timers that failed to query but mark them as unavailable
                results.push(TimerStatusResponse {
                    name: timer_name.clone(),
                    service: timer_name.replace(".timer", ".service"),
                    enabled: false,
                    schedule: "unknown".to_string(),
                    schedule_human: "Unable to read schedule".to_string(),
                    next_run: None,
                    last_run: None,
                    last_result: None,
                });
            }
        }
    }

    json_response(200, results)
}

/// Handle GET /timers/available - return all systemd timers
pub async fn handle_get_available_timers<E: CommandExecutor>(
    executor: E,
) -> TimerResult<HttpResponse> {
    let client = SystemctlClient::new(executor);
    let timers = client.list_timers().await?;

    let available: Vec<AvailableTimerResponse> = timers
        .into_iter()
        .map(|t| AvailableTimerResponse {
            name: t.name,
            description: format!("Activates {}", t.service),
        })
        .collect();

    json_response(200, available)
}

/// Handle POST /timers/:name/run - run timer now (full production)
pub async fn handle_run_timer<E: CommandExecutor>(
    executor: E,
    timer_name: &str,
) -> TimerResult<HttpResponse> {
    let client = SystemctlClient::new(executor);

    match client.run_timer(timer_name, false).await {
        Ok(_) => {
            let response = serde_json::json!({
                "success": true,
                "message": format!("Timer {} started", timer_name),
                "mode": "production"
            });
            json_response(200, response)
        }
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Timer not found")
        }
        Err(TimerError::PermissionDenied(_)) => {
            error_response(403, "Permission denied")
        }
        Err(e) => {
            error_response(500, &format!("Failed to start timer: {}", e))
        }
    }
}

/// Handle POST /timers/:name/test - run timer in test mode
pub async fn handle_test_timer<E: CommandExecutor>(
    executor: E,
    timer_name: &str,
) -> TimerResult<HttpResponse> {
    let client = SystemctlClient::new(executor);

    match client.run_timer(timer_name, true).await {
        Ok(_) => {
            let response = serde_json::json!({
                "success": true,
                "message": format!("Timer {} started in test mode", timer_name),
                "mode": "test"
            });
            json_response(200, response)
        }
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Timer not found")
        }
        Err(TimerError::PermissionDenied(_)) => {
            error_response(403, "Permission denied")
        }
        Err(e) => {
            error_response(500, &format!("Failed to start timer in test mode: {}", e))
        }
    }
}

/// Handle POST /timers/:name/enable
pub async fn handle_enable_timer<E: CommandExecutor>(
    executor: E,
    timer_name: &str,
) -> TimerResult<HttpResponse> {
    let client = SystemctlClient::new(executor);

    match client.enable_timer(timer_name).await {
        Ok(_) => success_response(&format!("Timer {} enabled", timer_name)),
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Timer not found")
        }
        Err(TimerError::PermissionDenied(_)) => {
            error_response(403, "Permission denied")
        }
        Err(e) => {
            error_response(500, &format!("Failed to enable timer: {}", e))
        }
    }
}

/// Handle POST /timers/:name/disable
pub async fn handle_disable_timer<E: CommandExecutor>(
    executor: E,
    timer_name: &str,
) -> TimerResult<HttpResponse> {
    let client = SystemctlClient::new(executor);

    match client.disable_timer(timer_name).await {
        Ok(_) => success_response(&format!("Timer {} disabled", timer_name)),
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Timer not found")
        }
        Err(TimerError::PermissionDenied(_)) => {
            error_response(403, "Permission denied")
        }
        Err(e) => {
            error_response(500, &format!("Failed to disable timer: {}", e))
        }
    }
}

/// Handle GET /timers/:name/history - get execution history
pub async fn handle_get_history<E: CommandExecutor>(
    executor: E,
    timer_name: &str,
    query_params: &HashMap<String, String>,
) -> TimerResult<HttpResponse> {
    // Convert timer name to service name
    let service_name = timer_name.replace(".timer", ".service");

    // Parse limit parameter (default to 20)
    let limit = query_params
        .get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20);

    let client = JournalClient::new(executor);

    match client.get_execution_history(&service_name, limit).await {
        Ok(history) => json_response(200, history),
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Timer not found")
        }
        Err(e) => {
            error_response(500, &format!("Failed to get history: {}", e))
        }
    }
}

/// Handle GET /timers/:name/history/:invocation_id - get execution details
pub async fn handle_get_history_details<E: CommandExecutor + Clone>(
    executor: E,
    timer_name: &str,
    invocation_id: &str,
) -> TimerResult<HttpResponse> {
    // Convert timer name to service name
    let service_name = timer_name.replace(".timer", ".service");
    let base_name = service_name.trim_end_matches(".service");

    let client = JournalClient::new(executor.clone());

    match client.get_execution_details(&service_name, invocation_id).await {
        Ok(mut details) => {
            // Try to get actual log file output instead of journal messages
            let log_file = format!("/var/log/{}.log", base_name);
            if let Ok(output) = executor.execute("tail", &["-n", "200", &log_file]).await {
                if output.exit_code == 0 && !output.stdout.is_empty() {
                    // Replace journal output with actual log file content
                    details.output = output.stdout.lines().map(|s| s.to_string()).collect();
                }
            }
            json_response(200, details)
        }
        Err(TimerError::NotFound(_)) => {
            error_response(404, "Execution not found")
        }
        Err(e) => {
            error_response(500, &format!("Failed to get execution details: {}", e))
        }
    }
}

/// Handle POST /timers/settings - save watched timers
pub async fn handle_save_settings(
    kv_store: &dyn PluginKvStore,
    body: &str,
) -> TimerResult<HttpResponse> {
    // Parse request body
    #[derive(Deserialize)]
    struct SaveSettingsRequest {
        watched_timers: Vec<String>,
    }

    let request: SaveSettingsRequest = serde_json::from_str(body).map_err(|e| {
        TimerError::ParseError {
            source: "request_body".to_string(),
            reason: e.to_string(),
        }
    })?;

    // Save to KV storage
    save_watched_timers(kv_store, &request.watched_timers).await?;

    success_response("Settings saved")
}

/// Handle GET /timers/settings - get current settings
pub async fn handle_get_settings(
    kv_store: &dyn PluginKvStore,
) -> TimerResult<HttpResponse> {
    let watched_timers = get_watched_timers(kv_store).await?;

    let response = serde_json::json!({
        "watched_timers": watched_timers
    });

    json_response(200, response)
}

/// Helper: Get watched timers from KV storage
pub async fn get_watched_timers(kv_store: &dyn PluginKvStore) -> TimerResult<Vec<String>> {
    match kv_store.get("watched_timers").await {
        Ok(Some(json_str)) => {
            let timers: Vec<String> = serde_json::from_str(&json_str)?;
            Ok(timers)
        }
        Ok(None) => Ok(Vec::new()),
        Err(e) => Err(TimerError::IoError(format!("KV storage error: {}", e))),
    }
}

/// Helper: Save watched timers to KV storage
pub async fn save_watched_timers(
    kv_store: &dyn PluginKvStore,
    timers: &[String],
) -> TimerResult<()> {
    let json_str = serde_json::to_string(timers)?;
    kv_store.set("watched_timers", &json_str).await
        .map_err(|e| TimerError::IoError(format!("KV storage error: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_params() {
        let path = "/timers/test.timer/history?limit=50&offset=10";
        let params = parse_query_params(path);
        assert_eq!(params.get("limit"), Some(&"50".to_string()));
        assert_eq!(params.get("offset"), Some(&"10".to_string()));
    }

    #[test]
    fn test_path_without_query() {
        let path = "/timers/test.timer/history?limit=50";
        assert_eq!(path_without_query(path), "/timers/test.timer/history");
    }

    #[test]
    fn test_error_response() {
        let resp = error_response(404, "Not found").unwrap();
        assert_eq!(resp.status, 404);
        assert!(resp.body.as_ref().unwrap().contains("Not found"));
    }

    #[test]
    fn test_success_response() {
        let resp = success_response("Timer started").unwrap();
        assert_eq!(resp.status, 200);
        assert!(resp.body.as_ref().unwrap().contains("success"));
        assert!(resp.body.as_ref().unwrap().contains("Timer started"));
    }
}
