use super::*;
use crate::systemctl::executor::MockCommandExecutor;
use std::sync::Arc;

#[test]
fn test_validate_service_name_valid() {
    assert!(validate_service_name("nginx").is_ok());
    assert!(validate_service_name("chfscraper@rest").is_ok());
    assert!(validate_service_name("my-service_123").is_ok());
    assert!(validate_service_name("service.name").is_ok());
}

#[test]
fn test_validate_service_name_invalid() {
    // Empty name
    assert!(validate_service_name("").is_err());

    // Contains spaces
    let result = validate_service_name("my service");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ServiceError::InvalidServiceName(_)));

    // Path traversal attempt
    let result = validate_service_name("../../etc/passwd");
    assert!(result.is_err());

    // Command injection attempt
    let result = validate_service_name("nginx; rm -rf /");
    assert!(result.is_err());

    // Special characters
    let result = validate_service_name("service$name");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_services_success() {
    let output = r#"nginx.service                  loaded active   running NGINX HTTP Server
postgresql.service             loaded active   running PostgreSQL Database
redis.service                  loaded inactive dead    Redis Server"#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "systemctl",
            &["list-units", "--type=service", "--all", "--no-pager", "--plain", "--no-legend"],
            output,
        )
    );

    let services = list_services(executor).await.unwrap();
    assert_eq!(services.len(), 3);
    assert_eq!(services[0].name, "nginx.service");
    assert_eq!(services[0].active_state, "active");
    assert_eq!(services[1].name, "postgresql.service");
    assert_eq!(services[2].active_state, "inactive");
}

#[tokio::test]
async fn test_list_services_empty() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "systemctl",
            &["list-units", "--type=service", "--all", "--no-pager", "--plain", "--no-legend"],
            "",
        )
    );

    let services = list_services(executor).await.unwrap();
    assert_eq!(services.len(), 0);
}

#[tokio::test]
async fn test_get_service_status_running() {
    let output = r#"ActiveState=active
SubState=running
MainPID=1234
ActiveEnterTimestamp=1705315845000000"#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "systemctl",
            &["show", "nginx", "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"],
            output,
        )
    );

    let status = get_service_status(executor, "nginx").await.unwrap();
    assert_eq!(status.name, "nginx");
    assert_eq!(status.active_state, "active");
    assert_eq!(status.sub_state, "running");
    assert_eq!(status.main_pid, Some(1234));
    assert!(status.uptime_seconds > 0);
}

#[tokio::test]
async fn test_get_service_status_stopped() {
    let output = r#"ActiveState=inactive
SubState=dead
MainPID=0
ActiveEnterTimestamp="#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "systemctl",
            &["show", "stopped-service", "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"],
            output,
        )
    );

    let status = get_service_status(executor, "stopped-service").await.unwrap();
    assert_eq!(status.active_state, "inactive");
    assert_eq!(status.sub_state, "dead");
    assert_eq!(status.main_pid, None);
    assert_eq!(status.uptime_seconds, 0);
}

#[tokio::test]
async fn test_get_service_status_failed() {
    let output = r#"ActiveState=failed
SubState=failed
MainPID=0
ActiveEnterTimestamp=1705315845000000"#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "systemctl",
            &["show", "failed-service", "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"],
            output,
        )
    );

    let status = get_service_status(executor, "failed-service").await.unwrap();
    assert_eq!(status.active_state, "failed");
    assert_eq!(status.sub_state, "failed");
}

#[tokio::test]
async fn test_start_service_success() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout("systemctl", &["start", "nginx"], "")
    );

    let result = start_service(executor, "nginx").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_not_found() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_error(
            "systemctl",
            &["start", "nonexistent"],
            5,
            "Unit nonexistent.service not found."
        )
    );

    let result = start_service(executor, "nonexistent").await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ServiceError::ServiceNotFound(_)));
}

#[tokio::test]
async fn test_start_service_permission_denied() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_error(
            "systemctl",
            &["start", "protected"],
            4,
            "Access denied"
        )
    );

    let result = start_service(executor, "protected").await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ServiceError::PermissionDenied(_)));
}

#[tokio::test]
async fn test_stop_service_success() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout("systemctl", &["stop", "nginx"], "")
    );

    let result = stop_service(executor, "nginx").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_restart_service_success() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout("systemctl", &["restart", "nginx"], "")
    );

    let result = restart_service(executor, "nginx").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_logs_success() {
    let output = r#"{"MESSAGE":"Service started","PRIORITY":"6","__REALTIME_TIMESTAMP":"1705315845000000"}
{"MESSAGE":"Processing request","PRIORITY":"6","__REALTIME_TIMESTAMP":"1705315846000000"}
{"MESSAGE":"Error occurred","PRIORITY":"3","__REALTIME_TIMESTAMP":"1705315847000000"}"#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "journalctl",
            &["-u", "nginx", "-n", "100", "--no-pager", "--output=json"],
            output,
        )
    );

    let logs = get_logs(executor, "nginx", 100).await.unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].message, "Service started");
    assert_eq!(logs[0].priority, 6);
    assert_eq!(logs[2].message, "Error occurred");
    assert_eq!(logs[2].priority, 3);
}

#[tokio::test]
async fn test_get_logs_empty() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "journalctl",
            &["-u", "new-service", "-n", "50", "--no-pager", "--output=json"],
            "",
        )
    );

    let logs = get_logs(executor, "new-service", 50).await.unwrap();
    assert_eq!(logs.len(), 0);
}

#[tokio::test]
async fn test_get_logs_service_not_found() {
    let executor = Arc::new(
        MockCommandExecutor::new().with_error(
            "journalctl",
            &["-u", "nonexistent", "-n", "100", "--no-pager", "--output=json"],
            1,
            "Unit nonexistent.service does not exist",
        )
    );

    let result = get_logs(executor, "nonexistent", 100).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ServiceError::ServiceNotFound(_)));
}

#[tokio::test]
async fn test_get_logs_custom_line_count() {
    let output = r#"{"MESSAGE":"Log line 1","PRIORITY":"6","__REALTIME_TIMESTAMP":"1705315845000000"}
{"MESSAGE":"Log line 2","PRIORITY":"6","__REALTIME_TIMESTAMP":"1705315846000000"}"#;

    let executor = Arc::new(
        MockCommandExecutor::new().with_stdout(
            "journalctl",
            &["-u", "nginx", "-n", "50", "--no-pager", "--output=json"],
            output,
        )
    );

    let logs = get_logs(executor, "nginx", 50).await.unwrap();
    assert_eq!(logs.len(), 2);
}

#[tokio::test]
async fn test_service_name_injection_prevention() {
    let executor = Arc::new(MockCommandExecutor::new());

    // Test various injection attempts
    let injection_attempts = vec![
        "nginx; rm -rf /",
        "../../../etc/passwd",
        "service$name",
        "service`whoami`",
        "service$(whoami)",
        "service|cat /etc/passwd",
        "service&& rm -rf /",
    ];

    for attempt in injection_attempts {
        let result = start_service(executor.clone(), attempt).await;
        assert!(result.is_err(), "Should reject injection attempt: {}", attempt);
        let err = result.unwrap_err();
        assert!(matches!(err, ServiceError::InvalidServiceName(_)));
    }
}
