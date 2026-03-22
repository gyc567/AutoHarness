//! Sandbox execution environment for AutoHarness
//!
//! This module provides a secure sandbox for executing untrusted code with
//! resource limits, process isolation, and comprehensive security controls.
//!
//! # Example
//!
//! ```rust
//! use autoharness::sandbox::{SandboxConfig, SandboxExecutor};
//!
//! async fn example() {
//!     let config = SandboxConfig::new()
//!         .with_memory_limit(256)
//!         .with_time_limit(30000);
//!
//!     let executor = SandboxExecutor::new(config).unwrap();
//!     let result = executor.execute("echo 'Hello, World!'").await.unwrap();
//!
//!     assert!(result.success());
//!     assert!(result.stdout.contains("Hello, World!"));
//! }
//! ```

pub mod config;
pub mod executor;
pub mod limits;

pub use config::{
    ConfigError, SandboxConfig, DEFAULT_ALLOWED_SYSCALLS, DEFAULT_MAX_FILE_DESCRIPTORS,
    DEFAULT_MAX_OUTPUT_SIZE, DEFAULT_MAX_PROCESSES, DEFAULT_MEMORY_LIMIT_MB, DEFAULT_TIME_LIMIT_MS,
};
pub use executor::{
    utils, ExecutionResult, SandboxError, SandboxExecutor, SandboxExecutorBuilder,
};
pub use limits::{
    platform, LimitError, ResourceLimiter, ResourceUsage,
};

use crate::core::error::HarnessError;

impl From<SandboxError> for HarnessError {
    fn from(err: SandboxError) -> Self {
        HarnessError::action_execution(err.to_string())
    }
}

/// Initialize the sandbox module
///
/// This function should be called before using any sandbox functionality.
/// It performs any necessary initialization for the sandbox environment.
pub fn init() {
    tracing::info!("Sandbox module initialized");
}

/// Check if the sandbox is supported on the current platform
///
/// Returns true if sandbox features can be used on this platform.
pub fn is_supported() -> bool {
    cfg!(target_os = "linux")
}

/// Get platform-specific sandbox capabilities
///
/// Returns a list of supported sandbox features on the current platform.
pub fn capabilities() -> Vec<String> {
    let caps = vec!["process_isolation".to_string(), "timeout".to_string()];

    #[cfg(target_os = "linux")]
    {
        caps.push("rlimits".to_string());
        caps.push("seccomp".to_string());
    }

    caps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
    }

    #[test]
    fn test_is_supported() {
        let supported = is_supported();
        #[cfg(target_os = "linux")]
        assert!(supported);
    }

    #[test]
    fn test_capabilities() {
        let caps = capabilities();
        assert!(!caps.is_empty());
        assert!(caps.contains(&"process_isolation".to_string()));
        assert!(caps.contains(&"timeout".to_string()));
    }

    #[test]
    fn test_error_conversion() {
        let sandbox_err = SandboxError::Timeout;
        let harness_err: HarnessError = sandbox_err.into();
        assert!(matches!(harness_err, HarnessError::ActionExecution(_)));
    }
}
