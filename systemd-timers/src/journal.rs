use crate::command::CommandExecutor;
use crate::error::{TimerError, TimerResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Success,
    Failed,
    Running,
}

/// Trigger type for an execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    Scheduled,
    Manual,
}

/// Execution history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHistory {
    pub invocation_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_secs: Option<u64>,
    pub status: ExecutionStatus,
    pub exit_code: Option<i32>,
    pub trigger: TriggerType,
}

/// Full execution details including output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDetails {
    pub invocation_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_secs: Option<u64>,
    pub status: ExecutionStatus,
    pub exit_code: Option<i32>,
    pub trigger: TriggerType,
    pub output: Vec<String>,
}

/// Journal entry from journalctl JSON output
#[derive(Debug, Clone, Deserialize)]
struct JournalEntry {
    #[serde(rename = "_SYSTEMD_INVOCATION_ID")]
    invocation_id: Option<String>,

    #[serde(rename = "__REALTIME_TIMESTAMP")]
    timestamp: Option<String>,

    #[serde(rename = "MESSAGE")]
    message: Option<String>,

    #[serde(rename = "EXIT_STATUS")]
    exit_status: Option<String>,

    #[serde(rename = "_SYSTEMD_UNIT")]
    unit: Option<String>,
}

/// Journal client for querying execution history
pub struct JournalClient<E: CommandExecutor> {
    executor: E,
}

