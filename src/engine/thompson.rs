//! Thompson sampling implementation for code synthesis
//!
//! Thompson sampling is a Bayesian approach to the multi-armed bandit problem
//! that balances exploration (trying new approaches) and exploitation (refining
//! what works). It uses Beta distributions to model the probability of success
//! for each option.

use crate::engine::search::CodeNode;
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use statrs::distribution::Beta;
use std::sync::Mutex;

/// Thompson sampler for node selection
///
/// Uses Beta distribution sampling to balance exploration vs exploitation.
/// Each node maintains alpha (successes) and beta (failures) parameters.
#[derive(Debug, Serialize)]
pub struct ThompsonSampler {
    alpha_prior: f64,
    beta_prior: f64,
    #[serde(skip)]
    rng: Mutex<StdRng>,
}

impl Clone for ThompsonSampler {
    fn clone(&self) -> Self {
        Self {
            alpha_prior: self.alpha_prior,
            beta_prior: self.beta_prior,
            rng: Mutex::new(StdRng::from_entropy()),
        }
    }
}

impl<'de> Deserialize<'de> for ThompsonSampler {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ThompsonSamplerData {
            alpha_prior: f64,
            beta_prior: f64,
        }

        let data = ThompsonSamplerData::deserialize(deserializer)?;
        Ok(Self {
            alpha_prior: data.alpha_prior,
            beta_prior: data.beta_prior,
            rng: Mutex::new(StdRng::from_entropy()),
        })
    }
}

impl Default for ThompsonSampler {
    fn default() -> Self {
        Self::default_prior()
    }
}

