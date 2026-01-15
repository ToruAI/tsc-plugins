// Tests for HTTP handlers

use super::*;
use crate::systemctl::{CommandOutput, MockCommandExecutor};
use std::sync::Arc;
use toru_plugin_api::{PluginKvStore, PluginResult};

// Mock KV Store for testing
struct TestKvStore {
    data: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl TestKvStore {
    fn new() -> Self {
        Self {
            data: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    fn with_data(data: std::collections::HashMap<String, String>) -> Self {
        Self {
            data: std::sync::Mutex::new(data),
        }
    }
}

#[async_trait::async_trait]
impl PluginKvStore for TestKvStore {
    async fn get(&self, key: &str) -> PluginResult<Option<String>> {
        Ok(self.data.lock().unwrap().get(key).cloned())
    }

    async fn set(&self, key: &str, value: &str) -> PluginResult<()> {
        self.data.lock().unwrap().insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn delete(&self, key: &str) -> PluginResult<()> {
        self.data.lock().unwrap().remove(key);
        Ok(())
    }
}

#[tokio::test]
async fn test_get_services_empty_list() {
    let executor = Arc::new(MockCommandExecutor::new());
    let kv_store = TestKvStore::new();

    let response = services::handle_get_services(executor, &kv_store).await.unwrap();

    assert_eq!(response.status, 200);
    assert!(response.body.is_some());

    let body: Vec<services::ServiceStatusResponse> =
        serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body.len(), 0);
}

#[tokio::test]
async fn test_get_services_with_watched_services() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["show", "nginx.service", "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"],
            CommandOutput {
                exit_code: 0,
                stdout: "ActiveState=active\nSubState=running\nMainPID=1234\nActiveEnterTimestamp=Wed 2024-01-10 10:00:00 UTC\n".to_string(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    // KV store with watched services
    let mut data = std::collections::HashMap::new();
    data.insert("watched_services".to_string(), r#"["nginx.service"]"#.to_string());
    let kv_store = TestKvStore::with_data(data);

    let response = services::handle_get_services(executor, &kv_store).await.unwrap();

    assert_eq!(response.status, 200);
    let body: Vec<services::ServiceStatusResponse> =
        serde_json::from_str(&response.body.unwrap()).unwrap();

    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "nginx.service");
    assert_eq!(body[0].status, "running");
    assert_eq!(body[0].active_state, "active");
}

#[tokio::test]
async fn test_get_services_handles_failures_gracefully() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["show", "nonexistent.service", "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"],
            CommandOutput {
                exit_code: 5,
                stdout: String::new(),
                stderr: "Unit nonexistent.service could not be found.".to_string(),
            },
        );

    let executor = Arc::new(executor);

    let mut data = std::collections::HashMap::new();
    data.insert("watched_services".to_string(), r#"["nonexistent.service"]"#.to_string());
    let kv_store = TestKvStore::with_data(data);

    let response = services::handle_get_services(executor, &kv_store).await.unwrap();

    assert_eq!(response.status, 200);
    let body: Vec<services::ServiceStatusResponse> =
        serde_json::from_str(&response.body.unwrap()).unwrap();

    // Should still return the service but with unknown status
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "nonexistent.service");
    assert_eq!(body[0].status, "unknown");
}

#[tokio::test]
async fn test_get_available_services() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["list-units", "--type=service", "--all", "--no-pager", "--plain", "--no-legend"],
            CommandOutput {
                exit_code: 0,
                stdout: "nginx.service    loaded active running nginx web server\nsshd.service     loaded active running OpenSSH server daemon\n".to_string(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_get_available_services(executor).await.unwrap();

    assert_eq!(response.status, 200);
    let body: Vec<crate::systemctl::ServiceInfo> =
        serde_json::from_str(&response.body.unwrap()).unwrap();

    assert_eq!(body.len(), 2);
    assert_eq!(body[0].name, "nginx.service");
}

#[tokio::test]
async fn test_service_action_start() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["start", "nginx.service"],
            CommandOutput {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_service_action(executor, "nginx.service", "start").await.unwrap();

    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], true);
    assert_eq!(body["message"], "Service start successful");
}

#[tokio::test]
async fn test_service_action_stop() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["stop", "nginx.service"],
            CommandOutput {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_service_action(executor, "nginx.service", "stop").await.unwrap();

    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], true);
}

#[tokio::test]
async fn test_service_action_restart() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["restart", "nginx.service"],
            CommandOutput {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_service_action(executor, "nginx.service", "restart").await.unwrap();

    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], true);
}

