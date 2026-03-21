//! Resource limit enforcement for sandbox execution
//!
//! This module provides mechanisms for enforcing resource limits on sandboxed processes,
//! including memory, CPU time, file descriptors, and process count limits.

use crate::sandbox::config::SandboxConfig;
use std::time::{Duration, Instant};

/// Tracks resource usage during sandbox execution
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceUsage {
    /// Memory used in bytes
    pub memory_bytes: u64,
    /// CPU time used in milliseconds
    pub cpu_time_ms: u64,
    /// Wall clock time in milliseconds
    pub wall_time_ms: u64,
    /// Number of file descriptors used
    pub file_descriptors: u32,
    /// Number of processes/threads
    pub process_count: u32,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_bytes: 0,
            cpu_time_ms: 0,
            wall_time_ms: 0,
            file_descriptors: 0,
            process_count: 1,
        }
    }
}

impl ResourceUsage {
    /// Create a new resource usage tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Get memory usage in megabytes
    pub fn memory_mb(&self) -> u64 {
        self.memory_bytes / (1024 * 1024)
    }

    /// Update memory usage
    pub fn update_memory(&mut self, bytes: u64) {
        self.memory_bytes = self.memory_bytes.max(bytes);
    }

    /// Update CPU time
    pub fn update_cpu_time(&mut self, ms: u64) {
        self.cpu_time_ms = ms;
    }

    /// Update wall clock time
    pub fn update_wall_time(&mut self, ms: u64) {
        self.wall_time_ms = ms;
    }

    /// Update file descriptor count
    pub fn update_file_descriptors(&mut self, count: u32) {
        self.file_descriptors = count;
    }

    /// Update process count
    pub fn update_process_count(&mut self, count: u32) {
        self.process_count = count;
    }
}

/// Resource limit checker
///
/// Validates resource usage against configured limits and detects violations.
#[derive(Debug, Clone)]
pub struct ResourceLimiter {
    config: SandboxConfig,
    start_time: Instant,
}

impl ResourceLimiter {
    /// Create a new resource limiter with the given configuration
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config,
            start_time: Instant::now(),
        }
    }

    /// Reset the start time
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    /// Check if memory limit is exceeded
    pub fn check_memory(&self, usage: &ResourceUsage) -> Result<(), LimitError> {
        let limit_bytes = self.config.memory_limit_bytes();
        if usage.memory_bytes > limit_bytes {
            return Err(LimitError::MemoryExceeded {
                used: usage.memory_bytes,
                limit: limit_bytes,
            });
        }
        Ok(())
    }

    /// Check if time limit is exceeded
    pub fn check_time(&self, usage: &ResourceUsage) -> Result<(), LimitError> {
        if usage.wall_time_ms > self.config.time_limit_ms {
            return Err(LimitError::TimeExceeded {
                used_ms: usage.wall_time_ms,
                limit_ms: self.config.time_limit_ms,
            });
        }
        Ok(())
    }

    /// Check if file descriptor limit is exceeded
    pub fn check_file_descriptors(&self, usage: &ResourceUsage) -> Result<(), LimitError> {
        if usage.file_descriptors > self.config.max_file_descriptors {
            return Err(LimitError::FileDescriptorExceeded {
                used: usage.file_descriptors,
                limit: self.config.max_file_descriptors,
            });
        }
        Ok(())
    }

    /// Check if process limit is exceeded
    pub fn check_processes(&self, usage: &ResourceUsage) -> Result<(), LimitError> {
        if usage.process_count > self.config.max_processes {
            return Err(LimitError::ProcessLimitExceeded {
                used: usage.process_count,
                limit: self.config.max_processes,
            });
        }
        Ok(())
    }

    /// Check if output size limit is exceeded
    pub fn check_output_size(&self, size: usize) -> Result<(), LimitError> {
        if size > self.config.max_output_size {
            return Err(LimitError::OutputSizeExceeded {
                used: size,
                limit: self.config.max_output_size,
            });
        }
        Ok(())
    }

    /// Check all resource limits
    pub fn check_all(&self, usage: &ResourceUsage) -> Result<(), LimitError> {
        self.check_memory(usage)?;
        self.check_time(usage)?;
        self.check_file_descriptors(usage)?;
        self.check_processes(usage)?;
        Ok(())
    }

    /// Get elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get elapsed milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    /// Get the configuration
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
}