impl ThompsonSampler {
    /// Create a new Thompson sampler with default priors
    ///
    /// Default priors are alpha=1.0, beta=1.0 (uniform prior)
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self {
            alpha_prior: alpha,
            beta_prior: beta,
            rng: Mutex::new(StdRng::from_entropy()),
        }
    }

    /// Create a new Thompson sampler with default uniform prior
    pub fn default_prior() -> Self {
        Self::new(1.0, 1.0)
    }

    /// Create a new Thompson sampler with optimistic prior
    ///
    /// Optimistic prior encourages exploration early on
    pub fn optimistic_prior() -> Self {
        Self::new(1.0, 0.5)
    }

    /// Create a new Thompson sampler with conservative prior
    ///
    /// Conservative prior is more cautious about unexplored options
    pub fn conservative_prior() -> Self {
        Self::new(2.0, 2.0)
    }

    pub fn select_node<'a>(&self, nodes: &[&'a CodeNode]) -> Option<&'a CodeNode> {
        if nodes.is_empty() {
            return None;
        }

        let mut rng = self.rng.lock().unwrap();
        let mut best_node: Option<&CodeNode> = None;
        let mut best_sample: f64 = f64::NEG_INFINITY;

        for node in nodes {
            let (alpha, beta) = self.compute_beta_params(node);
            let beta_dist = Beta::new(alpha, beta).ok()?;
            let sample = beta_dist.sample(&mut *rng);

            if sample > best_sample {
                best_sample = sample;
                best_node = Some(*node);
            }
        }

        best_node
    }

    pub fn select_best_expected<'a>(&self, nodes: &[&'a CodeNode]) -> Option<&'a CodeNode> {
        nodes
            .iter()
            .max_by(|a, b| {
                let mean_a = self.expected_value(a);
                let mean_b = self.expected_value(b);
                mean_a
                    .partial_cmp(&mean_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }

    /// Compute Beta distribution parameters for a node
    ///
    /// alpha = prior_alpha + successes
    /// beta = prior_beta + failures
    fn compute_beta_params(&self, node: &CodeNode) -> (f64, f64) {
        let successes = node.score * node.visits as f64;
        let failures = node.visits as f64 - successes;

        let alpha = self.alpha_prior + successes.max(0.0);
        let beta = self.beta_prior + failures.max(0.0);

        (alpha, beta)
    }

    /// Compute the expected value (mean) of a node's Beta distribution
    fn expected_value(&self, node: &CodeNode) -> f64 {
        let (alpha, beta) = self.compute_beta_params(node);
        alpha / (alpha + beta)
    }

    /// Compute the variance of a node's Beta distribution
    fn variance(&self, node: &CodeNode) -> f64 {
        let (alpha, beta) = self.compute_beta_params(node);
        let sum = alpha + beta;
        (alpha * beta) / (sum * sum * (sum + 1.0))
    }

    /// Compute uncertainty (standard deviation) of a node's Beta distribution
    pub fn uncertainty(&self, node: &CodeNode) -> f64 {
        self.variance(node).sqrt()
    }

    /// Get the confidence interval for a node's score
    ///
    /// Returns (lower_bound, upper_bound) for the given confidence level
    pub fn confidence_interval(&self, node: &CodeNode, confidence: f64) -> (f64, f64) {
        let (alpha, beta) = self.compute_beta_params(node);

        // For simplicity, use normal approximation for large samples
        // or return full range [0, 1] for small samples
        let total = alpha + beta;
        if total < 10.0 {
            return (0.0, 1.0);
        }

        let mean = alpha / total;
        let std_dev = (alpha * beta / (total * total * (total + 1.0))).sqrt();

        // 95% confidence interval uses ~1.96 standard deviations
        let z = match confidence {
            c if c >= 0.99 => 2.576,
            c if c >= 0.95 => 1.96,
            c if c >= 0.90 => 1.645,
            _ => 1.0,
        };

        let margin = z * std_dev;
        let lower = (mean - margin).max(0.0);
        let upper = (mean + margin).min(1.0);

        (lower, upper)
    }

    /// Check if a node should be explored more (high uncertainty)
    pub fn should_explore(&self, node: &CodeNode, threshold: f64) -> bool {
        self.uncertainty(node) > threshold
    }

    /// Get the acquisition function value (UCB-like)
    ///
    /// Combines expected value with exploration bonus
    pub fn acquisition_value(&self, node: &CodeNode, exploration_weight: f64) -> f64 {
        let expected = self.expected_value(node);
        let uncertainty = self.uncertainty(node);
        expected + exploration_weight * uncertainty
    }
}

/// Batch Thompson sampler for efficient multi-selection
#[derive(Debug, Clone)]
pub struct BatchThompsonSampler {
    sampler: ThompsonSampler,
    batch_size: usize,
}

impl BatchThompsonSampler {
    /// Create a new batch sampler
    pub fn new(sampler: ThompsonSampler, batch_size: usize) -> Self {
        Self {
            sampler,
            batch_size,
        }
    }

    /// Select multiple nodes in a batch
    pub fn select_batch<'a>(&self, nodes: &[&'a CodeNode]) -> Vec<&'a CodeNode> {
        let mut selected = Vec::with_capacity(self.batch_size.min(nodes.len()));
        let mut available: Vec<&CodeNode> = nodes.to_vec();

        for _ in 0..self.batch_size {
            if available.is_empty() {
                break;
            }
            if let Some(node) = self.sampler.select_node(&available) {
                available.retain(|n| n.id != node.id);
                selected.push(node);
            }
        }

        selected
    }
}

/// Adaptive Thompson sampler that adjusts priors based on progress
#[derive(Debug, Clone)]
pub struct AdaptiveThompsonSampler {
    base_sampler: ThompsonSampler,
    iteration: u32,
    target_iterations: u32,
}

impl AdaptiveThompsonSampler {
    /// Create a new adaptive sampler
    pub fn new(target_iterations: u32) -> Self {
        Self {
            base_sampler: ThompsonSampler::optimistic_prior(),
            iteration: 0,
            target_iterations,
        }
    }

    /// Update iteration counter and adjust strategy
    pub fn step(&mut self) {
        self.iteration += 1;

        // Shift from exploration to exploitation as we progress
        let progress = self.iteration as f64 / self.target_iterations as f64;
        if progress > 0.7 {
            self.base_sampler = ThompsonSampler::conservative_prior();
        } else if progress > 0.3 {
            self.base_sampler = ThompsonSampler::default_prior();
        }
    }

