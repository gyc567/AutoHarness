//! Feedback types for AutoHarness
//!
//! This module defines the core types used for collecting and representing
//! feedback from code execution and environment interactions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of feedback that can be collected from execution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackType {
    /// Action that violates constraints or is not allowed
    IllegalAction,
    /// Error during code execution (compilation or runtime)
    ExecutionError,
    /// Validation failed (assertions, checks, etc.)
    ValidationError,
    /// Execution timed out
    TimeoutError,
    /// Resource limit exceeded (memory, CPU, etc.)
    ResourceLimitExceeded,
    /// Environment interaction error
    EnvironmentError,
    /// Sandbox security violation
    SecurityViolation,
}

impl fmt::Display for FeedbackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedbackType::IllegalAction => write!(f, "Illegal Action"),
            FeedbackType::ExecutionError => write!(f, "Execution Error"),
            FeedbackType::ValidationError => write!(f, "Validation Error"),
            FeedbackType::TimeoutError => write!(f, "Timeout Error"),
            FeedbackType::ResourceLimitExceeded => write!(f, "Resource Limit Exceeded"),
            FeedbackType::EnvironmentError => write!(f, "Environment Error"),
            FeedbackType::SecurityViolation => write!(f, "Security Violation"),
        }
    }
}

/// Severity level of feedback items
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeedbackSeverity {
    /// Informational feedback, not an error
    Info = 0,
    /// Warning that may indicate a problem
    Warning = 1,
    /// Error that prevented successful execution
    Error = 2,
    /// Critical error that requires immediate attention
    Critical = 3,
}

impl fmt::Display for FeedbackSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedbackSeverity::Info => write!(f, "INFO"),
            FeedbackSeverity::Warning => write!(f, "WARNING"),
            FeedbackSeverity::Error => write!(f, "ERROR"),
            FeedbackSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Source of the feedback
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackSource {
    /// Feedback from code execution
    Execution,
    /// Feedback from state validation
    Validation,
    /// Feedback from environment interaction
    Environment,
    /// Feedback from sandbox limits
    Sandbox,
    /// Feedback from static analysis
    StaticAnalysis,
}

impl fmt::Display for FeedbackSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedbackSource::Execution => write!(f, "Execution"),
            FeedbackSource::Validation => write!(f, "Validation"),
            FeedbackSource::Environment => write!(f, "Environment"),
            FeedbackSource::Sandbox => write!(f, "Sandbox"),
            FeedbackSource::StaticAnalysis => write!(f, "Static Analysis"),
        }
    }
}

/// An individual feedback item representing a single observation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackItem {
    /// Type of feedback
    pub feedback_type: FeedbackType,
    /// Human-readable message describing the feedback
    pub message: String,
    /// Optional context (e.g., code snippet, stack trace)
    pub context: Option<String>,
    /// Source of the feedback
    pub source: FeedbackSource,
    /// Severity level
    pub severity: FeedbackSeverity,
    /// Timestamp when the feedback was created
    pub timestamp: DateTime<Utc>,
    /// Optional step number in the execution sequence
    pub step_number: Option<usize>,
    /// Optional action that triggered this feedback
    pub triggering_action: Option<String>,
}

impl FeedbackItem {
    /// Create a new feedback item with the current timestamp
    pub fn new(
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
        severity: FeedbackSeverity,
    ) -> Self {
        Self {
            feedback_type,
            message: message.into(),
            context: None,
            source,
            severity,
            timestamp: Utc::now(),
            step_number: None,
            triggering_action: None,
        }
    }

