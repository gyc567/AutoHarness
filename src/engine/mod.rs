//! Code synthesis engine for AutoHarness
//!
//! This module provides the core code synthesis functionality using tree search
//! with Thompson sampling to generate and optimize code harnesses.
//!
//! # Architecture
//!
//! The synthesis engine consists of three main components:
//!
//! - `search`: Tree search algorithm for exploring code variants
//! - `thompson`: Thompson sampling for balancing exploration vs exploitation
//! - `synthesis`: Main synthesis engine that orchestrates the search process
//!
//! # Example
//!
//! ```rust
//! use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig};
//!
//! let config = SynthesisConfig::default();
//! let mut engine = CodeSynthesisEngine::new(config);
//! ```

pub mod search;
pub mod synthesis;
pub mod thompson;

pub use crate::templates::{
    AdaptiveTemplate, CriticTemplate, EnsembleTemplate, FilterTemplate, HarnessTemplate,
    PolicyTemplate, RefinerTemplate, TemplateConfig, VerifierTemplate,
};
pub use search::{CodeNode, SearchTree};
pub use synthesis::{CodeSynthesisEngine, Evaluator, SynthesisConfig, SynthesisError};
pub use thompson::ThompsonSampler;

use serde::{Deserialize, Serialize};

/// Statistics for the synthesis process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisStats {
    /// Total number of iterations performed
    pub iterations: u32,
    /// Number of nodes explored in the search tree
    pub nodes_explored: usize,
    /// Best score achieved
    pub best_score: f64,
    /// Average score across all evaluated nodes
    pub average_score: f64,
    /// Convergence iteration (when solution was found)
    pub convergence_iteration: Option<u32>,
}

impl SynthesisStats {
    /// Create new synthesis statistics
    pub fn new() -> Self {
        Self {
            iterations: 0,
            nodes_explored: 0,
            best_score: 0.0,
            average_score: 0.0,
            convergence_iteration: None,
        }
    }

    /// Update statistics with a new score
    pub fn update_score(&mut self, score: f64) {
        self.nodes_explored += 1;
        if score > self.best_score {
            self.best_score = score;
        }
        // Update running average
        self.average_score = (self.average_score * (self.nodes_explored - 1) as f64 + score)
            / self.nodes_explored as f64;
    }

    /// Mark convergence
    pub fn mark_convergence(&mut self, iteration: u32) {
        if self.convergence_iteration.is_none() {
            self.convergence_iteration = Some(iteration);
        }
    }
}

impl Default for SynthesisStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for synthesis operations
pub type Result<T> = std::result::Result<T, SynthesisError>;

/// Trait for code mutation strategies
pub trait MutationStrategy: Send + Sync {
    /// Generate mutations for a given code snippet
    fn mutate(&self, code: &str) -> Vec<String>;
    /// Get the name of this strategy
    fn name(&self) -> &str;
}

/// Simple mutation strategy that applies basic transformations
#[derive(Debug, Clone)]
pub struct SimpleMutationStrategy {
    max_mutations: usize,
}

impl SimpleMutationStrategy {
    /// Create a new simple mutation strategy
    pub fn new(max_mutations: usize) -> Self {
        Self { max_mutations }
    }
}

impl Default for SimpleMutationStrategy {
    fn default() -> Self {
        Self::new(5)
    }
}

impl MutationStrategy for SimpleMutationStrategy {
    fn mutate(&self, code: &str) -> Vec<String> {
        let mut mutations = Vec::new();

        // Add whitespace variations
        if code.contains("  ") {
            mutations.push(code.replace("  ", " "));
        }

        // Add newline variations
        if !code.contains('\n') && code.len() > 40 {
            let mid = code.len() / 2;
            if let Some(space_pos) = code[..mid].rfind(' ') {
                let mut mutated = code.to_string();
                mutated.insert(space_pos, '\n');
                mutations.push(mutated);
            }
        }

        // Add comment variations
        if !code.starts_with("//") {
            mutations.push(format!("// Auto-generated harness\n{}", code));
        }

        // Limit mutations
        mutations.truncate(self.max_mutations);
        mutations
    }

    fn name(&self) -> &str {
        "simple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis_stats() {
        let mut stats = SynthesisStats::new();
        assert_eq!(stats.iterations, 0);
        assert_eq!(stats.best_score, 0.0);

        stats.update_score(0.5);
        assert_eq!(stats.nodes_explored, 1);
        assert_eq!(stats.best_score, 0.5);

        stats.update_score(0.8);
        assert_eq!(stats.nodes_explored, 2);
        assert_eq!(stats.best_score, 0.8);

        stats.mark_convergence(10);
        assert_eq!(stats.convergence_iteration, Some(10));
    }

    #[test]
    fn test_simple_mutation() {
        let strategy = SimpleMutationStrategy::new(5);
        let code = "fn  test() {}";
        let mutations = strategy.mutate(code);
        assert!(!mutations.is_empty());
    }
}
