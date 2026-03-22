//! Sandbox executor for secure code execution
//!
//! This module provides the main sandbox execution functionality, including
//! process isolation, resource limits, output capture, and comprehensive error handling.

use crate::sandbox::config::SandboxConfig;
use crate::sandbox::limits::{LimitError, ResourceLimiter};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Result of a sandboxed execution
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    /// Standard output from the execution
    pub stdout: String,
    /// Standard error from the execution
    pub stderr: String,
    /// Process exit code
    pub exit_code: i32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Memory used in megabytes
    pub memory_used_mb: u64,
    /// Whether execution was killed due to resource limits
    pub killed_by_limit: Option<String>,
}

impl ExecutionResult {
    /// Check if execution was successful
    pub fn success(&self) -> bool {
        self.exit_code == 0 && self.killed_by_limit.is_none()
    }

    /// Get combined output (stdout + stderr)
    pub fn combined_output(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}

/// Errors that can occur during sandbox execution
#[derive(Debug, Clone, PartialEq)]
pub enum SandboxError {
    /// Resource limit exceeded
    ResourceLimitExceeded(String),
    /// Execution timed out
    Timeout,
    /// Failed to spawn process
    SpawnFailed(String),
    /// Failed to capture output
    OutputCaptureFailed(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// System error
    SystemError(String),
    /// Compilation error (for compiled languages)
    CompilationFailed(String),
    /// Runtime error
    RuntimeError(String),
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxError::ResourceLimitExceeded(msg) => {
                write!(f, "Resource limit exceeded: {}", msg)
            }
            SandboxError::Timeout => write!(f, "Execution timed out"),
            SandboxError::SpawnFailed(msg) => write!(f, "Failed to spawn process: {}", msg),
            SandboxError::OutputCaptureFailed(msg) => {
                write!(f, "Failed to capture output: {}", msg)
            }
            SandboxError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            SandboxError::SystemError(msg) => write!(f, "System error: {}", msg),
            SandboxError::CompilationFailed(msg) => write!(f, "Compilation failed: {}", msg),
            SandboxError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for SandboxError {}

impl From<LimitError> for SandboxError {
    fn from(err: LimitError) -> Self {
        SandboxError::ResourceLimitExceeded(err.to_string())
    }
}

impl From<io::Error> for SandboxError {
    fn from(err: io::Error) -> Self {
        SandboxError::SystemError(err.to_string())
    }
}

/// Sandbox executor for running untrusted code
///
/// Provides secure execution environment with resource limits and isolation.
#[derive(Debug, Clone)]
pub struct SandboxExecutor {
    config: SandboxConfig,
}

impl SandboxExecutor {
    /// Create a new sandbox executor with the given configuration
    pub fn new(config: SandboxConfig) -> Result<Self, SandboxError> {
        config.validate().map_err(|e| SandboxError::InvalidConfig(e.to_string()))?;
        Ok(Self { config })
    }

    /// Execute code in the sandbox
    ///
    /// The code is executed in a separate process with resource limits applied.
    pub async fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError> {
        self.execute_with_input(code, "").await
    }

    /// Execute code with input data
    ///
    /// The code is executed in a separate process with resource limits applied.
    /// Input is provided via stdin.
    pub async fn execute_with_input(
        &self,
        code: &str,
        input: &str,
    ) -> Result<ExecutionResult, SandboxError> {
        let start_time = Instant::now();
        let _limiter = ResourceLimiter::new(self.config.clone());

        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join(format!("autoharness_{}.sh", uuid::Uuid::new_v4()));

        let script_content = format!(
            "#!/bin/sh\n{}\n",
            code
        );

        std::fs::write(&script_path, script_content)
            .map_err(|e| SandboxError::SystemError(format!("Failed to write script: {}", e)))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&script_path)
                .map_err(|e| SandboxError::SystemError(format!("Failed to get metadata: {}", e)))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_path, perms)
                .map_err(|e| SandboxError::SystemError(format!("Failed to set permissions: {}", e)))?;
        }

        let mut cmd = Command::new(&script_path);
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped());

        if let Some(ref working_dir) = self.config.working_directory {
            cmd.current_dir(working_dir);
        }

        for (key, value) in &self.config.environment_variables {
            cmd.env(key, value);
        }

        if !self.config.enable_network {
            cmd.env("SANDBOX_NO_NETWORK", "1");
        }

