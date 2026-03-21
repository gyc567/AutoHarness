//! Main synthesis engine for AutoHarness
//!
//! This module implements the core code synthesis engine that uses tree search
//! with Thompson sampling to generate and optimize code harnesses. The engine
//! follows the approach described in the AutoHarness paper, achieving an average
//! of 14.5 iterations to reach 100% legal action rate.

use crate::core::error::{HarnessError, Result};
use crate::engine::search::{CodeNode, SearchTree};
use crate::engine::thompson::AdaptiveThompsonSampler;
use crate::engine::{MutationStrategy, SimpleMutationStrategy, SynthesisStats};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace, warn};

/// Error type for synthesis operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum SynthesisError {
    /// Error when synthesis fails to converge
    #[error("Synthesis failed to converge after {iterations} iterations")]
    ConvergenceFailure { iterations: u32 },

    /// Error when evaluator returns invalid score
    #[error("Invalid score from evaluator: {0}")]
    InvalidScore(String),

    /// Error when mutation produces invalid code
    #[error("Mutation failed: {0}")]
    MutationFailed(String),

    /// Error when search tree operation fails
    #[error("Search tree error: {0}")]
    SearchTreeError(String),

    /// Error when configuration is invalid
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl From<HarnessError> for SynthesisError {
    fn from(err: HarnessError) -> Self {
        SynthesisError::SearchTreeError(err.to_string())
    }
}

/// Configuration for the synthesis engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisConfig {
    /// Maximum number of iterations (default: 50, paper uses ~14.5 avg)
    pub max_iterations: u32,
    /// Convergence threshold for score (default: 0.95)
    pub convergence_threshold: f64,
    /// Maximum depth of the search tree (default: 10)
    pub max_depth: u32,
    /// Number of mutations to generate per node (default: 3)
    pub mutations_per_node: usize,
    /// Exploration constant for UCT (default: 1.414 = sqrt(2))
    pub exploration_constant: f64,
    /// Whether to use adaptive Thompson sampling (default: true)
    pub adaptive_sampling: bool,
    /// Target iterations for adaptive sampling (default: 20)
    pub target_iterations: u32,
    /// Minimum score improvement to continue (default: 0.01)
    pub min_improvement: f64,
    /// Maximum number of nodes to explore (default: 1000)
    pub max_nodes: usize,
}

impl SynthesisConfig {
    /// Create a new synthesis config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max_iterations: u32) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Set convergence threshold
    pub fn with_convergence_threshold(mut self, threshold: f64) -> Self {
        self.convergence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set maximum depth
    pub fn with_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Set mutations per node
    pub fn with_mutations_per_node(mut self, mutations: usize) -> Self {
        self.mutations_per_node = mutations;
        self
    }

    /// Set exploration constant
    pub fn with_exploration_constant(mut self, constant: f64) -> Self {
        self.exploration_constant = constant;
        self
    }

    /// Enable or disable adaptive sampling
    pub fn with_adaptive_sampling(mut self, adaptive: bool) -> Self {
        self.adaptive_sampling = adaptive;
        self
    }

    /// Set target iterations for adaptive sampling
    pub fn with_target_iterations(mut self, target: u32) -> Self {
        self.target_iterations = target;
        self
    }

    /// Set minimum improvement threshold
    pub fn with_min_improvement(mut self, improvement: f64) -> Self {
        self.min_improvement = improvement;
        self
    }

    /// Set maximum nodes
    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;
        self
    }
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            convergence_threshold: 0.95,
            max_depth: 10,
            mutations_per_node: 3,
            exploration_constant: 1.414,
            adaptive_sampling: true,
            target_iterations: 20,
            min_improvement: 0.01,
            max_nodes: 1000,
        }
    }
}

/// Trait for code evaluators
///
/// Implement this trait to provide custom evaluation logic for code harnesses.
/// The evaluator should return a score between 0.0 and 1.0, where 1.0 is perfect.
pub trait Evaluator: Send + Sync {
    /// Evaluate a code snippet and return a score
    ///
    /// The score should be between 0.0 and 1.0, where:
    /// - 0.0 = completely invalid/unusable code
    /// - 1.0 = perfect code that meets all requirements
    fn evaluate(&self, code: &str) -> Result<f64>;

    /// Check if the code is syntactically valid
    fn is_valid(&self, code: &str) -> bool {
        self.evaluate(code).map(|s| s > 0.0).unwrap_or(false)
    }

    /// Get the name of this evaluator
    fn name(&self) -> &str {
        "default"
    }
}

/// Simple evaluator that checks basic code properties
#[derive(Debug, Clone)]
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    /// Create a new simple evaluator
    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, code: &str) -> Result<f64> {
        if code.is_empty() {
            return Ok(0.0);
        }

        let mut score: f64 = 0.5; // Base score

        // Check for balanced braces
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();
        if open_braces == close_braces {
            score += 0.2;
        }

        // Check for balanced parentheses
        let open_parens = code.matches('(').count();
        let close_parens = code.matches(')').count();
        if open_parens == close_parens {
            score += 0.15;
        }

        // Check for function definition
        if code.contains("fn ") {
            score += 0.1;
        }

        // Penalize very short code
        if code.len() < 10 {
            score -= 0.2;
        }

        // Penalize very long code
        if code.len() > 1000 {
            score -= 0.1;
        }

        Ok(score.clamp(0.0, 1.0))
    }

    fn name(&self) -> &str {
        "simple"
    }
}

