//! Error types for AutoHarness
//!
//! This module defines the error types used throughout the AutoHarness library.
//! All errors implement the `HarnessError` trait for consistent error handling.

use thiserror::Error;

/// The main error type for AutoHarness operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HarnessError {
    /// Error when state validation fails
    #[error("State validation failed: {0}")]
    StateValidation(String),

    /// Error when action parsing fails
    #[error("Failed to parse action: {0}")]
    ActionParse(String),

    /// Error when action execution fails
    #[error("Action execution failed: {0}")]
    ActionExecution(String),

    /// Error when harness evaluation fails
    #[error("Harness evaluation failed: {0}")]
    Evaluation(String),

    /// Error when serialization/deserialization fails
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Error when a required resource is not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Error when an invalid configuration is provided
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Generic error for unexpected situations
    #[error("Internal error: {0}")]
    Internal(String),
}

impl HarnessError {
    /// Create a new state validation error
    pub fn state_validation<S: Into<String>>(msg: S) -> Self {
        Self::StateValidation(msg.into())
    }

    /// Create a new action parse error
    pub fn action_parse<S: Into<String>>(msg: S) -> Self {
        Self::ActionParse(msg.into())
    }

    /// Create a new action execution error
    pub fn action_execution<S: Into<String>>(msg: S) -> Self {
        Self::ActionExecution(msg.into())
    }

    /// Create a new evaluation error
    pub fn evaluation<S: Into<String>>(msg: S) -> Self {
        Self::Evaluation(msg.into())
    }

    /// Create a new serialization error
    pub fn serialization<S: Into<String>>(msg: S) -> Self {
        Self::Serialization(msg.into())
    }

    /// Create a new not found error
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a new invalid config error
    pub fn invalid_config<S: Into<String>>(msg: S) -> Self {
        Self::InvalidConfig(msg.into())
    }

    /// Create a new internal error
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }
}

/// Result type alias for AutoHarness operations
pub type Result<T> = std::result::Result<T, HarnessError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = HarnessError::state_validation("test error");
        assert!(matches!(err, HarnessError::StateValidation(_)));
        assert_eq!(err.to_string(), "State validation failed: test error");
    }

    #[test]
    fn test_error_equality() {
        let err1 = HarnessError::state_validation("test");
        let err2 = HarnessError::state_validation("test");
        let err3 = HarnessError::action_parse("test");

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
