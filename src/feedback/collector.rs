//! Feedback collector for AutoHarness
//!
//! This module provides the `FeedbackCollector` which collects and manages
//! feedback items from various sources during code execution.

use crate::feedback::types::*;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// Maximum number of failed steps to collect (as per paper)
pub const DEFAULT_MAX_FAILED_STEPS: usize = 5;

/// Default maximum number of feedback items to store
pub const DEFAULT_MAX_ITEMS: usize = 100;

/// Collector that gathers feedback from multiple sources
#[derive(Debug, Clone)]
pub struct FeedbackCollector {
    /// Queue of feedback items (oldest first)
    items: VecDeque<FeedbackItem>,
    /// Maximum number of items to store
    max_items: usize,
    /// Maximum number of failed steps to track
    max_failed_steps: usize,
    /// Current step number in execution
    current_step: usize,
    /// Statistics about collected feedback
    stats: FeedbackStats,
    /// Whether to auto-increment step on each add
    auto_increment_step: bool,
}

impl FeedbackCollector {
    /// Create a new feedback collector with default limits
    pub fn new() -> Self {
        Self::with_limits(DEFAULT_MAX_ITEMS, DEFAULT_MAX_FAILED_STEPS)
    }

    /// Create a feedback collector with custom limits
    pub fn with_limits(max_items: usize, max_failed_steps: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(max_items.min(100)),
            max_items,
            max_failed_steps,
            current_step: 0,
            stats: FeedbackStats::new(),
            auto_increment_step: false,
        }
    }

    /// Create a collector configured for the paper's approach (5 failed steps)
    pub fn for_paper_approach() -> Self {
        Self::with_limits(DEFAULT_MAX_ITEMS, DEFAULT_MAX_FAILED_STEPS)
    }

    /// Add a feedback item to the collector
    pub fn add(&mut self, mut item: FeedbackItem) -> bool {
        // Auto-assign step number if not set and auto-increment is enabled
        if item.step_number.is_none() && self.auto_increment_step {
            item.step_number = Some(self.current_step);
        }

        // Update statistics
        self.stats.add(&item);

        // Add to collection
        self.items.push_back(item);

        // Trim if exceeding max items (remove oldest)
        while self.items.len() > self.max_items {
            self.items.pop_front();
        }

        true
    }

    /// Add an error feedback item
    pub fn add_error(
        &mut self,
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> bool {
        let item = FeedbackItem::error(feedback_type, message, source).with_step(self.current_step);
        self.add(item)
    }

    /// Add a warning feedback item
    pub fn add_warning(
        &mut self,
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> bool {
        let item =
            FeedbackItem::warning(feedback_type, message, source).with_step(self.current_step);
        self.add(item)
    }

    /// Add an info feedback item
    pub fn add_info(
        &mut self,
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> bool {
        let item = FeedbackItem::info(feedback_type, message, source).with_step(self.current_step);
        self.add(item)
    }

    /// Add a critical feedback item
    pub fn add_critical(
        &mut self,
        feedback_type: FeedbackType,
        message: impl Into<String>,
        source: FeedbackSource,
    ) -> bool {
        let item =
            FeedbackItem::critical(feedback_type, message, source).with_step(self.current_step);
        self.add(item)
    }

    /// Increment the current step counter
    pub fn next_step(&mut self) {
        self.current_step += 1;
    }

    /// Set the current step number
    pub fn set_step(&mut self, step: usize) {
        self.current_step = step;
    }

    /// Get the current step number
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    /// Enable auto-increment of step numbers when adding items
    pub fn enable_auto_step(&mut self) {
        self.auto_increment_step = true;
    }

    /// Disable auto-increment of step numbers
    pub fn disable_auto_step(&mut self) {
        self.auto_increment_step = false;
    }

    /// Get all collected feedback items
    pub fn items(&self) -> &VecDeque<FeedbackItem> {
        &self.items
    }

    /// Get feedback items as a vector (cloned)
    pub fn to_vec(&self) -> Vec<FeedbackItem> {
        self.items.iter().cloned().collect()
    }

    /// Get only error-level or worse items
    pub fn errors(&self) -> Vec<&FeedbackItem> {
        self.items.iter().filter(|i| i.is_error()).collect()
    }

    /// Get items of a specific type
    pub fn by_type(&self, feedback_type: &FeedbackType) -> Vec<&FeedbackItem> {
        self.items
            .iter()
            .filter(|i| &i.feedback_type == feedback_type)
            .collect()
    }

    /// Get items from a specific source
    pub fn by_source(&self, source: &FeedbackSource) -> Vec<&FeedbackItem> {
        self.items.iter().filter(|i| &i.source == source).collect()
    }

    /// Get items at or above a severity level
    pub fn by_severity(&self, min_severity: FeedbackSeverity) -> Vec<&FeedbackItem> {
        self.items
            .iter()
            .filter(|i| i.severity >= min_severity)
            .collect()
    }

    /// Get the last N items
    pub fn last_n(&self, n: usize) -> Vec<&FeedbackItem> {
        self.items.iter().rev().take(n).collect::<Vec<_>>()
    }

    /// Get items from the last N steps (paper approach)
    pub fn from_last_steps(&self, n: usize) -> Vec<&FeedbackItem> {
        let cutoff_step = self.current_step.saturating_sub(n);
        self.items
            .iter()
            .filter(|i| i.step_number.map_or(true, |s| s > cutoff_step))
            .collect()
    }

    /// Get the last failed steps (up to max_failed_steps)
    pub fn last_failed_steps(&self) -> Vec<&FeedbackItem> {
        self.items
            .iter()
            .filter(|i| i.is_error())
            .rev()
            .take(self.max_failed_steps)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Get statistics about collected feedback
    pub fn stats(&self) -> &FeedbackStats {
        &self.stats
    }

    /// Get the number of items collected
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if collector is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if collector has reached capacity
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_items
    }

    /// Clear all collected feedback
    pub fn clear(&mut self) {
        self.items.clear();
        self.stats = FeedbackStats::new();
        self.current_step = 0;
    }

    /// Get the oldest item
    pub fn oldest(&self) -> Option<&FeedbackItem> {
        self.items.front()
    }

    /// Get the most recent item
    pub fn latest(&self) -> Option<&FeedbackItem> {
        self.items.back()
    }

    /// Get the timestamp of the first item
    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.oldest().map(|i| i.timestamp)
    }

    /// Get the timestamp of the last item
    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.latest().map(|i| i.timestamp)
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.stats.has_errors()
    }

    /// Check if there are any critical issues
    pub fn has_critical(&self) -> bool {
        self.stats.has_critical()
    }

    /// Get the count of failed steps
    pub fn failed_step_count(&self) -> usize {
        self.items.iter().filter(|i| i.is_error()).count()
    }

    /// Check if we've reached the maximum failed steps limit
    pub fn reached_failed_step_limit(&self) -> bool {
        self.failed_step_count() >= self.max_failed_steps
    }
}