/// Errors related to resource limit violations
#[derive(Debug, Clone, PartialEq)]
pub enum LimitError {
    /// Memory limit exceeded
    MemoryExceeded { used: u64, limit: u64 },
    /// Time limit exceeded
    TimeExceeded { used_ms: u64, limit_ms: u64 },
    /// File descriptor limit exceeded
    FileDescriptorExceeded { used: u32, limit: u32 },
    /// Process limit exceeded
    ProcessLimitExceeded { used: u32, limit: u32 },
    /// Output size limit exceeded
    OutputSizeExceeded { used: usize, limit: usize },
    /// System error when checking limits
    SystemError(String),
}

impl std::fmt::Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitError::MemoryExceeded { used, limit } => {
                write!(
                    f,
                    "Memory limit exceeded: {} MB used (limit: {} MB)",
                    used / (1024 * 1024),
                    limit / (1024 * 1024)
                )
            }
            LimitError::TimeExceeded { used_ms, limit_ms } => {
                write!(
                    f,
                    "Time limit exceeded: {} ms used (limit: {} ms)",
                    used_ms, limit_ms
                )
            }
            LimitError::FileDescriptorExceeded { used, limit } => {
                write!(
                    f,
                    "File descriptor limit exceeded: {} used (limit: {})",
                    used, limit
                )
            }
            LimitError::ProcessLimitExceeded { used, limit } => {
                write!(
                    f,
                    "Process limit exceeded: {} processes (limit: {})",
                    used, limit
                )
            }
            LimitError::OutputSizeExceeded { used, limit } => {
                write!(
                    f,
                    "Output size limit exceeded: {} bytes (limit: {} bytes)",
                    used, limit
                )
            }
            LimitError::SystemError(msg) => write!(f, "System error checking limits: {}", msg),
        }
    }
}

impl std::error::Error for LimitError {}

/// Platform-specific resource limit enforcement
#[cfg(target_os = "linux")]
pub mod platform {
    use super::*;
    use std::io;

    /// Set RLIMIT_AS (address space limit) for the current process
    pub fn set_memory_limit(bytes: u64) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: bytes,
            rlim_max: bytes,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_AS, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Set RLIMIT_CPU (CPU time limit) for the current process
    pub fn set_cpu_limit(seconds: u64) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: seconds,
            rlim_max: seconds,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_CPU, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Set RLIMIT_NOFILE (file descriptor limit) for the current process
    pub fn set_fd_limit(count: u32) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: count as u64,
            rlim_max: count as u64,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Set RLIMIT_NPROC (process limit) for the current process
    pub fn set_process_limit(count: u32) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: count as u64,
            rlim_max: count as u64,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_NPROC, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Set RLIMIT_FSIZE (file size limit) for the current process
    pub fn set_file_size_limit(bytes: u64) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: bytes,
            rlim_max: bytes,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_FSIZE, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Set RLIMIT_STACK (stack size limit) for the current process
    pub fn set_stack_limit(bytes: u64) -> io::Result<()> {
        let limit = libc::rlimit {
            rlim_cur: bytes,
            rlim_max: bytes,
        };
        let result = unsafe { libc::setrlimit(libc::RLIMIT_STACK, &limit) };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// Apply all resource limits from a configuration
    pub fn apply_limits(config: &SandboxConfig) -> io::Result<()> {
        set_memory_limit(config.memory_limit_bytes())?;
        set_cpu_limit(config.time_limit_seconds() as u64)?;
        set_fd_limit(config.max_file_descriptors)?;
        set_process_limit(config.max_processes)?;
        Ok(())
    }

    /// Get current resource usage
    pub fn get_resource_usage() -> io::Result<ResourceUsage> {
        let mut usage = libc::rusage {
            ru_utime: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            ru_stime: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            ru_maxrss: 0,
            ru_ixrss: 0,
            ru_idrss: 0,
            ru_isrss: 0,
            ru_minflt: 0,
            ru_majflt: 0,
            ru_nswap: 0,
            ru_inblock: 0,
            ru_oublock: 0,
            ru_msgsnd: 0,
            ru_msgrcv: 0,
            ru_nsignals: 0,
            ru_nvcsw: 0,
            ru_nivcsw: 0,
        };

        let result = unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) };
        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        let cpu_time_ms = (usage.ru_utime.tv_sec + usage.ru_stime.tv_sec) as u64 * 1000
            + (usage.ru_utime.tv_usec + usage.ru_stime.tv_usec) as u64 / 1000;

