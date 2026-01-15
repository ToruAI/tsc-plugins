use crate::error::{Result, ServiceError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Command execution output
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Trait for executing system commands (mockable for tests)
#[async_trait]
pub trait CommandExecutor: Send + Sync {
    async fn execute(&self, cmd: &str, args: &[&str]) -> Result<CommandOutput>;
}

/// Production command executor that runs real system commands
pub struct SystemCommandExecutor {
    timeout_secs: u64,
}

impl SystemCommandExecutor {
    pub fn new() -> Self {
        Self { timeout_secs: 10 }
    }

    pub fn with_timeout(timeout_secs: u64) -> Self {
        Self { timeout_secs }
    }
}

impl Default for SystemCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandExecutor for SystemCommandExecutor {
    async fn execute(&self, cmd: &str, args: &[&str]) -> Result<CommandOutput> {
        let cmd_string = format!("{} {}", cmd, args.join(" "));

        let child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ServiceError::IoError(format!("Failed to spawn command '{}': {}", cmd_string, e)))?;

        let output = timeout(Duration::from_secs(self.timeout_secs), child.wait_with_output())
            .await
            .map_err(|_| ServiceError::Timeout(format!("Command '{}' timed out after {}s", cmd_string, self.timeout_secs)))?
            .map_err(|e| ServiceError::IoError(format!("Failed to wait for command '{}': {}", cmd_string, e)))?;

        Ok(CommandOutput {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

/// Mock command executor for tests
pub struct MockCommandExecutor {
    responses: HashMap<String, CommandOutput>,
}

impl MockCommandExecutor {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }

    /// Adds a mock response for a specific command
    pub fn with_response(mut self, cmd: &str, args: &[&str], output: CommandOutput) -> Self {
        let key = format!("{} {}", cmd, args.join(" "));
        self.responses.insert(key, output);
        self
    }

    /// Adds a mock response with just stdout
    pub fn with_stdout(self, cmd: &str, args: &[&str], stdout: &str) -> Self {
        self.with_response(cmd, args, CommandOutput {
            exit_code: 0,
            stdout: stdout.to_string(),
            stderr: String::new(),
        })
    }

    /// Adds a mock response for an error
    pub fn with_error(self, cmd: &str, args: &[&str], exit_code: i32, stderr: &str) -> Self {
        self.with_response(cmd, args, CommandOutput {
            exit_code,
            stdout: String::new(),
            stderr: stderr.to_string(),
        })
    }
}

impl Default for MockCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandExecutor for MockCommandExecutor {
    async fn execute(&self, cmd: &str, args: &[&str]) -> Result<CommandOutput> {
        let key = format!("{} {}", cmd, args.join(" "));

        self.responses
            .get(&key)
            .cloned()
            .ok_or_else(|| ServiceError::Other(format!("No mock response for command: {}", key)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_executor_with_stdout() {
        let executor = MockCommandExecutor::new()
            .with_stdout("echo", &["hello"], "hello\n");

        let output = executor.execute("echo", &["hello"]).await.unwrap();
        assert_eq!(output.exit_code, 0);
        assert_eq!(output.stdout, "hello\n");
        assert_eq!(output.stderr, "");
    }

    #[tokio::test]
    async fn test_mock_executor_with_error() {
        let executor = MockCommandExecutor::new()
            .with_error("systemctl", &["start", "nginx"], 5, "Unit not found");

        let output = executor.execute("systemctl", &["start", "nginx"]).await.unwrap();
        assert_eq!(output.exit_code, 5);
        assert_eq!(output.stderr, "Unit not found");
    }

    #[tokio::test]
    async fn test_mock_executor_missing_command() {
        let executor = MockCommandExecutor::new();

        let result = executor.execute("unknown", &["command"]).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ServiceError::Other(_)));
    }

    #[tokio::test]
    async fn test_system_executor_basic() {
        let executor = SystemCommandExecutor::new();

        // Test with a safe command that exists on all systems
        let output = executor.execute("echo", &["test"]).await.unwrap();
        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("test"));
    }

    #[tokio::test]
    async fn test_system_executor_command_not_found() {
        let executor = SystemCommandExecutor::new();

        let result = executor.execute("nonexistent_command_xyz", &[]).await;
        assert!(result.is_err());
        // Should be IoError because command doesn't exist
        assert!(matches!(result.unwrap_err(), ServiceError::IoError(_)));
    }
}