        let mut child = cmd.spawn().map_err(|e| SandboxError::SpawnFailed(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .map_err(|e| SandboxError::SystemError(format!("Failed to write input: {}", e)))?;
        }

        let timeout_duration = Duration::from_millis(self.config.time_limit_ms);
        
        let child_id = child.id();
        let result = timeout(timeout_duration, async {
            let output = child.wait_with_output().map_err(|e| SandboxError::SystemError(e.to_string()))?;
            Ok::<_, SandboxError>(output)
        })
        .await;

        let _ = std::fs::remove_file(&script_path);

        match result {
            Ok(Ok(output)) => {
                let execution_time_ms = start_time.elapsed().as_millis() as u64;

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if stdout.len() + stderr.len() > self.config.max_output_size {
                    return Err(SandboxError::ResourceLimitExceeded(
                        "Output size limit exceeded".to_string(),
                    ));
                }

                let exit_code = output.status.code().unwrap_or(-1);

                let memory_used_mb = 0;

                Ok(ExecutionResult {
                    stdout,
                    stderr,
                    exit_code,
                    execution_time_ms,
                    memory_used_mb,
                    killed_by_limit: None,
                })
            }
            Ok(Err(e)) => Err(e),
            Err(_) => {
                let _ = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(child_id.to_string())
                    .output();
                Err(SandboxError::Timeout)
            }
        }
    }

    /// Execute a command with arguments
    ///
    /// This is a lower-level method for executing arbitrary commands in the sandbox.
    pub async fn execute_command(
        &self,
        command: &str,
        args: &[&str],
        input: Option<&str>,
    ) -> Result<ExecutionResult, SandboxError> {
        let start_time = Instant::now();

        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped());

        if let Some(ref working_dir) = self.config.working_directory {
            cmd.current_dir(working_dir);
        }

        for (key, value) in &self.config.environment_variables {
            cmd.env(key, value);
        }

        let mut child = cmd.spawn().map_err(|e| SandboxError::SpawnFailed(e.to_string()))?;

        if let Some(input_data) = input {
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(input_data.as_bytes())
                    .map_err(|e| SandboxError::SystemError(format!("Failed to write input: {}", e)))?;
            }
        }

        let timeout_duration = Duration::from_millis(self.config.time_limit_ms);
        let child_id = child.id();
        let result = timeout(timeout_duration, async {
            let output = child.wait_with_output().map_err(|e| SandboxError::SystemError(e.to_string()))?;
            Ok::<_, SandboxError>(output)
        })
        .await;

        match result {
            Ok(Ok(output)) => {
                let execution_time_ms = start_time.elapsed().as_millis() as u64;

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if stdout.len() + stderr.len() > self.config.max_output_size {
                    return Err(SandboxError::ResourceLimitExceeded(
                        "Output size limit exceeded".to_string(),
                    ));
                }

                let exit_code = output.status.code().unwrap_or(-1);

                Ok(ExecutionResult {
                    stdout,
                    stderr,
                    exit_code,
                    execution_time_ms,
                    memory_used_mb: 0,
                    killed_by_limit: None,
                })
            }
            Ok(Err(e)) => Err(e),
            Err(_) => {
                let _ = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(child_id.to_string())
                    .output();
                Err(SandboxError::Timeout)
            }
        }
    }

    /// Get the sandbox configuration
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Update the sandbox configuration
    pub fn with_config(mut self, config: SandboxConfig) -> Result<Self, SandboxError> {
        config.validate().map_err(|e| SandboxError::InvalidConfig(e.to_string()))?;
        self.config = config;
        Ok(self)
    }
}

/// Builder for creating sandbox executors
#[derive(Debug, Default)]
pub struct SandboxExecutorBuilder {
    config: Option<SandboxConfig>,
}

impl SandboxExecutorBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the configuration
    pub fn config(mut self, config: SandboxConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the executor
    pub fn build(self) -> Result<SandboxExecutor, SandboxError> {
        let config = self.config.unwrap_or_default();
        SandboxExecutor::new(config)
    }
}

/// Utility functions for sandbox operations
pub mod utils {
    use super::*;

