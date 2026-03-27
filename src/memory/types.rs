//! Memory types for AutoHarness
//!
//! This module defines the core types for the memory system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A principle that applies across all template types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
    /// The principle text
    pub text: String,
    /// How many times this principle has been applied
    pub frequency: u32,
    /// How many times this principle led to success
    pub success_count: u32,
    /// When this principle was first added
    pub created_at: DateTime<Utc>,
    /// When this principle was last updated
    pub updated_at: DateTime<Utc>,
}

impl Principle {
    pub fn new(text: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            text: text.into(),
            frequency: 0,
            success_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn record_use(&mut self, success: bool) {
        self.frequency += 1;
        if success {
            self.success_count += 1;
        }
        self.updated_at = Utc::now();
    }

    pub fn success_rate(&self) -> f64 {
        if self.frequency == 0 {
            0.0
        } else {
            self.success_count as f64 / self.frequency as f64
        }
    }
}

/// A success pattern for a specific template type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPattern {
    /// Description of the pattern
    pub pattern: String,
    /// Code snippet (optional)
    pub code_snippet: Option<String>,
    /// How many times this pattern succeeded
    pub count: u32,
    /// When this pattern was first observed
    pub created_at: DateTime<Utc>,
}

impl SuccessPattern {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            code_snippet: None,
            count: 1,
            created_at: Utc::now(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code_snippet = Some(code.into());
        self
    }

    pub fn record_success(&mut self) {
        self.count += 1;
    }
}

/// An error seed - a known failure pattern to avoid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSeed {
    /// Description of the error
    pub description: String,
    /// Code snippet that caused the error (optional)
    pub code_snippet: Option<String>,
    /// How many times this error was observed
    pub count: u32,
    /// When this error was first observed
    pub created_at: DateTime<Utc>,
}

impl ErrorSeed {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            code_snippet: None,
            count: 1,
            created_at: Utc::now(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code_snippet = Some(code.into());
        self
    }

    pub fn record_occurrence(&mut self) {
        self.count += 1;
    }
}

/// Knowledge specific to a template type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateKnowledge {
    /// Success patterns observed
    pub success_patterns: Vec<SuccessPattern>,
    /// Error seeds to avoid
    pub failure_seeds: Vec<ErrorSeed>,
    /// Number of successful syntheses
    pub success_count: u32,
    /// Number of failed syntheses
    pub failure_count: u32,
    /// Last update time
    pub last_updated: DateTime<Utc>,
}

impl TemplateKnowledge {
    pub fn new() -> Self {
        Self {
            success_patterns: Vec::new(),
            failure_seeds: Vec::new(),
            success_count: 0,
            failure_count: 0,
            last_updated: Utc::now(),
        }
    }

    pub fn record_success(&mut self, pattern: SuccessPattern) {
        self.success_count += 1;
        self.last_updated = Utc::now();

        // Merge with existing or add new
        if let Some(existing) = self.success_patterns.iter_mut().find(|p| p.pattern == pattern.pattern) {
            existing.record_success();
        } else {
            self.success_patterns.push(pattern);
        }

        // Enforce limit
        self.prune_success_patterns(30);
    }

    pub fn record_failure(&mut self, seed: ErrorSeed) {
        self.failure_count += 1;
        self.last_updated = Utc::now();

        // Merge with existing or add new
        if let Some(existing) = self.failure_seeds.iter_mut().find(|s| s.description == seed.description) {
            existing.record_occurrence();
        } else {
            self.failure_seeds.push(seed);
        }

        // Enforce limit
        self.prune_failure_seeds(30);
    }

    fn prune_success_patterns(&mut self, max: usize) {
        if self.success_patterns.len() > max {
            // Sort by count (descending) and keep top N
            self.success_patterns.sort_by(|a, b| b.count.cmp(&a.count));
            self.success_patterns.truncate(max);
        }
    }

    fn prune_failure_seeds(&mut self, max: usize) {
        if self.failure_seeds.len() > max {
            // Sort by count (descending) and keep top N
            self.failure_seeds.sort_by(|a, b| b.count.cmp(&a.count));
            self.failure_seeds.truncate(max);
        }
    }
}

/// A lesson extracted from a synthesis run
#[derive(Debug, Clone)]
pub struct Lesson {
    /// The template type this lesson applies to
    pub template_type: crate::core::HarnessType,
    /// Whether the synthesis succeeded
    pub success: bool,
    /// The generated code (if successful)
    pub generated_code: Option<String>,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Number of iterations to converge
    pub iterations: u32,
    /// Final score achieved
    pub score: f64,
    /// Extracted patterns/seeds
    pub patterns: Vec<String>,
}

impl Lesson {
    pub fn success(
        template_type: crate::core::HarnessType,
        code: String,
        iterations: u32,
        score: f64,
        patterns: Vec<String>,
    ) -> Self {
        Self {
            template_type,
            success: true,
            generated_code: Some(code),
            error_message: None,
            iterations,
            score,
            patterns,
        }
    }

    pub fn failure(
        template_type: crate::core::HarnessType,
        error: String,
        iterations: u32,
        score: f64,
    ) -> Self {
        Self {
            template_type,
            success: false,
            generated_code: None,
            error_message: Some(error),
            iterations,
            score,
            patterns: Vec::new(),
        }
    }
}

/// Memory content that can be serialized to/from Markdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryContent {
    /// Global principles
    pub principles: Vec<Principle>,
    /// Template-specific knowledge
    pub template_knowledge: std::collections::HashMap<String, TemplateKnowledge>,
}

impl MemoryContent {
    pub fn new() -> Self {
        Self {
            principles: Vec::new(),
            template_knowledge: std::collections::HashMap::new(),
        }
    }

    /// Add a principle, enforcing the 20 principle limit
    pub fn add_principle(&mut self, mut principle: Principle) {
        principle.frequency = 1;
        principle.success_count = 1;

        if let Some(existing) = self.principles.iter_mut().find(|p| p.text == principle.text) {
            existing.record_use(true);
        } else {
            self.principles.push(principle);
        }

        // Prune to 20, keeping highest frequency
        self.prune_principles(20);
    }

    fn prune_principles(&mut self, max: usize) {
        if self.principles.len() > max {
            self.principles.sort_by(|a, b| b.frequency.cmp(&a.frequency));
            self.principles.truncate(max);
        }
    }
}