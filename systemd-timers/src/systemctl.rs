use crate::command::CommandExecutor;
use crate::error::{TimerError, TimerResult};
use serde::{Deserialize, Serialize};

/// Information about a systemd timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerInfo {
    pub name: String,
    pub enabled: bool,
    pub schedule: String,
    pub next_run: Option<String>,
    pub last_trigger: Option<String>,
    pub service: String,
}

/// Systemctl wrapper for timer operations
pub struct SystemctlClient<E: CommandExecutor> {
    executor: E,
}

impl<E: CommandExecutor> SystemctlClient<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// List all systemd timers
    pub async fn list_timers(&self) -> TimerResult<Vec<TimerInfo>> {
        let output = self.executor
            .execute("systemctl", &["list-timers", "--all", "--no-pager", "--plain"])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: "systemctl list-timers".to_string(),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        self.parse_list_timers(&output.stdout)
    }

    /// Get detailed information about a specific timer
    pub async fn get_timer_info(&self, name: &str) -> TimerResult<TimerInfo> {
        Self::validate_timer_name(name)?;

        let output = self.executor
            .execute("systemctl", &[
                "show",
                name,
                "--property=Id,LoadState,UnitFileState,NextElapseUSecRealtime,LastTriggerUSec",
            ])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("systemctl show {}", name),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        self.parse_timer_info(&output.stdout, name)
    }

    /// Trigger a timer's associated service immediately
    pub async fn run_timer(&self, name: &str, test_mode: bool) -> TimerResult<()> {
        Self::validate_timer_name(name)?;

        let service = Self::timer_to_service(name)?;

        // Use --no-block to return immediately without waiting for service completion
        let output = if test_mode {
            self.executor
                .execute("systemctl", &["start", "--no-block", &service])
                .await?
        } else {
            self.executor
                .execute("systemctl", &["start", "--no-block", &service])
                .await?
        };

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("systemctl start {}", service),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        Ok(())
    }

    /// Enable a timer
    pub async fn enable_timer(&self, name: &str) -> TimerResult<()> {
        Self::validate_timer_name(name)?;

        let output = self.executor
            .execute("systemctl", &["enable", name])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("systemctl enable {}", name),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        Ok(())
    }

    /// Disable a timer
    pub async fn disable_timer(&self, name: &str) -> TimerResult<()> {
        Self::validate_timer_name(name)?;

        let output = self.executor
            .execute("systemctl", &["disable", name])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("systemctl disable {}", name),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        Ok(())
    }

    /// Validate timer name to prevent command injection
    fn validate_timer_name(name: &str) -> TimerResult<()> {
        if name.is_empty() {
            return Err(TimerError::InvalidInput("Timer name cannot be empty".to_string()));
        }

        if name.contains(['/', '\\', '|', '&', ';', '`', '$', '\n', '\r']) {
            return Err(TimerError::InvalidInput(
                "Timer name contains invalid characters".to_string()
            ));
        }

        if !name.ends_with(".timer") && !name.ends_with(".service") {
            return Err(TimerError::InvalidInput(
                "Timer name must end with .timer or .service".to_string()
            ));
        }

        Ok(())
    }

    /// Convert timer name to service name (foo.timer -> foo.service)
    fn timer_to_service(timer: &str) -> TimerResult<String> {
        if let Some(base) = timer.strip_suffix(".timer") {
            Ok(format!("{}.service", base))
        } else {
            Err(TimerError::InvalidInput(
                "Timer name must end with .timer".to_string()
            ))
        }
    }

    /// Parse output from systemctl list-timers
    fn parse_list_timers(&self, output: &str) -> TimerResult<Vec<TimerInfo>> {
        let mut timers = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("NEXT") || line.starts_with("---") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 7 {
                continue;
            }

            // Format: NEXT LEFT LAST PASSED UNIT ACTIVATES
            // Example: Wed 2026-01-15 14:00:00 CET 45min left n/a n/a chfscraper-scrape-bcp.timer chfscraper-scrape-bcp.service

            let unit_idx = parts.len() - 2;
            let activates_idx = parts.len() - 1;

            let timer_name = parts[unit_idx].to_string();
            let service_name = parts[activates_idx].to_string();

            timers.push(TimerInfo {
                name: timer_name,
                enabled: true, // We'll determine this more accurately in get_timer_info
                schedule: "".to_string(), // Parsed separately
                next_run: if parts[0] == "n/a" { None } else { Some(parts[0..5].join(" ")) },
                last_trigger: if parts[5] == "n/a" { None } else { Some(parts[5].to_string()) },
                service: service_name,
            });
        }

        Ok(timers)
    }

    /// Parse output from systemctl show
    fn parse_timer_info(&self, output: &str, name: &str) -> TimerResult<TimerInfo> {
        let mut id = String::new();
        let mut load_state = String::new();
        let mut unit_file_state = String::new();
        let mut next_elapse = None;
        let mut last_trigger = None;

        for line in output.lines() {
            if let Some(value) = line.strip_prefix("Id=") {
                id = value.to_string();
            } else if let Some(value) = line.strip_prefix("LoadState=") {
                load_state = value.to_string();
            } else if let Some(value) = line.strip_prefix("UnitFileState=") {
                unit_file_state = value.to_string();
            } else if let Some(value) = line.strip_prefix("NextElapseUSecRealtime=") {
                if value != "0" && !value.is_empty() {
                    next_elapse = Some(value.to_string());
                }
            } else if let Some(value) = line.strip_prefix("LastTriggerUSec=") {
                if value != "0" && !value.is_empty() {
                    last_trigger = Some(value.to_string());
                }
            }
        }

        if load_state == "not-found" {
            return Err(TimerError::NotFound(name.to_string()));
        }

        let enabled = unit_file_state == "enabled";
        let service = Self::timer_to_service(name).unwrap_or_else(|_| name.to_string());

        Ok(TimerInfo {
            name: id,
            enabled,
            schedule: "".to_string(), // Will be filled by schedule parser
            next_run: next_elapse,
            last_trigger,
            service,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::mock::MockCommandExecutor;
    use crate::command::CommandOutput;

    #[tokio::test]
    async fn test_validate_timer_name_valid() {
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo.timer").is_ok());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo.service").is_ok());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("chfscraper-scrape-bcp.timer").is_ok());
    }

    #[tokio::test]
    async fn test_validate_timer_name_invalid() {
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo/bar.timer").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo;bar.timer").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo|bar.timer").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo`bar`.timer").is_err());
        assert!(SystemctlClient::<MockCommandExecutor>::validate_timer_name("foo$bar.timer").is_err());
    }

    #[tokio::test]
    async fn test_timer_to_service() {
        assert_eq!(
            SystemctlClient::<MockCommandExecutor>::timer_to_service("foo.timer").unwrap(),
            "foo.service"
        );
        assert_eq!(
            SystemctlClient::<MockCommandExecutor>::timer_to_service("chfscraper-scrape-bcp.timer").unwrap(),
            "chfscraper-scrape-bcp.service"
        );
        assert!(SystemctlClient::<MockCommandExecutor>::timer_to_service("foo.service").is_err());
    }

    #[tokio::test]
    async fn test_list_timers_success() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: "NEXT                         LEFT       LAST                         PASSED  UNIT                              ACTIVATES\n\
                     Wed 2026-01-15 14:00:00 CET  45min left n/a                          n/a     chfscraper-scrape-bcp.timer       chfscraper-scrape-bcp.service\n\
                     Wed 2026-01-15 13:30:00 CET  15min left Wed 2026-01-15 12:30:00 CET  45min   chfscraper-scrape-scc.timer       chfscraper-scrape-scc.service\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl list-timers --all --no-pager --plain", output);

        let client = SystemctlClient::new(mock);
        let timers = client.list_timers().await.unwrap();

        assert_eq!(timers.len(), 2);
        assert_eq!(timers[0].name, "chfscraper-scrape-bcp.timer");
        assert_eq!(timers[0].service, "chfscraper-scrape-bcp.service");
        assert!(timers[0].next_run.is_some());
        assert_eq!(timers[1].name, "chfscraper-scrape-scc.timer");
        assert!(timers[1].next_run.is_some());
    }

    #[tokio::test]
    async fn test_list_timers_empty() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: "NEXT LEFT LAST PASSED UNIT ACTIVATES\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl list-timers --all --no-pager --plain", output);

        let client = SystemctlClient::new(mock);
        let timers = client.list_timers().await.unwrap();
        assert_eq!(timers.len(), 0);
    }

    #[tokio::test]
    async fn test_list_timers_command_failed() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: "Permission denied".to_string(),
            exit_code: 1,
        };
        mock.expect("systemctl list-timers --all --no-pager --plain", output);

        let client = SystemctlClient::new(mock);
        let result = client.list_timers().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_timer_info_enabled() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: "Id=test.timer\nLoadState=loaded\nUnitFileState=enabled\nNextElapseUSecRealtime=1705324800000000\nLastTriggerUSec=1705323000000000\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect(
            "systemctl show test.timer --property=Id,LoadState,UnitFileState,NextElapseUSecRealtime,LastTriggerUSec",
            output
        );

        let client = SystemctlClient::new(mock);
        let info = client.get_timer_info("test.timer").await.unwrap();
        assert_eq!(info.name, "test.timer");
        assert!(info.enabled);
        assert!(info.next_run.is_some());
        assert!(info.last_trigger.is_some());
    }

    #[tokio::test]
    async fn test_get_timer_info_disabled() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: "Id=test.timer\nLoadState=loaded\nUnitFileState=disabled\nNextElapseUSecRealtime=0\nLastTriggerUSec=0\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect(
            "systemctl show test.timer --property=Id,LoadState,UnitFileState,NextElapseUSecRealtime,LastTriggerUSec",
            output
        );

        let client = SystemctlClient::new(mock);
        let info = client.get_timer_info("test.timer").await.unwrap();
        assert!(!info.enabled);
        assert!(info.next_run.is_none());
    }

    #[tokio::test]
    async fn test_get_timer_info_not_found() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: "LoadState=not-found\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect(
            "systemctl show missing.timer --property=Id,LoadState,UnitFileState,NextElapseUSecRealtime,LastTriggerUSec",
            output
        );

        let client = SystemctlClient::new(mock);
        let result = client.get_timer_info("missing.timer").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TimerError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_run_timer_production() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl start test.service", output);

        let client = SystemctlClient::new(mock);
        let result = client.run_timer("test.timer", false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_timer_test_mode() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl start test.service", output);

        let client = SystemctlClient::new(mock);
        let result = client.run_timer("test.timer", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_timer_failed() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: "Service not found".to_string(),
            exit_code: 5,
        };
        mock.expect("systemctl start test.service", output);

        let client = SystemctlClient::new(mock);
        let result = client.run_timer("test.timer", false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_enable_timer() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl enable test.timer", output);

        let client = SystemctlClient::new(mock);
        let result = client.enable_timer("test.timer").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disable_timer() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("systemctl disable test.timer", output);

        let client = SystemctlClient::new(mock);
        let result = client.disable_timer("test.timer").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_timer_permission_denied() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: String::new(),
            stderr: "Permission denied".to_string(),
            exit_code: 1,
        };
        mock.expect("systemctl enable test.timer", output);

        let client = SystemctlClient::new(mock);
        let result = client.enable_timer("test.timer").await;
        assert!(result.is_err());
    }
}
