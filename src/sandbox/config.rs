//! Sandbox configuration module
//!
//! This module defines the configuration structures for the sandbox execution environment.
//! All limits and security policies are configurable to allow flexible deployment scenarios
//! while maintaining secure defaults.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Default memory limit in megabytes (256 MB)
pub const DEFAULT_MEMORY_LIMIT_MB: u64 = 256;

/// Default time limit in milliseconds (30 seconds)
pub const DEFAULT_TIME_LIMIT_MS: u64 = 30000;

/// Default maximum number of file descriptors (64)
pub const DEFAULT_MAX_FILE_DESCRIPTORS: u32 = 64;

/// Default maximum output size in bytes (1 MB)
pub const DEFAULT_MAX_OUTPUT_SIZE: usize = 1024 * 1024;

/// Default maximum number of processes (1 - no forking allowed)
pub const DEFAULT_MAX_PROCESSES: u32 = 1;

/// Default allowed syscalls for basic code execution
pub const DEFAULT_ALLOWED_SYSCALLS: &[&str] = &[
    // Memory management
    "brk",
    "mmap",
    "mmap2",
    "munmap",
    "mprotect",
    // File operations (read-only)
    "openat",
    "close",
    "read",
    "pread64",
    "lseek",
    "fstat",
    "fstat64",
    "stat",
    "stat64",
    "access",
    "faccessat",
    // Process control
    "exit",
    "exit_group",
    "getpid",
    "getppid",
    "getpgrp",
    "getpgid",
    "getsid",
    "getuid",
    "geteuid",
    "getgid",
    "getegid",
    "getgroups",
    "getrlimit",
    "gettimeofday",
    "clock_gettime",
    "time",
    "getcwd",
    "chdir",
    // Signal handling
    "rt_sigaction",
    "rt_sigprocmask",
    "rt_sigreturn",
    "sigaltstack",
    // Threading
    "clone",
    "set_tid_address",
    "set_robust_list",
    "futex",
    "sched_getaffinity",
    "sched_setaffinity",
    "sched_yield",
    "sched_getparam",
    "sched_getscheduler",
    // I/O
    "write",
    "writev",
    "pwrite64",
    "fcntl",
    "ioctl",
    "pipe",
    "pipe2",
    "dup",
    "dup2",
    "dup3",
    // Memory
    "madvise",
    "mincore",
    "msync",
    "mremap",
    // Misc
    "uname",
    "sysinfo",
    "prctl",
    "arch_prctl",
    "setrlimit",
];

/// Configuration for the sandbox execution environment
///
/// This struct defines all security and resource limits for sandboxed code execution.
/// Use `SandboxConfig::default()` for secure defaults or build custom configurations
/// using the builder pattern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SandboxConfig {
    /// Maximum memory allowed in megabytes
    pub memory_limit_mb: u64,

    /// Maximum execution time in milliseconds
    pub time_limit_ms: u64,

    /// Maximum number of file descriptors
    pub max_file_descriptors: u32,

    /// Maximum number of processes/threads allowed
    pub max_processes: u32,

    /// Maximum output size in bytes (stdout + stderr)
    pub max_output_size: usize,

    /// List of allowed system calls
    pub allowed_syscalls: Vec<String>,

    /// Whether network access is enabled (default: false)
    pub enable_network: bool,

    /// Working directory for sandbox execution
    pub working_directory: Option<PathBuf>,

    /// Environment variables to set in sandbox
    pub environment_variables: Vec<(String, String)>,

    /// Whether to mount /tmp as tmpfs
    pub mount_tmp: bool,

    /// Additional read-only directories to mount
    pub read_only_dirs: Vec<PathBuf>,

    /// Whether to use seccomp for syscall filtering
    pub use_seccomp: bool,

    /// Whether to use cgroups for resource limiting
    pub use_cgroups: bool,

    /// Whether to use namespaces for isolation
    pub use_namespaces: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            time_limit_ms: DEFAULT_TIME_LIMIT_MS,
            max_file_descriptors: DEFAULT_MAX_FILE_DESCRIPTORS,
            max_processes: DEFAULT_MAX_PROCESSES,
            max_output_size: DEFAULT_MAX_OUTPUT_SIZE,
            allowed_syscalls: DEFAULT_ALLOWED_SYSCALLS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            enable_network: false,
            working_directory: None,
            environment_variables: Vec::new(),
            mount_tmp: true,
            read_only_dirs: Vec::new(),
            use_seccomp: true,
            use_cgroups: true,
            use_namespaces: true,
        }
    }
}

impl SandboxConfig {
    /// Create a new sandbox configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set memory limit in megabytes
    pub fn with_memory_limit(mut self, mb: u64) -> Self {
        self.memory_limit_mb = mb;
        self
    }

    /// Set time limit in milliseconds
    pub fn with_time_limit(mut self, ms: u64) -> Self {
        self.time_limit_ms = ms;
        self
    }