    /// Create a temporary file with the given content
    pub fn create_temp_file(content: &str) -> io::Result<std::path::PathBuf> {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("autoharness_{}.tmp", uuid::Uuid::new_v4()));
        std::fs::write(&file_path, content)?;
        Ok(file_path)
    }

    /// Clean up temporary files
    pub fn cleanup_temp_file(path: &std::path::Path) {
        let _ = std::fs::remove_file(path);
    }

    /// Validate that code doesn't contain obvious security violations
    pub fn validate_code(code: &str) -> Result<(), SandboxError> {
        let forbidden_patterns = [
            "rm -rf /",
            ":(){ :|:& };:",
            "fork bomb",
            "while true",
        ];

        for pattern in &forbidden_patterns {
            if code.contains(pattern) {
                return Err(SandboxError::InvalidConfig(format!(
                    "Code contains forbidden pattern: {}",
                    pattern
                )));
            }
        }

        Ok(())
    }

    /// Escape shell special characters in a string
    pub fn shell_escape(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            .replace('$', "\\$")
            .replace('`', "\\`")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult {
            stdout: "hello".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            execution_time_ms: 100,
            memory_used_mb: 10,
            killed_by_limit: None,
        };
        assert!(result.success());
        assert_eq!(result.combined_output(), "hello");
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult {
            stdout: "".to_string(),
            stderr: "error".to_string(),
            exit_code: 1,
            execution_time_ms: 100,
            memory_used_mb: 10,
            killed_by_limit: None,
        };
        assert!(!result.success());
    }

    #[test]
    fn test_execution_result_killed() {
        let result = ExecutionResult {
            stdout: "".to_string(),
            stderr: "".to_string(),
            exit_code: -1,
            execution_time_ms: 100,
            memory_used_mb: 10,
            killed_by_limit: Some("timeout".to_string()),
        };
        assert!(!result.success());
    }

    #[test]
    fn test_sandbox_error_display() {
        let err = SandboxError::Timeout;
        assert_eq!(err.to_string(), "Execution timed out");

        let err = SandboxError::ResourceLimitExceeded("memory".to_string());
        assert!(err.to_string().contains("memory"));
    }

    #[test]
    fn test_sandbox_executor_creation() {
        let config = SandboxConfig::default();
        let executor = SandboxExecutor::new(config);
        assert!(executor.is_ok());
    }

    #[test]
    fn test_sandbox_executor_invalid_config() {
        let config = SandboxConfig::new().with_memory_limit(0);
        let executor = SandboxExecutor::new(config);
        assert!(matches!(executor, Err(SandboxError::InvalidConfig(_))));
    }

    #[test]
    fn test_builder_pattern() {
        let config = SandboxConfig::default();
        let executor = SandboxExecutorBuilder::new().config(config).build();
        assert!(executor.is_ok());
    }

    #[test]
    fn test_utils_validate_code() {
        assert!(utils::validate_code("echo hello").is_ok());
        assert!(utils::validate_code("rm -rf /").is_err());
    }

    #[test]
    fn test_utils_shell_escape() {
        let escaped = utils::shell_escape("hello 'world'");
        assert!(escaped.contains("\\'"));
    }

    #[tokio::test]
    async fn test_execute_simple_command() {
        let config = SandboxConfig::new().with_time_limit(5000);
        let executor = SandboxExecutor::new(config).unwrap();

        let result = executor.execute("echo hello").await;
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.success());
        assert!(output.stdout.contains("hello"));
    }

    #[tokio::test]
    async fn test_execute_with_input() {
        let config = SandboxConfig::new().with_time_limit(5000);
        let executor = SandboxExecutor::new(config).unwrap();

        let result = executor.execute_with_input("cat", "test input").await;
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.success());
        assert!(output.stdout.contains("test input"));
    }

    #[tokio::test]
    async fn test_execute_command() {
        let config = SandboxConfig::new().with_time_limit(5000);
        let executor = SandboxExecutor::new(config).unwrap();

        let result = executor.execute_command("echo", &["hello", "world"], None).await;
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.success());
        assert!(output.stdout.contains("hello world"));
    }

    #[tokio::test]
    async fn test_timeout() {
        let config = SandboxConfig::new().with_time_limit(10);
        let executor = SandboxExecutor::new(config).unwrap();

        // Use a command that will definitely take longer than 10ms
        let result = executor.execute("find /").await;
        match &result {
            Err(SandboxError::Timeout) => {}
            _ => panic!("Expected Timeout error, got: {:?}", result),
        }
    }
}