impl Default for FeedbackCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing feedback collectors
#[derive(Debug)]
pub struct FeedbackCollectorBuilder {
    max_items: usize,
    max_failed_steps: usize,
    auto_increment_step: bool,
}

impl FeedbackCollectorBuilder {
    /// Create a new builder with defaults
    pub fn new() -> Self {
        Self {
            max_items: DEFAULT_MAX_ITEMS,
            max_failed_steps: DEFAULT_MAX_FAILED_STEPS,
            auto_increment_step: false,
        }
    }

    /// Set the maximum number of items
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max;
        self
    }

    /// Set the maximum number of failed steps
    pub fn max_failed_steps(mut self, max: usize) -> Self {
        self.max_failed_steps = max;
        self
    }

    /// Enable auto-increment of step numbers
    pub fn auto_increment_step(mut self) -> Self {
        self.auto_increment_step = true;
        self
    }

    /// Build the collector
    pub fn build(self) -> FeedbackCollector {
        let mut collector = FeedbackCollector::with_limits(self.max_items, self.max_failed_steps);
        if self.auto_increment_step {
            collector.enable_auto_step();
        }
        collector
    }
}

impl Default for FeedbackCollectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = FeedbackCollector::new();
        assert!(collector.is_empty());
        assert_eq!(collector.len(), 0);
        assert!(!collector.has_errors());
    }

    #[test]
    fn test_add_feedback() {
        let mut collector = FeedbackCollector::new();
        let item = FeedbackItem::error(
            FeedbackType::ExecutionError,
            "Test error",
            FeedbackSource::Execution,
        );

        assert!(collector.add(item));
        assert_eq!(collector.len(), 1);
        assert!(collector.has_errors());
    }

    #[test]
    fn test_step_tracking() {
        let mut collector = FeedbackCollector::new();
        collector.set_step(5);

        collector.add_error(
            FeedbackType::ExecutionError,
            "Error",
            FeedbackSource::Execution,
        );

        let items = collector.to_vec();
        assert_eq!(items[0].step_number, Some(5));
    }

    #[test]
    fn test_auto_step() {
        let mut collector = FeedbackCollector::new();
        collector.enable_auto_step();

        collector.add_error(
            FeedbackType::ExecutionError,
            "Error 1",
            FeedbackSource::Execution,
        );
        collector.next_step();
        collector.add_error(
            FeedbackType::ValidationError,
            "Error 2",
            FeedbackSource::Validation,
        );

        let items = collector.to_vec();
        assert_eq!(items[0].step_number, Some(0));
        assert_eq!(items[1].step_number, Some(1));
    }

    #[test]
    fn test_filtering() {
        let mut collector = FeedbackCollector::new();

        collector.add_error(
            FeedbackType::ExecutionError,
            "Error",
            FeedbackSource::Execution,
        );
        collector.add_warning(
            FeedbackType::ValidationError,
            "Warning",
            FeedbackSource::Validation,
        );
        collector.add_info(
            FeedbackType::ExecutionError,
            "Info",
            FeedbackSource::Execution,
        );

        assert_eq!(collector.errors().len(), 1);
        assert_eq!(collector.by_type(&FeedbackType::ExecutionError).len(), 2);
        assert_eq!(collector.by_source(&FeedbackSource::Execution).len(), 2);
    }

    #[test]
    fn test_last_failed_steps() {
        let mut collector = FeedbackCollector::with_limits(100, 3);

        // Add 5 errors
        for i in 0..5 {
            collector.add_error(
                FeedbackType::ExecutionError,
                format!("Error {}", i),
                FeedbackSource::Execution,
            );
        }

        // Should only get last 3
        let failed = collector.last_failed_steps();
        assert_eq!(failed.len(), 3);
    }

    #[test]
    fn test_capacity_limit() {
        let mut collector = FeedbackCollector::with_limits(3, 5);

        for i in 0..5 {
            collector.add_info(
                FeedbackType::ExecutionError,
                format!("Item {}", i),
                FeedbackSource::Execution,
            );
        }

        assert_eq!(collector.len(), 3);
        assert!(collector.is_full());
    }

    #[test]
    fn test_builder() {
        let collector = FeedbackCollectorBuilder::new()
            .max_items(50)
            .max_failed_steps(10)
            .auto_increment_step()
            .build();

        assert_eq!(collector.len(), 0);
    }

    #[test]
    fn test_clear() {
        let mut collector = FeedbackCollector::new();
        collector.add_error(
            FeedbackType::ExecutionError,
            "Error",
            FeedbackSource::Execution,
        );

        collector.clear();
        assert!(collector.is_empty());
        assert_eq!(collector.current_step(), 0);
    }
}