    /// Select a node using the current strategy
    pub fn select_node<'a>(&self, nodes: &[&'a CodeNode]) -> Option<&'a CodeNode> {
        self.base_sampler.select_node(nodes)
    }

    /// Check if we should continue exploring
    pub fn should_continue(&self) -> bool {
        self.iteration < self.target_iterations
    }

    /// Get current iteration
    pub fn iteration(&self) -> u32 {
        self.iteration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(score: f64, visits: u32) -> CodeNode {
        let mut node = CodeNode::new("fn test() {}".to_string());
        node.score = score;
        node.visits = visits;
        node
    }

    #[test]
    fn test_thompson_sampler_creation() {
        let sampler = ThompsonSampler::new(1.0, 1.0);
        assert_eq!(sampler.alpha_prior, 1.0);
        assert_eq!(sampler.beta_prior, 1.0);
    }

    #[test]
    fn test_default_prior() {
        let sampler = ThompsonSampler::default_prior();
        assert_eq!(sampler.alpha_prior, 1.0);
        assert_eq!(sampler.beta_prior, 1.0);
    }

    #[test]
    fn test_optimistic_prior() {
        let sampler = ThompsonSampler::optimistic_prior();
        assert_eq!(sampler.alpha_prior, 1.0);
        assert_eq!(sampler.beta_prior, 0.5);
    }

    #[test]
    fn test_select_node_empty() {
        let sampler = ThompsonSampler::default();
        let nodes: Vec<&CodeNode> = vec![];
        assert!(sampler.select_node(&nodes).is_none());
    }

    #[test]
    fn test_select_node_single() {
        let sampler = ThompsonSampler::default();
        let node = create_test_node(0.5, 10);
        let nodes = vec![&node];
        let selected = sampler.select_node(&nodes);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, node.id);
    }

    #[test]
    fn test_expected_value() {
        let sampler = ThompsonSampler::default();
        let node = create_test_node(0.8, 10);
        let expected = sampler.expected_value(&node);
        assert!(expected > 0.0 && expected < 1.0);
    }

    #[test]
    fn test_uncertainty_high_for_unvisited() {
        let sampler = ThompsonSampler::default();
        let unvisited = create_test_node(0.0, 0);
        let visited = create_test_node(0.5, 100);

        let unvisited_uncertainty = sampler.uncertainty(&unvisited);
        let visited_uncertainty = sampler.uncertainty(&visited);

        assert!(unvisited_uncertainty > visited_uncertainty);
    }

    #[test]
    fn test_confidence_interval() {
        let sampler = ThompsonSampler::default();
        let node = create_test_node(0.5, 100);
        let (lower, upper) = sampler.confidence_interval(&node, 0.95);

        assert!(lower >= 0.0 && lower <= 1.0);
        assert!(upper >= 0.0 && upper <= 1.0);
        assert!(lower < upper);
    }

    #[test]
    fn test_should_explore() {
        let sampler = ThompsonSampler::default();
        let unvisited = create_test_node(0.0, 0);
        let visited = create_test_node(0.5, 100);

        assert!(sampler.should_explore(&unvisited, 0.1));
        assert!(!sampler.should_explore(&visited, 0.1));
    }

    #[test]
    fn test_batch_selection() {
        let sampler = ThompsonSampler::default();
        let batch_sampler = BatchThompsonSampler::new(sampler, 3);

        let node1 = create_test_node(0.3, 10);
        let node2 = create_test_node(0.5, 10);
        let node3 = create_test_node(0.7, 10);
        let node4 = create_test_node(0.9, 10);

        let nodes = vec![&node1, &node2, &node3, &node4];
        let selected = batch_sampler.select_batch(&nodes);

        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn test_adaptive_sampler() {
        let mut sampler = AdaptiveThompsonSampler::new(100);
        assert_eq!(sampler.iteration(), 0);

        sampler.step();
        assert_eq!(sampler.iteration(), 1);
        assert!(sampler.should_continue());
    }
}
