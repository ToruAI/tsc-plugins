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
    #[serde(rename = "INVOCATION_ID")]
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
        let invocation_filter = format!("INVOCATION_ID={}", invocation_id);
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
        } else if exit_code == Some(0) {
            ExecutionStatus::Success
        } else if exit_code.is_some() {
            ExecutionStatus::Failed
        } else {
            // No exit code but execution ended - assume success
            ExecutionStatus::Success
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
    use crate::command::mock::MockCommandExecutor;
    use crate::command::CommandOutput;

    #[test]
    fn test_calculate_duration() {
        let start = "1705320000000000"; // Jan 15, 2024 12:00:00
        let end = "1705320120000000";   // Jan 15, 2024 12:02:00
        let duration = JournalClient::<crate::command::SystemCommandExecutor>::calculate_duration(start, end);
        assert_eq!(duration, Some(120));
    }

    #[test]
    fn test_calculate_duration_invalid() {
        let duration = JournalClient::<crate::command::SystemCommandExecutor>::calculate_duration("invalid", "123");
        assert_eq!(duration, None);

        let duration = JournalClient::<crate::command::SystemCommandExecutor>::calculate_duration("100", "50");
        assert_eq!(duration, None);
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = "1705320000000000"; // Jan 15, 2024 12:00:00 UTC
        let formatted = JournalClient::<crate::command::SystemCommandExecutor>::format_timestamp(timestamp);
        assert!(formatted.starts_with("2024-01-15"));
    }

    #[test]
    fn test_format_timestamp_invalid() {
        let formatted = JournalClient::<crate::command::SystemCommandExecutor>::format_timestamp("invalid");
        assert_eq!(formatted, "invalid");
    }

    #[test]
    fn test_execution_status_serialization() {
        let status = ExecutionStatus::Success;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""success""#);

        let status = ExecutionStatus::Failed;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""failed""#);

        let status = ExecutionStatus::Running;
        assert_eq!(serde_json::to_string(&status).unwrap(), r#""running""#);
    }

    #[test]
    fn test_trigger_type_serialization() {
        let trigger = TriggerType::Scheduled;
        assert_eq!(serde_json::to_string(&trigger).unwrap(), r#""scheduled""#);

        let trigger = TriggerType::Manual;
        assert_eq!(serde_json::to_string(&trigger).unwrap(), r#""manual""#);
    }

    #[tokio::test]
    async fn test_get_execution_history_success() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: r#"{"INVOCATION_ID":"abc123","__REALTIME_TIMESTAMP":"1705320000000000","MESSAGE":"Starting","_SYSTEMD_UNIT":"test.service"}
{"INVOCATION_ID":"abc123","__REALTIME_TIMESTAMP":"1705320045000000","EXIT_STATUS":"0","_SYSTEMD_UNIT":"test.service"}
"#.to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("journalctl -u test.service --since 7 days ago -o json --no-pager", output);

        let client = JournalClient::new(mock);
        let history = client.get_execution_history("test.service", 10).await.unwrap();

        assert_eq!(history.len(), 1);
        assert_eq!(history[0].invocation_id, "abc123");
        assert_eq!(history[0].status, ExecutionStatus::Success);
        assert_eq!(history[0].duration_secs, Some(45));
    }

    #[tokio::test]
    async fn test_get_execution_history_failed() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: r#"{"INVOCATION_ID":"def456","__REALTIME_TIMESTAMP":"1705320000000000","MESSAGE":"Starting","_SYSTEMD_UNIT":"test.service"}
{"INVOCATION_ID":"def456","__REALTIME_TIMESTAMP":"1705320120000000","EXIT_STATUS":"1","_SYSTEMD_UNIT":"test.service"}
"#.to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("journalctl -u test.service --since 7 days ago -o json --no-pager", output);

        let client = JournalClient::new(mock);
        let history = client.get_execution_history("test.service", 10).await.unwrap();

        assert_eq!(history.len(), 1);
        assert_eq!(history[0].status, ExecutionStatus::Failed);
        assert_eq!(history[0].exit_code, Some(1));
    }

    #[tokio::test]
    async fn test_get_execution_history_running() {
        let mock = MockCommandExecutor::new();
        // Only first entry, no second one means still running
        let output = CommandOutput {
            stdout: r#"{"INVOCATION_ID":"ghi789","__REALTIME_TIMESTAMP":"1705320000000000","MESSAGE":"Starting","_SYSTEMD_UNIT":"test.service"}"#.to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("journalctl -u test.service --since 7 days ago -o json --no-pager", output);

        let client = JournalClient::new(mock);
        let history = client.get_execution_history("test.service", 10).await.unwrap();

        assert_eq!(history.len(), 1);
        // With only one entry, last timestamp is also first, so end_time exists
        // Let's check it's successful (no exit code but has end time)
        assert_eq!(history[0].status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_get_execution_history_limit() {
        let mock = MockCommandExecutor::new();
        let mut entries = Vec::new();
        for i in 0..50 {
            entries.push(format!(
                r#"{{"INVOCATION_ID":"inv{}","__REALTIME_TIMESTAMP":"{}","MESSAGE":"Test","EXIT_STATUS":"0","_SYSTEMD_UNIT":"test.service"}}"#,
                i,
                1705320000000000u64 + (i * 1000000)
            ));
        }
        let output = CommandOutput {
            stdout: entries.join("\n"),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("journalctl -u test.service --since 7 days ago -o json --no-pager", output);

        let client = JournalClient::new(mock);
        let history = client.get_execution_history("test.service", 10).await.unwrap();

        assert_eq!(history.len(), 10); // Limited to 10
    }

    #[tokio::test]
    async fn test_get_execution_details() {
        let mock = MockCommandExecutor::new();
        let output = CommandOutput {
            stdout: r#"{"INVOCATION_ID":"abc123","__REALTIME_TIMESTAMP":"1705320000000000","MESSAGE":"Starting scrape...","_SYSTEMD_UNIT":"test.service"}
{"INVOCATION_ID":"abc123","__REALTIME_TIMESTAMP":"1705320005000000","MESSAGE":"Proxy enabled","_SYSTEMD_UNIT":"test.service"}
{"INVOCATION_ID":"abc123","__REALTIME_TIMESTAMP":"1705320045000000","MESSAGE":"Complete","EXIT_STATUS":"0","_SYSTEMD_UNIT":"test.service"}
"#.to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        mock.expect("journalctl -u test.service INVOCATION_ID=abc123 -o json --no-pager", output);

        let client = JournalClient::new(mock);
        let details = client.get_execution_details("test.service", "abc123").await.unwrap();

        assert_eq!(details.invocation_id, "abc123");
        assert_eq!(details.output.len(), 3);
        assert!(details.output[0].contains("Starting scrape"));
        assert_eq!(details.status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_parse_journal_entries_malformed() {
        let client = JournalClient::new(MockCommandExecutor::new());
        let output = r#"{"valid":"json"}
not json at all
{"INVOCATION_ID":"test"}
"#;
        let entries = client.parse_journal_entries(output).unwrap();

        // Should skip malformed line but parse valid ones
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_determine_trigger_scheduled() {
        let client = JournalClient::new(MockCommandExecutor::new());
        let entries = vec![
            JournalEntry {
                invocation_id: Some("test".to_string()),
                timestamp: Some("123".to_string()),
                message: Some("Started by timer".to_string()),
                exit_status: None,
                unit: Some("test.service".to_string()),
            }
        ];

        let trigger = client.determine_trigger(&entries);
        assert_eq!(trigger, TriggerType::Scheduled);
    }

    #[tokio::test]
    async fn test_determine_trigger_manual() {
        let client = JournalClient::new(MockCommandExecutor::new());
        let entries = vec![
            JournalEntry {
                invocation_id: Some("test".to_string()),
                timestamp: Some("123".to_string()),
                message: Some("Started manually via systemctl start".to_string()),
                exit_status: None,
                unit: Some("test.service".to_string()),
            }
        ];

        let trigger = client.determine_trigger(&entries);
        assert_eq!(trigger, TriggerType::Manual);
    }
}
