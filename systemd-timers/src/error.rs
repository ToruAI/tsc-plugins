use std::fmt;

#[derive(Debug, Clone)]
pub enum TimerError {
    /// Timer or service not found
    NotFound(String),

    /// Command execution failed
    CommandFailed { command: String, stderr: String, exit_code: Option<i32> },

    /// Failed to parse systemd output
    ParseError { source: String, reason: String },

    /// Invalid input (timer name, etc.)
    InvalidInput(String),

    /// Permission denied
    PermissionDenied(String),

    /// I/O error
    IoError(String),

    /// JSON serialization/deserialization error
    JsonError(String),
}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerError::NotFound(name) => write!(f, "Timer or service not found: {}", name),
            TimerError::CommandFailed { command, stderr, exit_code } => {
                write!(f, "Command '{}' failed with exit code {:?}: {}", command, exit_code, stderr)
            }
            TimerError::ParseError { source, reason } => {
                write!(f, "Failed to parse {}: {}", source, reason)
            }
            TimerError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            TimerError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            TimerError::IoError(msg) => write!(f, "I/O error: {}", msg),
            TimerError::JsonError(msg) => write!(f, "JSON error: {}", msg),
        }
    }
}

impl std::error::Error for TimerError {}

impl From<std::io::Error> for TimerError {
    fn from(err: std::io::Error) -> Self {
        TimerError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for TimerError {
    fn from(err: serde_json::Error) -> Self {
        TimerError::JsonError(err.to_string())
    }
}

pub type TimerResult<T> = Result<T, TimerError>;