pub struct CodeSynthesisEngine {
    tree: SearchTree,
    config: SynthesisConfig,
    stats: SynthesisStats,
    mutation_strategy: Box<dyn MutationStrategy>,
    sampler: AdaptiveThompsonSampler,
    best_code: Option<CodeNode>,
    iteration: u32,
}

impl CodeSynthesisEngine {
    /// Create a new synthesis engine with the given configuration
    pub fn new(config: SynthesisConfig) -> Self {
        let tree =
            SearchTree::with_config(String::new(), config.max_depth, config.exploration_constant);
        let sampler = AdaptiveThompsonSampler::new(config.target_iterations);

        Self {
            tree,
            config,
            stats: SynthesisStats::new(),
            mutation_strategy: Box::new(SimpleMutationStrategy::default()),
            sampler,
            best_code: None,
            iteration: 0,
        }
    }

    /// Create a new synthesis engine with default configuration
    pub fn default() -> Self {
        Self::new(SynthesisConfig::default())
    }

    /// Set a custom mutation strategy
    pub fn with_mutation_strategy(mut self, strategy: Box<dyn MutationStrategy>) -> Self {
        self.mutation_strategy = strategy;
        self
    }

    /// Synthesize optimized code starting from initial code
    ///
    /// This is the main entry point for code synthesis. It performs tree search
    /// with Thompson sampling to find the best code variant.
    pub fn synthesize(
        &mut self,
        initial_code: &str,
        evaluator: &dyn Evaluator,
    ) -> std::result::Result<String, SynthesisError> {
        info!("Starting code synthesis with initial code");

        // Initialize tree with initial code
        self.tree = SearchTree::with_config(
            initial_code.to_string(),
            self.config.max_depth,
            self.config.exploration_constant,
        );

        // Evaluate initial code
        let initial_score = evaluator.evaluate(initial_code)?;
        let root_id = self.tree.root().id.clone();
        self.tree.update_node_score(&root_id, initial_score)?;
        self.stats.update_score(initial_score);

        if initial_score >= self.config.convergence_threshold {
            info!("Initial code already meets convergence threshold");
            self.stats.mark_convergence(0);
            return Ok(initial_code.to_string());
        }

        // Main synthesis loop
        for iteration in 1..=self.config.max_iterations {
            self.iteration = iteration;
            self.sampler.step();

            trace!("Synthesis iteration {}", iteration);

            // Select node to expand
            let selected_node = self.select_node_for_expansion();

            if let Some(node) = selected_node {
                // Generate mutations
                let mutations = self.generate_mutations(&node.code);

                // Evaluate and add mutations to tree
                for mutated_code in mutations {
                    if self.tree.node_count() >= self.config.max_nodes {
                        warn!("Maximum node count reached, stopping expansion");
                        break;
                    }

                    let score = evaluator.evaluate(&mutated_code)?;
                    self.add_node(mutated_code.clone(), &node.id, node.depth + 1, score)?;
                    self.stats.update_score(score);

                    // Update best code
                    if self.best_code.as_ref().map(|n| n.score).unwrap_or(0.0) < score {
                        self.best_code = self.tree.get_node(&node.id).cloned();
                    }

                    // Check convergence
                    if score >= self.config.convergence_threshold {
                        info!("Convergence reached at iteration {}", iteration);
                        self.stats.mark_convergence(iteration);
                        return Ok(mutated_code);
                    }
                }
            } else {
                debug!("No expandable nodes found");
                break;
            }

            // Check for early stopping
            if self.should_stop_early() {
                debug!("Early stopping triggered");
                break;
            }
        }

        // Return best code found
        if let Some(best) = self.get_best_code() {
            info!("Synthesis completed. Best score: {:.2}", best.score);
            Ok(best.code.clone())
        } else {
            Err(SynthesisError::ConvergenceFailure {
                iterations: self.config.max_iterations,
            })
        }
    }

    /// Select a node for expansion using Thompson sampling
    fn select_node_for_expansion(&self) -> Option<CodeNode> {
        let leaves: Vec<&CodeNode> = self.tree.get_leaves();

        if leaves.is_empty() {
            return Some(self.tree.root().clone());
        }

        // Filter leaves that haven't reached max depth
        let expandable: Vec<&CodeNode> = leaves
            .into_iter()
            .filter(|n| n.depth < self.config.max_depth && !n.expanded)
            .collect();

        if expandable.is_empty() {
            return None;
        }

        self.sampler.select_node(&expandable).cloned()
    }

