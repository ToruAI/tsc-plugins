use std::fmt;

/// Result type for service operations
pub type Result<T> = std::result::Result<T, ServiceError>;

/// Error types for systemd service operations
#[derive(Debug, Clone)]
pub enum ServiceError {
    /// Service not found in systemd
    ServiceNotFound(String),

    /// Permission denied (need root/sudo)
    PermissionDenied(String),

    /// Invalid service name (potential injection attack)
    InvalidServiceName(String),

    /// Failed to parse systemctl/journalctl output
    ParseError(String),

    /// Command execution timeout
    Timeout(String),

    /// Command execution failed
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: String,
    },

    /// Generic I/O error
    IoError(String),

    /// Other systemctl/journalctl errors
    Other(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ServiceNotFound(name) => {
                write!(f, "Service not found: {}", name)
            }
            ServiceError::PermissionDenied(msg) => {
                write!(f, "Permission denied: {}", msg)
            }
            ServiceError::InvalidServiceName(name) => {
                write!(f, "Invalid service name: {}", name)
            }
            ServiceError::ParseError(msg) => {
                write!(f, "Failed to parse output: {}", msg)
            }
            ServiceError::Timeout(msg) => {
                write!(f, "Operation timed out: {}", msg)
            }
            ServiceError::CommandFailed { command, exit_code, stderr } => {
                write!(f, "Command '{}' failed with exit code {}: {}", command, exit_code, stderr)
            }
            ServiceError::IoError(msg) => {
                write!(f, "I/O error: {}", msg)
            }
            ServiceError::Other(msg) => {
                write!(f, "Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::ParseError(format!("JSON parse error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ServiceError::ServiceNotFound("nginx".to_string());
        assert_eq!(err.to_string(), "Service not found: nginx");

        let err = ServiceError::InvalidServiceName("../../etc/passwd".to_string());
        assert!(err.to_string().contains("Invalid service name"));

        let err = ServiceError::CommandFailed {
            command: "systemctl start nginx".to_string(),
            exit_code: 1,
            stderr: "Failed to start".to_string(),
        };
        assert!(err.to_string().contains("exit code 1"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let service_err: ServiceError = io_err.into();
        assert!(matches!(service_err, ServiceError::IoError(_)));
    }
}
