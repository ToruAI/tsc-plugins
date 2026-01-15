use crate::error::TimerResult;
use async_trait::async_trait;
use std::sync::Arc;

/// Output from a command execution
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Trait for executing system commands (allows mocking in tests)
#[async_trait]
pub trait CommandExecutor: Send + Sync {
    /// Execute a command with arguments
    async fn execute(&self, program: &str, args: &[&str]) -> TimerResult<CommandOutput>;
}

/// Blanket implementation for Arc<E> where E: CommandExecutor
#[async_trait]
impl<E: CommandExecutor> CommandExecutor for Arc<E> {
    async fn execute(&self, program: &str, args: &[&str]) -> TimerResult<CommandOutput> {
        self.as_ref().execute(program, args).await
    }
}

/// Production command executor using std::process::Command
pub struct SystemCommandExecutor;

#[async_trait]
impl CommandExecutor for SystemCommandExecutor {
    async fn execute(&self, program: &str, args: &[&str]) -> TimerResult<CommandOutput> {
        use tokio::process::Command;

        let output = Command::new(program)
            .args(args)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok(CommandOutput {
            stdout,
            stderr,
            exit_code,
        })
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    /// Mock command executor for tests
    pub struct MockCommandExecutor {
        responses: Arc<Mutex<HashMap<String, CommandOutput>>>,
    }

    impl MockCommandExecutor {
        pub fn new() -> Self {
            Self {
                responses: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Set expected response for a command
        pub fn expect(&self, command_key: &str, output: CommandOutput) {
            let mut responses = self.responses.lock().unwrap();
            responses.insert(command_key.to_string(), output);
        }

        fn make_key(program: &str, args: &[&str]) -> String {
            format!("{} {}", program, args.join(" "))
        }
    }

    #[async_trait]
    impl CommandExecutor for MockCommandExecutor {
        async fn execute(&self, program: &str, args: &[&str]) -> TimerResult<CommandOutput> {
            let key = Self::make_key(program, args);
            let responses = self.responses.lock().unwrap();

            responses.get(&key)
                .cloned()
                .ok_or_else(|| crate::error::TimerError::CommandFailed {
                    command: key.clone(),
                    stderr: format!("No mock response configured for: {}", key),
                    exit_code: Some(-1),
                })
        }
    }
}