    /// Generate mutations for a code snippet
    fn generate_mutations(&self, code: &str) -> Vec<String> {
        self.mutation_strategy.mutate(code)
    }

    /// Add a new node to the tree
    fn add_node(
        &mut self,
        code: String,
        parent_id: &str,
        depth: u32,
        score: f64,
    ) -> std::result::Result<(), SynthesisError> {
        let mut node = CodeNode::with_parent(code, parent_id.to_string(), depth);
        node.score = score;
        node.visits = 1;

        let node_id = node.id.clone();
        self.tree.insert_node(node)?;

        // Mark parent as expanded
        if let Some(parent) = self.tree.get_node_mut(parent_id) {
            parent.mark_expanded();
        }

        // Backpropagate score
        self.tree.backpropagate(&node_id, score)?;

        Ok(())
    }

    /// Check if we should stop early
    fn should_stop_early(&self) -> bool {
        // Stop if we've reached max nodes
        if self.tree.node_count() >= self.config.max_nodes {
            return true;
        }

        // Stop if sampler indicates we should
        if !self.sampler.should_continue() {
            return true;
        }

        // Stop if no improvement for many iterations
        if self.iteration > 10 {
            let recent_scores: Vec<f64> = self.tree.get_leaves().iter().map(|n| n.score).collect();

            if !recent_scores.is_empty() {
                let max_score = recent_scores
                    .iter()
                    .cloned()
                    .fold(f64::NEG_INFINITY, f64::max);

                if max_score < self.config.convergence_threshold - 0.1 {
                    // Check if we've plateaued
                    let best = self.get_best_code();
                    if let Some(best_node) = best {
                        if self.iteration > self.config.max_iterations / 2
                            && best_node.score < self.config.convergence_threshold - 0.2
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Get the best code found so far
    pub fn get_best_code(&self) -> Option<&CodeNode> {
        self.tree.get_best_node()
    }

    /// Get synthesis statistics
    pub fn stats(&self) -> &SynthesisStats {
        &self.stats
    }

    /// Get the search tree
    pub fn tree(&self) -> &SearchTree {
        &self.tree
    }

    /// Get current iteration
    pub fn iteration(&self) -> u32 {
        self.iteration
    }

    /// Check if synthesis has converged
    pub fn has_converged(&self) -> bool {
        self.tree.has_converged(self.config.convergence_threshold)
    }

    /// Reset the engine for a new synthesis run
    pub fn reset(&mut self) {
        self.tree = SearchTree::with_config(
            String::new(),
            self.config.max_depth,
            self.config.exploration_constant,
        );
        self.stats = SynthesisStats::new();
        self.sampler = AdaptiveThompsonSampler::new(self.config.target_iterations);
        self.best_code = None;
        self.iteration = 0;
    }
}

impl Default for CodeSynthesisEngine {
    fn default() -> Self {
        Self::new(SynthesisConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis_config_default() {
        let config = SynthesisConfig::default();
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.convergence_threshold, 0.95);
        assert_eq!(config.max_depth, 10);
        assert_eq!(config.mutations_per_node, 3);
        assert!(config.adaptive_sampling);
    }

    #[test]
    fn test_synthesis_config_builder() {
        let config = SynthesisConfig::new()
            .with_max_iterations(100)
            .with_convergence_threshold(0.99)
            .with_max_depth(15);

        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.convergence_threshold, 0.99);
        assert_eq!(config.max_depth, 15);
    }

    #[test]
    fn test_simple_evaluator() {
        let evaluator = SimpleEvaluator::new();

        let score = evaluator.evaluate("fn test() {}").unwrap();
        assert!(score > 0.0);

        let score = evaluator.evaluate("").unwrap();
        assert_eq!(score, 0.0);

        let score = evaluator.evaluate("fn test() { { }").unwrap();
        assert!(score > 0.0);
    }

    #[test]
    fn test_engine_creation() {
        let engine = CodeSynthesisEngine::new(SynthesisConfig::default());
        assert_eq!(engine.iteration(), 0);
        assert!(!engine.has_converged());
    }

    #[test]
    fn test_synthesize_simple() {
        let config = SynthesisConfig::new()
            .with_max_iterations(10)
            .with_convergence_threshold(0.8);

        let mut engine = CodeSynthesisEngine::new(config);
        let evaluator = SimpleEvaluator::new();

        let result = engine.synthesize("fn test() {}", &evaluator);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(!code.is_empty());
    }

    #[test]
    fn test_engine_reset() {
        let config = SynthesisConfig::new().with_convergence_threshold(0.99);
        let mut engine = CodeSynthesisEngine::new(config);
        let evaluator = SimpleEvaluator::new();

        let _ = engine.synthesize("fn test() {}", &evaluator);
        assert!(engine.iteration() > 0);

        engine.reset();
        assert_eq!(engine.iteration(), 0);
        assert!(!engine.has_converged());
    }

    #[test]
    fn test_synthesis_error_display() {
        let err = SynthesisError::ConvergenceFailure { iterations: 50 };
        assert!(err.to_string().contains("50"));
    }
}
