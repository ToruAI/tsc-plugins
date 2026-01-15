use crate::command::CommandExecutor;
use crate::error::{TimerError, TimerResult};
use crate::journal::{ExecutionDetails, ExecutionHistory, ExecutionStatus, TriggerType};

/// Log directory base path
const LOG_BASE_DIR: &str = "/var/log/timers";

/// Log reader for file-based execution logs
pub struct LogReader<E: CommandExecutor> {
    executor: E,
}

impl<E: CommandExecutor> LogReader<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Get execution history from log files
    pub async fn get_execution_history(
        &self,
        service_name: &str,
        limit: usize,
    ) -> TimerResult<Vec<ExecutionHistory>> {
        let base_name = service_name.trim_end_matches(".service");
        let log_dir = format!("{}/{}", LOG_BASE_DIR, base_name);

        // List log files (excluding latest.log symlink)
        let output = self.executor
            .execute("ls", &["-1t", &log_dir])
            .await?;

        if output.exit_code != 0 {
            // Directory might not exist yet
            return Ok(Vec::new());
        }

        let mut history = Vec::new();
        let files: Vec<&str> = output.stdout
            .lines()
            .filter(|f| f.ends_with(".log") && *f != "latest.log")
            .take(limit)
            .collect();

        for filename in files {
            let log_path = format!("{}/{}", log_dir, filename);
            if let Ok(entry) = self.parse_log_file(&log_path, filename).await {
                history.push(entry);
            }
        }

        Ok(history)
    }

    /// Get detailed execution info including full output
    pub async fn get_execution_details(
        &self,
        service_name: &str,
        timestamp: &str,
    ) -> TimerResult<ExecutionDetails> {
        let base_name = service_name.trim_end_matches(".service");
        let log_path = format!("{}/{}/{}.log", LOG_BASE_DIR, base_name, timestamp);

        let output = self.executor
            .execute("cat", &[&log_path])
            .await?;

        if output.exit_code != 0 {
            return Err(TimerError::NotFound(format!("Log file not found: {}", log_path)));
        }

        self.parse_log_file_details(&output.stdout, timestamp)
    }

    /// Parse a log file to extract execution history entry
    async fn parse_log_file(
        &self,
        log_path: &str,
        filename: &str,
    ) -> TimerResult<ExecutionHistory> {
        // Read first and last lines to get metadata
        let tail_output = self.executor
            .execute("tail", &["-n", "1", log_path])
            .await?;

        let last_line = tail_output.stdout.trim();

        // Extract timestamp from filename (YYYY-MM-DD_HHMMSS.log)
        let timestamp = filename.trim_end_matches(".log");
        let start_time = Self::filename_to_datetime(timestamp);

        // Parse [END] line for metadata
        let (end_time, exit_code, duration_secs, status) = self.parse_end_line(last_line);

        // Determine if still running (no [END] line)
        let status = if last_line.starts_with("[END]") {
            status
        } else {
            ExecutionStatus::Running
        };

        Ok(ExecutionHistory {
            invocation_id: timestamp.to_string(),
            start_time,
            end_time,
            duration_secs,
            status,
            exit_code,
            trigger: TriggerType::Scheduled, // Default, could be enhanced later
        })
    }

    /// Parse log file for detailed output
    fn parse_log_file_details(
        &self,
        content: &str,
        timestamp: &str,
    ) -> TimerResult<ExecutionDetails> {
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return Err(TimerError::ParseError {
                source: "log_file".to_string(),
                reason: "Empty log file".to_string(),
            });
        }

        let last_line = lines.last().unwrap_or(&"");

        let start_time = Self::filename_to_datetime(timestamp);
        let (end_time, exit_code, duration_secs, status) = self.parse_end_line(last_line);

        // Get output lines (everything except [START] and [END] lines)
        let output: Vec<String> = lines.iter()
            .filter(|l| !l.starts_with("[START]") && !l.starts_with("[END]"))
            .map(|s| s.to_string())
            .collect();

        let status = if last_line.starts_with("[END]") {
            status
        } else {
            ExecutionStatus::Running
        };

        Ok(ExecutionDetails {
            invocation_id: timestamp.to_string(),
            start_time,
            end_time,
            duration_secs,
            status,
            exit_code,
            trigger: TriggerType::Scheduled,
            output,
        })
    }

    /// Parse [END] line to extract metadata
    /// Format: [END] {timestamp} exit_code={N} duration={S}s
    fn parse_end_line(&self, line: &str) -> (Option<String>, Option<i32>, Option<u64>, ExecutionStatus) {
        if !line.starts_with("[END]") {
            return (None, None, None, ExecutionStatus::Running);
        }

        let exit_code = Self::extract_value(line, "exit_code=")
            .and_then(|s| s.parse::<i32>().ok());

        let duration_secs = Self::extract_value(line, "duration=")
            .and_then(|s| s.trim_end_matches('s').parse::<u64>().ok());

        // Extract end timestamp (second part after [END])
        let end_time = line
            .strip_prefix("[END] ")
            .and_then(|s| s.split_whitespace().next())
            .map(|s| s.to_string());

        let status = match exit_code {
            Some(0) => ExecutionStatus::Success,
            Some(_) => ExecutionStatus::Failed,
            None => ExecutionStatus::Success, // No exit code = assume success
        };

        (end_time, exit_code, duration_secs, status)
    }

    /// Extract value after a key= pattern
    fn extract_value(line: &str, key: &str) -> Option<String> {
        line.find(key).map(|pos| {
            let start = pos + key.len();
            let rest = &line[start..];
            rest.split_whitespace()
                .next()
                .unwrap_or("")
                .to_string()
        })
    }

    /// Convert filename timestamp to datetime string
    /// Input: YYYY-MM-DD_HHMMSS
    /// Output: YYYY-MM-DD HH:MM:SS
    fn filename_to_datetime(timestamp: &str) -> String {
        if timestamp.len() >= 17 {
            // YYYY-MM-DD_HHMMSS -> YYYY-MM-DD HH:MM:SS
            let date = &timestamp[0..10];
            let time = &timestamp[11..17];
            if time.len() == 6 {
                return format!("{} {}:{}:{}",
                    date,
                    &time[0..2],
                    &time[2..4],
                    &time[4..6]
                );
            }
        }
        timestamp.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_to_datetime() {
        assert_eq!(
            LogReader::<crate::command::SystemCommandExecutor>::filename_to_datetime("2026-01-15_140000"),
            "2026-01-15 14:00:00"
        );
        assert_eq!(
            LogReader::<crate::command::SystemCommandExecutor>::filename_to_datetime("2026-01-15_093045"),
            "2026-01-15 09:30:45"
        );
    }

    #[test]
    fn test_extract_value() {
        let reader = LogReader { executor: crate::command::SystemCommandExecutor };

        let line = "[END] 2026-01-15T14:00:45+01:00 exit_code=0 duration=45s";
        assert_eq!(LogReader::<crate::command::SystemCommandExecutor>::extract_value(line, "exit_code="), Some("0".to_string()));
        assert_eq!(LogReader::<crate::command::SystemCommandExecutor>::extract_value(line, "duration="), Some("45s".to_string()));
    }

    #[test]
    fn test_parse_end_line() {
        let reader = LogReader { executor: crate::command::SystemCommandExecutor };

        let line = "[END] 2026-01-15T14:00:45+01:00 exit_code=0 duration=45s";
        let (end_time, exit_code, duration, status) = reader.parse_end_line(line);

        assert_eq!(end_time, Some("2026-01-15T14:00:45+01:00".to_string()));
        assert_eq!(exit_code, Some(0));
        assert_eq!(duration, Some(45));
        assert_eq!(status, ExecutionStatus::Success);
    }

    #[test]
    fn test_parse_end_line_failed() {
        let reader = LogReader { executor: crate::command::SystemCommandExecutor };

        let line = "[END] 2026-01-15T14:02:00+01:00 exit_code=1 duration=120s";
        let (end_time, exit_code, duration, status) = reader.parse_end_line(line);

        assert_eq!(exit_code, Some(1));
        assert_eq!(duration, Some(120));
        assert_eq!(status, ExecutionStatus::Failed);
    }

    #[test]
    fn test_parse_end_line_no_end() {
        let reader = LogReader { executor: crate::command::SystemCommandExecutor };

        let line = "Some random log line";
        let (end_time, exit_code, duration, status) = reader.parse_end_line(line);

        assert_eq!(end_time, None);
        assert_eq!(exit_code, None);
        assert_eq!(duration, None);
        assert_eq!(status, ExecutionStatus::Running);
    }
}
