// Module exports for systemd-services plugin

pub mod error;
pub mod systemctl;

// Re-export commonly used types
pub use error::{ServiceError, Result};
pub use systemctl::{
    CommandExecutor, ServiceInfo, ServiceStatus, LogEntry,
    list_services, get_service_status, start_service,
    stop_service, restart_service, get_logs
};
