mod executor;
mod parser;

#[cfg(test)]
mod tests;

pub use executor::{CommandExecutor, SystemCommandExecutor, MockCommandExecutor, CommandOutput};

use crate::error::{Result, ServiceError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::sync::Arc;

/// Information about a systemd service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub load_state: String,
    pub active_state: String,
    pub sub_state: String,
}

/// Detailed status of a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub active_state: String,
    pub sub_state: String,
    pub uptime_seconds: u64,
    pub main_pid: Option<u32>,
    pub active_enter_timestamp: Option<DateTime<Utc>>,
}

/// Log entry from journalctl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub priority: u8,
}

/// Validates service name to prevent command injection
pub fn validate_service_name(name: &str) -> Result<()> {
    let valid_pattern = Regex::new(r"^[a-zA-Z0-9@._-]+$").unwrap();

    if name.is_empty() {
        return Err(ServiceError::InvalidServiceName("Service name cannot be empty".to_string()));
    }

    if name.contains(' ') {
        return Err(ServiceError::InvalidServiceName(format!("Service name cannot contain spaces: {}", name)));
    }

    if !valid_pattern.is_match(name) {
        return Err(ServiceError::InvalidServiceName(format!("Service name contains invalid characters: {}", name)));
    }

    Ok(())
}

/// Lists all systemd services
pub async fn list_services<E: CommandExecutor>(executor: Arc<E>) -> Result<Vec<ServiceInfo>> {
    let output = executor.execute("systemctl", &[
        "list-units",
        "--type=service",
        "--all",
        "--no-pager",
        "--plain",
        "--no-legend"
    ]).await?;

    parser::parse_service_list(&output.stdout)
}

/// Gets detailed status of a specific service
pub async fn get_service_status<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str
) -> Result<ServiceStatus> {
    validate_service_name(service_name)?;

    let output = executor.execute("systemctl", &[
        "show",
        service_name,
        "--property=ActiveState,SubState,MainPID,ActiveEnterTimestamp"
    ]).await?;

    parser::parse_service_status(service_name, &output.stdout)
}

/// Starts a systemd service
pub async fn start_service<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str
) -> Result<()> {
    validate_service_name(service_name)?;

    let output = executor.execute("systemctl", &["start", service_name]).await?;

    if output.exit_code != 0 {
        return Err(parse_systemctl_error(&output));
    }

    Ok(())
}

/// Stops a systemd service
pub async fn stop_service<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str
) -> Result<()> {
    validate_service_name(service_name)?;

    let output = executor.execute("systemctl", &["stop", service_name]).await?;

    if output.exit_code != 0 {
        return Err(parse_systemctl_error(&output));
    }

    Ok(())
}

/// Restarts a systemd service
pub async fn restart_service<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str
) -> Result<()> {
    validate_service_name(service_name)?;

    let output = executor.execute("systemctl", &["restart", service_name]).await?;

    if output.exit_code != 0 {
        return Err(parse_systemctl_error(&output));
    }

    Ok(())
}

/// Gets recent logs for a service
pub async fn get_logs<E: CommandExecutor>(
    executor: Arc<E>,
    service_name: &str,
    lines: u32
) -> Result<Vec<LogEntry>> {
    validate_service_name(service_name)?;

    let lines_str = lines.to_string();
    let output = executor.execute("journalctl", &[
        "-u", service_name,
        "-n", &lines_str,
        "--no-pager",
        "--output=json"
    ]).await?;

    if output.exit_code != 0 {
        // Check if service doesn't exist
        if output.stderr.contains("No journal files were found") ||
           output.stderr.contains("No entries") {
            return Ok(Vec::new());
        }

        return Err(parse_journalctl_error(&output));
    }

    parser::parse_logs(&output.stdout)
}

/// Parses systemctl error from command output
fn parse_systemctl_error(output: &CommandOutput) -> ServiceError {
    match output.exit_code {
        4 => ServiceError::PermissionDenied(output.stderr.clone()),
        5 => ServiceError::ServiceNotFound(output.stderr.clone()),
        _ => ServiceError::CommandFailed {
            command: "systemctl".to_string(),
            exit_code: output.exit_code,
            stderr: output.stderr.clone(),
        }
    }
}

/// Parses journalctl error from command output
fn parse_journalctl_error(output: &CommandOutput) -> ServiceError {
    if output.stderr.contains("not found") || output.stderr.contains("does not exist") {
        return ServiceError::ServiceNotFound(output.stderr.clone());
    }

    ServiceError::CommandFailed {
        command: "journalctl".to_string(),
        exit_code: output.exit_code,
        stderr: output.stderr.clone(),
    }
}