#[tokio::test]
async fn test_service_action_invalid() {
    let executor = Arc::new(MockCommandExecutor::new());

    let response = services::handle_service_action(executor, "nginx.service", "invalid").await.unwrap();

    assert_eq!(response.status, 400);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], false);
    assert!(body["error"].as_str().unwrap().contains("Invalid action"));
}

#[tokio::test]
async fn test_service_action_not_found() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["start", "nonexistent.service"],
            CommandOutput {
                exit_code: 5,
                stdout: String::new(),
                stderr: "Failed to start nonexistent.service: Unit nonexistent.service not found.".to_string(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_service_action(executor, "nonexistent.service", "start").await.unwrap();

    assert_eq!(response.status, 404);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], false);
    assert_eq!(body["error"], "Service not found");
}

#[tokio::test]
async fn test_service_action_permission_denied() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "systemctl",
            &["start", "nginx.service"],
            CommandOutput {
                exit_code: 4,
                stdout: String::new(),
                stderr: "Failed to start nginx.service: Access denied".to_string(),
            },
        );

    let executor = Arc::new(executor);

    let response = services::handle_service_action(executor, "nginx.service", "start").await.unwrap();

    assert_eq!(response.status, 403);
    let body: serde_json::Value = serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body["success"], false);
    assert_eq!(body["error"], "Permission denied");
}

#[tokio::test]
async fn test_get_logs() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "journalctl",
            &["-u", "nginx.service", "-n", "50", "--no-pager", "--output=json"],
            CommandOutput {
                exit_code: 0,
                stdout: r#"{"MESSAGE":"Started nginx","PRIORITY":"6","__REALTIME_TIMESTAMP":"1704902400000000"}
{"MESSAGE":"Stopped nginx","PRIORITY":"6","__REALTIME_TIMESTAMP":"1704902500000000"}"#.to_string(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let mut params = std::collections::HashMap::new();
    params.insert("lines".to_string(), "50".to_string());

    let response = services::handle_get_logs(executor, "nginx.service", &params).await.unwrap();

    assert_eq!(response.status, 200);
    let body: Vec<crate::systemctl::LogEntry> =
        serde_json::from_str(&response.body.unwrap()).unwrap();

    assert_eq!(body.len(), 2);
    assert_eq!(body[0].message, "Started nginx");
}

#[tokio::test]
async fn test_get_logs_default_lines() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "journalctl",
            &["-u", "nginx.service", "-n", "100", "--no-pager", "--output=json"],
            CommandOutput {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
            },
        );

    let executor = Arc::new(executor);

    let params = std::collections::HashMap::new();
    let response = services::handle_get_logs(executor, "nginx.service", &params).await.unwrap();

    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_get_logs_service_not_found() {
    let executor = MockCommandExecutor::new()
        .with_response(
            "journalctl",
            &["-u", "nonexistent.service", "-n", "100", "--no-pager", "--output=json"],
            CommandOutput {
                exit_code: 1,
                stdout: String::new(),
                stderr: "No journal files were found.".to_string(),
            },
        );

    let executor = Arc::new(executor);

    let params = std::collections::HashMap::new();
    let response = services::handle_get_logs(executor, "nonexistent.service", &params).await.unwrap();

    // Should return empty array for services with no logs
    assert_eq!(response.status, 200);
    let body: Vec<crate::systemctl::LogEntry> =
        serde_json::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body.len(), 0);
}

#[tokio::test]
async fn test_parse_query_params() {
    let params = parse_query_params("/services/nginx/logs?lines=50&format=json");
    assert_eq!(params.get("lines"), Some(&"50".to_string()));
    assert_eq!(params.get("format"), Some(&"json".to_string()));
}

#[tokio::test]
async fn test_parse_query_params_no_query() {
    let params = parse_query_params("/services/nginx/logs");
    assert_eq!(params.len(), 0);
}

#[tokio::test]
async fn test_path_without_query() {
    assert_eq!(path_without_query("/services?foo=bar"), "/services");
    assert_eq!(path_without_query("/services"), "/services");
}

#[tokio::test]
async fn test_save_and_load_watched_services() {
    let kv_store = TestKvStore::new();

    let services = vec!["nginx.service".to_string(), "sshd.service".to_string()];
    services::save_watched_services(&kv_store, &services).await.unwrap();

    // Verify it was saved
    let loaded = kv_store.get("watched_services").await.unwrap();
    assert!(loaded.is_some());

    let loaded_services: Vec<String> = serde_json::from_str(&loaded.unwrap()).unwrap();
    assert_eq!(loaded_services, services);
}