impl<E: CommandExecutor> JournalClient<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Get execution history for a service
    pub async fn get_execution_history(
        &self,
        service: &str,
        limit: usize,
    ) -> TimerResult<Vec<ExecutionHistory>> {
        let output = self.executor
            .execute("journalctl", &[
                "-u", service,
                "--since", "7 days ago",
                "-o", "json",
                "--no-pager",
            ])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("journalctl -u {}", service),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        let entries = self.parse_journal_entries(&output.stdout)?;
        let history = self.group_by_invocation(entries, limit)?;

        Ok(history)
    }

    /// Get detailed execution information including output
    pub async fn get_execution_details(
        &self,
        service: &str,
        invocation_id: &str,
    ) -> TimerResult<ExecutionDetails> {
        let invocation_filter = format!("_SYSTEMD_INVOCATION_ID={}", invocation_id);
        let output = self.executor
            .execute("journalctl", &[
                "-u", service,
                &invocation_filter,
                "-o", "json",
                "--no-pager",
            ])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::CommandFailed {
                command: format!("journalctl -u {} invocation {}", service, invocation_id),
                stderr: output.stderr,
                exit_code: Some(output.exit_code),
            });
        }

        let entries = self.parse_journal_entries(&output.stdout)?;
        self.create_execution_details(invocation_id, entries)
    }

    /// Parse journalctl JSON output
    fn parse_journal_entries(&self, output: &str) -> TimerResult<Vec<JournalEntry>> {
        let mut entries = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<JournalEntry>(line) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    // Skip malformed lines (can happen with binary data)
                    eprintln!("Warning: Failed to parse journal line: {}", e);
                    continue;
                }
            }
        }

        Ok(entries)
    }

    /// Group entries by invocation ID
    fn group_by_invocation(
        &self,
        entries: Vec<JournalEntry>,
        limit: usize,
    ) -> TimerResult<Vec<ExecutionHistory>> {
        let mut invocations: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            if let Some(id) = &entry.invocation_id {
                invocations.entry(id.clone()).or_default().push(entry);
            }
        }

        let mut history: Vec<ExecutionHistory> = invocations
            .into_iter()
            .map(|(id, entries)| self.create_execution_history(&id, entries))
            .collect::<TimerResult<Vec<_>>>()?;

        // Sort by start time (newest first)
        history.sort_by(|a, b| b.start_time.cmp(&a.start_time));

        // Limit results
        history.truncate(limit);

        Ok(history)
    }

    /// Create execution history from grouped entries
    fn create_execution_history(
        &self,
        invocation_id: &str,
        entries: Vec<JournalEntry>,
    ) -> TimerResult<ExecutionHistory> {
        let start_time = entries
            .first()
            .and_then(|e| e.timestamp.clone())
            .ok_or_else(|| TimerError::ParseError {
                source: "journal".to_string(),
                reason: format!("No timestamp for invocation {}", invocation_id),
            })?;

        let end_time = entries.last().and_then(|e| e.timestamp.clone());

        let duration_secs = if let Some(end) = &end_time {
            Self::calculate_duration(&start_time, end)
        } else {
            None
        };

        let exit_code = entries
            .iter()
            .rev()
            .find_map(|e| e.exit_status.as_ref())
            .and_then(|s| s.parse::<i32>().ok());

        let status = if end_time.is_none() {
            ExecutionStatus::Running
        } else if exit_code == Some(0) || exit_code.is_none() {
            ExecutionStatus::Success
        } else {
            ExecutionStatus::Failed
        };

        let trigger = self.determine_trigger(&entries);

        Ok(ExecutionHistory {
            invocation_id: invocation_id.to_string(),
            start_time: Self::format_timestamp(&start_time),
            end_time: end_time.as_ref().map(|t| Self::format_timestamp(t)),
            duration_secs,
            status,
            exit_code,
            trigger,
        })
    }

    /// Create execution details with full output
    fn create_execution_details(
        &self,
        invocation_id: &str,
        entries: Vec<JournalEntry>,
    ) -> TimerResult<ExecutionDetails> {
        let history = self.create_execution_history(invocation_id, entries.clone())?;

        let output: Vec<String> = entries
            .iter()
            .filter_map(|e| e.message.clone())
            .collect();

        Ok(ExecutionDetails {
            invocation_id: history.invocation_id,
            start_time: history.start_time,
            end_time: history.end_time,
            duration_secs: history.duration_secs,
            status: history.status,
            exit_code: history.exit_code,
            trigger: history.trigger,
            output,
        })
    }

    /// Calculate duration between timestamps (in microseconds)
    fn calculate_duration(start: &str, end: &str) -> Option<u64> {
        let start_us: u64 = start.parse().ok()?;
        let end_us: u64 = end.parse().ok()?;

        if end_us > start_us {
            Some((end_us - start_us) / 1_000_000) // Convert to seconds
        } else {
            None
        }
    }

    /// Format timestamp from microseconds since epoch
    fn format_timestamp(timestamp: &str) -> String {
        if let Ok(us) = timestamp.parse::<i64>() {
            let secs = us / 1_000_000;
            if let Some(dt) = chrono::DateTime::from_timestamp(secs, 0) {
                return dt.format("%Y-%m-%d %H:%M:%S").to_string();
            }
        }
        timestamp.to_string()
    }

    /// Determine if execution was triggered by timer or manually
    fn determine_trigger(&self, entries: &[JournalEntry]) -> TriggerType {
        for entry in entries {
            if let Some(msg) = &entry.message {
                if msg.contains("timer") || msg.contains("scheduled") {
                    return TriggerType::Scheduled;
                }
                if msg.contains("manual") || msg.contains("systemctl start") {
                    return TriggerType::Manual;
                }
            }
        }

        // Default to scheduled (most common case)
        TriggerType::Scheduled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_duration() {
        let start = "1705320000000000"; // Jan 15, 2024 12:00:00
        let end = "1705320120000000";   // Jan 15, 2024 12:02:00
        let duration = JournalClient::<crate::command::SystemCommandExecutor>::calculate_duration(start, end);
        assert_eq!(duration, Some(120));
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = "1705320000000000"; // Jan 15, 2024 12:00:00 UTC
        let formatted = JournalClient::<crate::command::SystemCommandExecutor>::format_timestamp(timestamp);
        assert!(formatted.starts_with("2024-01-15"));
    }

    #[test]
    fn test_execution_status() {
        let status = ExecutionStatus::Success;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""success""#);

        let status = ExecutionStatus::Failed;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""failed""#);
    }
}