    /// Create a new error-level feedback item
    pub fn error(
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> Self {
        Self::new(feedback_type, message, source, FeedbackSeverity::Error)
    }

    /// Create a new warning-level feedback item
    pub fn warning(
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> Self {
        Self::new(feedback_type, message, source, FeedbackSeverity::Warning)
    }

    /// Create a new info-level feedback item
    pub fn info(
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> Self {
        Self::new(feedback_type, message, source, FeedbackSeverity::Info)
    }

    /// Create a new critical-level feedback item
    pub fn critical(
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> Self {
        Self::new(feedback_type, message, source, FeedbackSeverity::Critical)
    }

    /// Set the context for this feedback item
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Set the step number for this feedback item
    pub fn with_step(mut self, step: usize) -> Self {
        self.step_number = Some(step);
        self
    }

    /// Set the triggering action for this feedback item
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.triggering_action = Some(action.into());
        self
    }

    /// Check if this feedback item is an error or worse
    pub fn is_error(&self) -> bool {
        self.severity >= FeedbackSeverity::Error
    }

    /// Get a formatted string representation
    pub fn format(&self) -> String {
        let mut result = format!(
            "[{}] {} from {}: {}",
            self.severity, self.feedback_type, self.source, self.message
        );

        if let Some(step) = self.step_number {
            result.push_str(&format!(" (step {})", step));
        }

        if let Some(ref action) = self.triggering_action {
            result.push_str(&format!(" [action: {}]", action));
        }

        if let Some(ref ctx) = self.context {
            result.push_str(&format!("\n  Context: {}", ctx));
        }

        result
    }
}

impl fmt::Display for FeedbackItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Statistics about collected feedback
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedbackStats {
    /// Total number of feedback items
    pub total_count: usize,
    /// Count by type
    pub by_type: std::collections::HashMap<FeedbackType, usize>,
    /// Count by severity
    pub by_severity: std::collections::HashMap<FeedbackSeverity, usize>,
    /// Count by source
    pub by_source: std::collections::HashMap<FeedbackSource, usize>,
    /// Number of error-level or worse items
    pub error_count: usize,
    /// Number of critical items
    pub critical_count: usize,
}

impl FeedbackStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Update stats with a feedback item
    pub fn add(&mut self, item: &FeedbackItem) {
        self.total_count += 1;
        *self.by_type.entry(item.feedback_type.clone()).or_insert(0) += 1;
        *self.by_severity.entry(item.severity).or_insert(0) += 1;
        *self.by_source.entry(item.source.clone()).or_insert(0) += 1;

        if item.is_error() {
            self.error_count += 1;
        }
        if item.severity == FeedbackSeverity::Critical {
            self.critical_count += 1;
        }
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// Check if there are any critical issues
    pub fn has_critical(&self) -> bool {
        self.critical_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_item_creation() {
        let item = FeedbackItem::error(
            FeedbackType::ExecutionError,
            "Test error",
            FeedbackSource::Execution,
        );

        assert_eq!(item.feedback_type, FeedbackType::ExecutionError);
        assert_eq!(item.message, "Test error");
        assert_eq!(item.source, FeedbackSource::Execution);
        assert_eq!(item.severity, FeedbackSeverity::Error);
        assert!(item.is_error());
    }

    #[test]
    fn test_feedback_item_builder() {
        let item = FeedbackItem::error(
            FeedbackType::ValidationError,
            "Validation failed",
            FeedbackSource::Validation,
        )
        .with_context("assertion failed at line 42")
        .with_step(5)
        .with_action("assert_eq(x, y)");

        assert_eq!(item.step_number, Some(5));
        assert_eq!(
            item.context,
            Some("assertion failed at line 42".to_string())
        );
        assert_eq!(item.triggering_action, Some("assert_eq(x, y)".to_string()));
    }

    #[test]
    fn test_feedback_severity_ordering() {
        assert!(FeedbackSeverity::Info < FeedbackSeverity::Warning);
        assert!(FeedbackSeverity::Warning < FeedbackSeverity::Error);
        assert!(FeedbackSeverity::Error < FeedbackSeverity::Critical);
    }

    #[test]
    fn test_feedback_stats() {
        let mut stats = FeedbackStats::new();

        let item1 = FeedbackItem::error(
            FeedbackType::ExecutionError,
            "Error 1",
            FeedbackSource::Execution,
        );
        let item2 = FeedbackItem::warning(
            FeedbackType::ValidationError,
            "Warning 1",
            FeedbackSource::Validation,
        );
        let item3 = FeedbackItem::critical(
            FeedbackType::SecurityViolation,
            "Critical",
            FeedbackSource::Sandbox,
        );

        stats.add(&item1);
        stats.add(&item2);
        stats.add(&item3);

        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.error_count, 2); // Error + Critical
        assert_eq!(stats.critical_count, 1);
        assert!(stats.has_errors());
        assert!(stats.has_critical());
    }

    #[test]
    fn test_feedback_type_display() {
        assert_eq!(FeedbackType::ExecutionError.to_string(), "Execution Error");
        assert_eq!(FeedbackType::TimeoutError.to_string(), "Timeout Error");
    }
}
