//! Tree search implementation for code synthesis
//!
//! This module implements a tree-based search algorithm for exploring
//! code variants. Each node represents a code snippet, and edges represent
//! mutations or transformations.

use crate::core::error::{HarnessError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A node in the search tree representing a code variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    /// Unique identifier for this node
    pub id: String,
    /// The code snippet represented by this node
    pub code: String,
    /// Score assigned by the evaluator (0.0 to 1.0)
    pub score: f64,
    /// Number of times this node has been visited
    pub visits: u32,
    /// IDs of child nodes
    pub children: Vec<String>,
    /// ID of parent node (None for root)
    pub parent: Option<String>,
    /// Depth in the tree (0 for root)
    pub depth: u32,
    /// Whether this node has been fully expanded
    pub expanded: bool,
}

impl CodeNode {
    /// Create a new code node
    pub fn new(code: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            code,
            score: 0.0,
            visits: 0,
            children: Vec::new(),
            parent: None,
            depth: 0,
            expanded: false,
        }
    }

    /// Create a new code node with a specific parent
    pub fn with_parent(code: String, parent_id: String, depth: u32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            code,
            score: 0.0,
            visits: 0,
            children: Vec::new(),
            parent: Some(parent_id),
            depth,
            expanded: false,
        }
    }

    /// Update the score using incremental average
    pub fn update_score(&mut self, new_score: f64) {
        self.visits += 1;
        self.score = (self.score * (self.visits - 1) as f64 + new_score) / self.visits as f64;
    }

    /// Add a child node ID
    pub fn add_child(&mut self, child_id: String) {
        self.children.push(child_id);
    }

    /// Mark this node as expanded
    pub fn mark_expanded(&mut self) {
        self.expanded = true;
    }

    /// Check if this node is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Check if this node is the root
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Get the upper confidence bound for UCT selection
    pub fn uct_score(&self, parent_visits: u32, exploration_constant: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        let exploitation = self.score;
        let exploration =
            exploration_constant * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        exploitation + exploration
    }
}

/// Search tree for code variants
#[derive(Debug, Clone)]
pub struct SearchTree {
    /// Root node of the tree
    root: CodeNode,
    /// Map of node IDs to nodes
    nodes: HashMap<String, CodeNode>,
    /// Maximum depth of the tree
    max_depth: u32,
    /// Exploration constant for UCT
    exploration_constant: f64,
}

impl SearchTree {
    pub fn new(initial_code: String) -> Self {
        let root = CodeNode::new(initial_code);
        let mut nodes = HashMap::new();
        let root_id = root.id.clone();
        nodes.insert(root_id.clone(), root);
        let root_ref = nodes.get(&root_id).unwrap().clone();

        Self {
            root: root_ref,
            nodes,
            max_depth: 10,
            exploration_constant: 1.414,
        }
    }

    /// Create a new search tree with configuration
    pub fn with_config(initial_code: String, max_depth: u32, exploration_constant: f64) -> Self {
        let mut tree = Self::new(initial_code);
        tree.max_depth = max_depth;
        tree.exploration_constant = exploration_constant;
        tree
    }

