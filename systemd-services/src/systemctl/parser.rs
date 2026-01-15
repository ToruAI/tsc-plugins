use crate::error::{Result, ServiceError};
use crate::systemctl::{ServiceInfo, ServiceStatus, LogEntry};
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Parses systemctl list-units output
pub fn parse_service_list(output: &str) -> Result<Vec<ServiceInfo>> {
    let mut services = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // systemctl list-units format (space-separated):
        // UNIT LOAD ACTIVE SUB DESCRIPTION
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 4 {
            continue; // Skip malformed lines
        }

        let name = parts[0].to_string();
        let load_state = parts[1].to_string();
        let active_state = parts[2].to_string();
        let sub_state = parts[3].to_string();

        // Description is everything after the first 4 fields
        let description = if parts.len() > 4 {
            parts[4..].join(" ")
        } else {
            String::new()
        };

        services.push(ServiceInfo {
            name,
            description,
            load_state,
            active_state,
            sub_state,
        });
    }

    Ok(services)
}

/// Parses systemctl show output for service status
pub fn parse_service_status(service_name: &str, output: &str) -> Result<ServiceStatus> {
    let mut active_state = None;
    let mut sub_state = None;
    let mut main_pid = None;
    let mut active_enter_timestamp = None;

    for line in output.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "ActiveState" => active_state = Some(value.to_string()),
                "SubState" => sub_state = Some(value.to_string()),
                "MainPID" => {
                    if let Ok(pid) = value.parse::<u32>() {
                        if pid != 0 {
                            main_pid = Some(pid);
                        }
                    }
                }
                "ActiveEnterTimestamp" => {
                    if !value.is_empty() {
                        // Try to parse timestamp
                        // Format from systemctl: "Wed 2024-01-15 10:30:45 UTC"
                        if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
                            active_enter_timestamp = Some(dt.with_timezone(&Utc));
                        } else if let Ok(ts) = value.parse::<i64>() {
                            // Handle Unix timestamp in microseconds
                            if let Some(dt) = DateTime::from_timestamp(ts / 1_000_000, 0) {
                                active_enter_timestamp = Some(dt);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let active_state = active_state.ok_or_else(|| {
        ServiceError::ParseError("Missing ActiveState in systemctl output".to_string())
    })?;

    let sub_state = sub_state.ok_or_else(|| {
        ServiceError::ParseError("Missing SubState in systemctl output".to_string())
    })?;

    // Calculate uptime
    let uptime_seconds = if let Some(enter_time) = active_enter_timestamp {
        let now = Utc::now();
        let duration = now.signed_duration_since(enter_time);
        duration.num_seconds().max(0) as u64
    } else {
        0
    };

    Ok(ServiceStatus {
        name: service_name.to_string(),
        active_state,
        sub_state,
        uptime_seconds,
        main_pid,
        active_enter_timestamp,
    })
}

/// Parses journalctl JSON output
pub fn parse_logs(output: &str) -> Result<Vec<LogEntry>> {
    let mut logs = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let json: Value = serde_json::from_str(line)
            .map_err(|e| ServiceError::ParseError(format!("Invalid JSON in journalctl output: {}", e)))?;

        // Extract fields from journalctl JSON
        let message = json["MESSAGE"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let priority = json["PRIORITY"]
            .as_str()
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(6); // Default to INFO priority

        // Parse timestamp - journalctl provides __REALTIME_TIMESTAMP in microseconds
        let timestamp = if let Some(ts_str) = json["__REALTIME_TIMESTAMP"].as_str() {
            if let Ok(micros) = ts_str.parse::<i64>() {
                DateTime::from_timestamp(micros / 1_000_000, ((micros % 1_000_000) * 1000) as u32)
                    .unwrap_or_else(|| Utc::now())
            } else {
                Utc::now()
            }
        } else {
            Utc::now()
        };

        logs.push(LogEntry {
            timestamp,
            message,
            priority,
        });
    }

    Ok(logs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_service_list() {
        let output = r#"nginx.service                  loaded active   running NGINX HTTP Server
postgresql.service             loaded active   running PostgreSQL Database
failed-service.service         loaded failed   failed  Failed Service
inactive.service               loaded inactive dead    Inactive Service"#;

        let services = parse_service_list(output).unwrap();
        assert_eq!(services.len(), 4);

        assert_eq!(services[0].name, "nginx.service");
        assert_eq!(services[0].active_state, "active");
        assert_eq!(services[0].sub_state, "running");
        assert!(services[0].description.contains("NGINX"));

        assert_eq!(services[2].name, "failed-service.service");
        assert_eq!(services[2].active_state, "failed");
    }

    #[test]
    fn test_parse_service_list_empty() {
        let output = "";
        let services = parse_service_list(output).unwrap();
        assert_eq!(services.len(), 0);
    }

    #[test]
    fn test_parse_service_status_running() {
        let output = r#"ActiveState=active
SubState=running
MainPID=1234
ActiveEnterTimestamp=1705315845000000"#;

        let status = parse_service_status("nginx", output).unwrap();
        assert_eq!(status.name, "nginx");
        assert_eq!(status.active_state, "active");
        assert_eq!(status.sub_state, "running");
        assert_eq!(status.main_pid, Some(1234));
        assert!(status.uptime_seconds > 0);
    }

    #[test]
    fn test_parse_service_status_stopped() {
        let output = r#"ActiveState=inactive
SubState=dead
MainPID=0
ActiveEnterTimestamp="#;

        let status = parse_service_status("stopped-service", output).unwrap();
        assert_eq!(status.active_state, "inactive");
        assert_eq!(status.sub_state, "dead");
        assert_eq!(status.main_pid, None);
        assert_eq!(status.uptime_seconds, 0);
    }

    #[test]
    fn test_parse_service_status_failed() {
        let output = r#"ActiveState=failed
SubState=failed
MainPID=0
ActiveEnterTimestamp=1705315845000000"#;

        let status = parse_service_status("failed-service", output).unwrap();
        assert_eq!(status.active_state, "failed");
        assert_eq!(status.sub_state, "failed");
    }

    #[test]
    fn test_parse_service_status_missing_fields() {
        let output = "ActiveState=active";

        let result = parse_service_status("test", output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::ParseError(_)));
    }

    #[test]
    fn test_parse_logs() {
        let output = r#"{"MESSAGE":"Service started","PRIORITY":"6","__REALTIME_TIMESTAMP":"1705315845000000"}
{"MESSAGE":"Error occurred","PRIORITY":"3","__REALTIME_TIMESTAMP":"1705315846000000"}
{"MESSAGE":"Debug info","PRIORITY":"7","__REALTIME_TIMESTAMP":"1705315847000000"}"#;

        let logs = parse_logs(output).unwrap();
        assert_eq!(logs.len(), 3);

        assert_eq!(logs[0].message, "Service started");
        assert_eq!(logs[0].priority, 6);

        assert_eq!(logs[1].message, "Error occurred");
        assert_eq!(logs[1].priority, 3);

        assert_eq!(logs[2].message, "Debug info");
        assert_eq!(logs[2].priority, 7);
    }

    #[test]
    fn test_parse_logs_empty() {
        let output = "";
        let logs = parse_logs(output).unwrap();
        assert_eq!(logs.len(), 0);
    }

    #[test]
    fn test_parse_logs_invalid_json() {
        let output = "not valid json";
        let result = parse_logs(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::ParseError(_)));
    }

    #[test]
    fn test_parse_logs_missing_fields() {
        // Should handle missing optional fields gracefully
        let output = r#"{"MESSAGE":"Test message","__REALTIME_TIMESTAMP":"1705315845000000"}"#;
        let logs = parse_logs(output).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].priority, 6); // Default priority
    }
}