    /// Set maximum file descriptors
    pub fn with_max_file_descriptors(mut self, count: u32) -> Self {
        self.max_file_descriptors = count;
        self
    }

    /// Set maximum processes
    pub fn with_max_processes(mut self, count: u32) -> Self {
        self.max_processes = count;
        self
    }

    /// Set maximum output size in bytes
    pub fn with_max_output_size(mut self, bytes: usize) -> Self {
        self.max_output_size = bytes;
        self
    }

    /// Set allowed syscalls
    pub fn with_allowed_syscalls(mut self, syscalls: Vec<String>) -> Self {
        self.allowed_syscalls = syscalls;
        self
    }

    /// Enable network access
    pub fn with_network(mut self, enabled: bool) -> Self {
        self.enable_network = enabled;
        self
    }

    /// Set working directory
    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.working_directory = Some(path);
        self
    }

    /// Add environment variable
    pub fn with_env_var(mut self, key: String, value: String) -> Self {
        self.environment_variables.push((key, value));
        self
    }

    /// Set tmpfs mounting
    pub fn with_mount_tmp(mut self, mount: bool) -> Self {
        self.mount_tmp = mount;
        self
    }

    /// Add read-only directory
    pub fn with_read_only_dir(mut self, path: PathBuf) -> Self {
        self.read_only_dirs.push(path);
        self
    }

    /// Set seccomp usage
    pub fn with_seccomp(mut self, enabled: bool) -> Self {
        self.use_seccomp = enabled;
        self
    }

    /// Set cgroups usage
    pub fn with_cgroups(mut self, enabled: bool) -> Self {
        self.use_cgroups = enabled;
        self
    }

    /// Set namespaces usage
    pub fn with_namespaces(mut self, enabled: bool) -> Self {
        self.use_namespaces = enabled;
        self
    }

    /// Validate the configuration
    ///
    /// Returns an error if any configuration values are invalid or insecure
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.memory_limit_mb == 0 {
            return Err(ConfigError::InvalidValue(
                "memory_limit_mb must be greater than 0".to_string(),
            ));
        }

        if self.time_limit_ms == 0 {
            return Err(ConfigError::InvalidValue(
                "time_limit_ms must be greater than 0".to_string(),
            ));
        }

        if self.max_file_descriptors == 0 {
            return Err(ConfigError::InvalidValue(
                "max_file_descriptors must be greater than 0".to_string(),
            ));
        }

        if self.max_output_size == 0 {
            return Err(ConfigError::InvalidValue(
                "max_output_size must be greater than 0".to_string(),
            ));
        }

        if self.allowed_syscalls.is_empty() {
            return Err(ConfigError::InvalidValue(
                "allowed_syscalls cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the memory limit in bytes
    pub fn memory_limit_bytes(&self) -> u64 {
        self.memory_limit_mb * 1024 * 1024
    }

    /// Get the time limit in seconds (as f64)
    pub fn time_limit_seconds(&self) -> f64 {
        self.time_limit_ms as f64 / 1000.0
    }
}

/// Errors that can occur during configuration
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Invalid configuration value
    InvalidValue(String),
    /// Configuration file not found
    FileNotFound(String),
    /// Parse error
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found: {}", path),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SandboxConfig::default();
        assert_eq!(config.memory_limit_mb, DEFAULT_MEMORY_LIMIT_MB);
        assert_eq!(config.time_limit_ms, DEFAULT_TIME_LIMIT_MS);
        assert_eq!(config.max_file_descriptors, DEFAULT_MAX_FILE_DESCRIPTORS);
        assert!(!config.enable_network);
        assert!(config.use_seccomp);
        assert!(config.use_cgroups);
        assert!(config.use_namespaces);
    }

    #[test]
    fn test_builder_pattern() {
        let config = SandboxConfig::new()
            .with_memory_limit(512)
            .with_time_limit(60000)
            .with_network(true)
            .with_env_var("FOO".to_string(), "bar".to_string());

        assert_eq!(config.memory_limit_mb, 512);
        assert_eq!(config.time_limit_ms, 60000);
        assert!(config.enable_network);
        assert_eq!(config.environment_variables.len(), 1);
        assert_eq!(
            config.environment_variables[0],
            ("FOO".to_string(), "bar".to_string())
        );
    }

    #[test]
    fn test_config_validation() {
        let valid_config = SandboxConfig::default();
        assert!(valid_config.validate().is_ok());

        let invalid_config = SandboxConfig::new().with_memory_limit(0);
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_memory_limit_bytes() {
        let config = SandboxConfig::new().with_memory_limit(1);
        assert_eq!(config.memory_limit_bytes(), 1024 * 1024);
    }

    #[test]
    fn test_time_limit_seconds() {
        let config = SandboxConfig::new().with_time_limit(5000);
        assert_eq!(config.time_limit_seconds(), 5.0);
    }

    #[test]
    fn test_serialization() {
        let config = SandboxConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: SandboxConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