    /// Get the root node
    pub fn root(&self) -> &CodeNode {
        &self.root
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&CodeNode> {
        self.nodes.get(id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut CodeNode> {
        self.nodes.get_mut(id)
    }

    /// Insert a new node into the tree
    pub fn insert_node(&mut self, node: CodeNode) -> Result<()> {
        let node_id = node.id.clone();
        let parent_id = node.parent.clone();

        self.nodes.insert(node_id.clone(), node);

        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.add_child(node_id);
            } else {
                return Err(HarnessError::not_found(format!(
                    "Parent node {} not found",
                    parent_id
                )));
            }
        }

        Ok(())
    }

    /// Select the best leaf node for expansion using UCT
    pub fn select_leaf(&self) -> Option<&CodeNode> {
        let mut current = &self.root;

        while !current.is_leaf() && current.depth < self.max_depth {
            let best_child = current
                .children
                .iter()
                .filter_map(|child_id| self.nodes.get(child_id))
                .max_by(|a, b| {
                    let score_a = a.uct_score(current.visits, self.exploration_constant);
                    let score_b = b.uct_score(current.visits, self.exploration_constant);
                    score_a
                        .partial_cmp(&score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

            match best_child {
                Some(child) => current = child,
                None => break,
            }
        }

        Some(current)
    }

    /// Get all leaf nodes
    pub fn get_leaves(&self) -> Vec<&CodeNode> {
        self.nodes.values().filter(|n| n.is_leaf()).collect()
    }

    /// Get the best node by score
    pub fn get_best_node(&self) -> Option<&CodeNode> {
        self.nodes.values().filter(|n| n.visits > 0).max_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get the path from root to a specific node
    pub fn get_path(&self, node_id: &str) -> Vec<&CodeNode> {
        let mut path = Vec::new();
        let mut current_id = Some(node_id.to_string());

        while let Some(id) = current_id {
            if let Some(node) = self.nodes.get(&id) {
                current_id = node.parent.clone();
                path.push(node);
            } else {
                break;
            }
        }

        path.reverse();
        path
    }

    /// Get all nodes at a specific depth
    pub fn get_nodes_at_depth(&self, depth: u32) -> Vec<&CodeNode> {
        self.nodes.values().filter(|n| n.depth == depth).collect()
    }

    /// Get the total number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Update the score of a node
    pub fn update_node_score(&mut self, node_id: &str, score: f64) -> Result<()> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.update_score(score);
            Ok(())
        } else {
            Err(HarnessError::not_found(format!(
                "Node {} not found",
                node_id
            )))
        }
    }

    /// Backpropagate a score up the tree
    pub fn backpropagate(&mut self, node_id: &str, score: f64) -> Result<()> {
        let mut current_id = Some(node_id.to_string());

        while let Some(id) = current_id {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.update_score(score);
                current_id = node.parent.clone();
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Check if the tree has converged (best score >= threshold)
    pub fn has_converged(&self, threshold: f64) -> bool {
        self.get_best_node()
            .map(|n| n.score >= threshold)
            .unwrap_or(false)
    }

    /// Get statistics about the tree
    pub fn stats(&self) -> TreeStats {
        let total_visits: u32 = self.nodes.values().map(|n| n.visits).sum();
        let avg_score = if !self.nodes.is_empty() {
            self.nodes.values().map(|n| n.score).sum::<f64>() / self.nodes.len() as f64
        } else {
            0.0
        };

        TreeStats {
            node_count: self.nodes.len(),
            max_depth: self.nodes.values().map(|n| n.depth).max().unwrap_or(0),
            total_visits,
            average_score: avg_score,
            best_score: self.get_best_node().map(|n| n.score).unwrap_or(0.0),
        }
    }
}

/// Statistics about the search tree
#[derive(Debug, Clone, Copy)]
pub struct TreeStats {
    /// Total number of nodes
    pub node_count: usize,
    /// Maximum depth reached
    pub max_depth: u32,
    /// Total visits across all nodes
    pub total_visits: u32,
    /// Average score across all nodes
    pub average_score: f64,
    /// Best score found
    pub best_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_node_creation() {
        let node = CodeNode::new("fn test() {}".to_string());
        assert!(!node.id.is_empty());
        assert_eq!(node.code, "fn test() {}");
        assert_eq!(node.score, 0.0);
        assert_eq!(node.visits, 0);
        assert!(node.is_root());
        assert!(node.is_leaf());
    }

    #[test]
    fn test_code_node_with_parent() {
        let parent = CodeNode::new("fn parent() {}".to_string());
        let parent_id = parent.id.clone();
        let child = CodeNode::with_parent("fn child() {}".to_string(), parent_id, 1);

        assert_eq!(child.parent.as_ref().unwrap(), &parent.id);
        assert_eq!(child.depth, 1);
        assert!(!child.is_root());
    }

    #[test]
    fn test_search_tree_creation() {
        let tree = SearchTree::new("fn main() {}".to_string());
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.root().code, "fn main() {}");
    }

    #[test]
    fn test_insert_node() {
        let mut tree = SearchTree::new("fn main() {}".to_string());
        let parent_id = tree.root().id.clone();
        let child = CodeNode::with_parent("fn child() {}".to_string(), parent_id.clone(), 1);
        let child_id = child.id.clone();

        tree.insert_node(child).unwrap();
        assert_eq!(tree.node_count(), 2);

        let parent = tree.get_node(&parent_id).unwrap();
        assert!(parent.children.contains(&child_id));
    }

    #[test]
    fn test_update_score() {
        let mut node = CodeNode::new("fn test() {}".to_string());
        node.update_score(0.5);
        assert_eq!(node.visits, 1);
        assert_eq!(node.score, 0.5);

        node.update_score(0.7);
        assert_eq!(node.visits, 2);
        assert_eq!(node.score, 0.6);
    }

    #[test]
    fn test_uct_score() {
        let node = CodeNode::new("fn test() {}".to_string());
        assert_eq!(node.uct_score(10, 1.414), f64::INFINITY);

        let mut node = CodeNode::new("fn test() {}".to_string());
        node.visits = 5;
        node.score = 0.5;
        let uct = node.uct_score(20, 1.414);
        assert!(uct > 0.5);
    }

    #[test]
    fn test_get_best_node() {
        let mut tree = SearchTree::new("fn main() {}".to_string());
        let root_id = tree.root().id.clone();
        tree.update_node_score(&root_id, 0.5).unwrap();

        let best = tree.get_best_node().unwrap();
        assert_eq!(best.score, 0.5);
    }

    #[test]
    fn test_has_converged() {
        let mut tree = SearchTree::new("fn main() {}".to_string());
        assert!(!tree.has_converged(0.9));

        let root_id = tree.root().id.clone();
        tree.update_node_score(&root_id, 0.95).unwrap();
        assert!(tree.has_converged(0.9));
    }
}