        Ok(ResourceUsage {
            memory_bytes: usage.ru_maxrss as u64 * 1024,
            cpu_time_ms,
            wall_time_ms: 0,
            file_descriptors: 0,
            process_count: 1,
        })
    }
}

/// Platform-specific resource limit enforcement (non-Linux stub)
#[cfg(not(target_os = "linux"))]
pub mod platform {
    use super::*;
    use std::io;

    pub fn set_memory_limit(_bytes: u64) -> io::Result<()> {
        Ok(())
    }

    pub fn set_cpu_limit(_seconds: u64) -> io::Result<()> {
        Ok(())
    }

    pub fn set_fd_limit(_count: u32) -> io::Result<()> {
        Ok(())
    }

    pub fn set_process_limit(_count: u32) -> io::Result<()> {
        Ok(())
    }

    pub fn set_file_size_limit(_bytes: u64) -> io::Result<()> {
        Ok(())
    }

    pub fn set_stack_limit(_bytes: u64) -> io::Result<()> {
        Ok(())
    }

    pub fn apply_limits(_config: &SandboxConfig) -> io::Result<()> {
        Ok(())
    }

    pub fn get_resource_usage() -> io::Result<ResourceUsage> {
        Ok(ResourceUsage::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        assert_eq!(usage.memory_bytes, 0);
        assert_eq!(usage.cpu_time_ms, 0);
        assert_eq!(usage.wall_time_ms, 0);
        assert_eq!(usage.file_descriptors, 0);
        assert_eq!(usage.process_count, 1);
    }

    #[test]
    fn test_resource_usage_updates() {
        let mut usage = ResourceUsage::new();
        usage.update_memory(1024 * 1024);
        assert_eq!(usage.memory_mb(), 1);

        usage.update_cpu_time(1000);
        assert_eq!(usage.cpu_time_ms, 1000);

        usage.update_wall_time(2000);
        assert_eq!(usage.wall_time_ms, 2000);

        usage.update_file_descriptors(10);
        assert_eq!(usage.file_descriptors, 10);

        usage.update_process_count(2);
        assert_eq!(usage.process_count, 2);
    }

    #[test]
    fn test_resource_limiter_checks() {
        let config = SandboxConfig::new()
            .with_memory_limit(100)
            .with_time_limit(1000)
            .with_max_file_descriptors(10)
            .with_max_processes(2);

        let limiter = ResourceLimiter::new(config);

        let mut usage = ResourceUsage::new();
        assert!(limiter.check_all(&usage).is_ok());

        usage.update_memory(101 * 1024 * 1024);
        assert!(matches!(
            limiter.check_memory(&usage),
            Err(LimitError::MemoryExceeded { .. })
        ));

        usage = ResourceUsage::new();
        usage.update_wall_time(1001);
        assert!(matches!(
            limiter.check_time(&usage),
            Err(LimitError::TimeExceeded { .. })
        ));

        usage = ResourceUsage::new();
        usage.update_file_descriptors(11);
        assert!(matches!(
            limiter.check_file_descriptors(&usage),
            Err(LimitError::FileDescriptorExceeded { .. })
        ));

        usage = ResourceUsage::new();
        usage.update_process_count(3);
        assert!(matches!(
            limiter.check_processes(&usage),
            Err(LimitError::ProcessLimitExceeded { .. })
        ));
    }

    #[test]
    fn test_output_size_limit() {
        let config = SandboxConfig::new().with_max_output_size(100);
        let limiter = ResourceLimiter::new(config);

        assert!(limiter.check_output_size(50).is_ok());
        assert!(matches!(
            limiter.check_output_size(101),
            Err(LimitError::OutputSizeExceeded { .. })
        ));
    }

    #[test]
    fn test_limit_error_display() {
        let err = LimitError::MemoryExceeded {
            used: 200 * 1024 * 1024,
            limit: 100 * 1024 * 1024,
        };
        let msg = err.to_string();
        assert!(msg.contains("Memory limit exceeded"));
        assert!(msg.contains("200 MB"));
        assert!(msg.contains("100 MB"));
    }
}
