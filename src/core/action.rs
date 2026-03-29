//! Action trait definition for AutoHarness
//!
//! The Action trait represents an action that can be taken in an environment.
//! Actions must be serializable for persistence and comparable for deduplication.

use serde::Serialize;
use std::fmt::Display;

use crate::core::error::{HarnessError, Result};

/// Trait representing an action that can be taken in an environment
///
/// Implementors of this trait represent actions that can be executed
/// in an environment (e.g., game moves, code edits, etc.).
///
/// # Type Requirements
///
/// - `Serialize`: Required for serialization to JSON or other formats
/// - `Clone`: Required for creating copies of the action
/// - `Send + Sync`: Required for thread-safe sharing
/// - `PartialEq`: Required for comparing actions
///
/// # Example
///
/// ```rust
/// use autoharness::core::Action;
/// use autoharness::core::error::HarnessError;
/// use serde::Serialize;
///
/// #[derive(Serialize, Clone, PartialEq)]
/// enum GameAction {
///     Move { x: i32, y: i32 },
///     Attack { target: String },
///     Defend,
/// }
///
/// impl Action for GameAction {
///     fn to_string(&self) -> String {
///         match self {
///             GameAction::Move { x, y } => format!("Move to ({}, {})", x, y),
///             GameAction::Attack { target } => format!("Attack {}", target),
///             GameAction::Defend => "Defend".to_string(),
///         }
///     }
///
///     fn from_string(_s: &str) -> Result<Self, HarnessError> {
///         Err(HarnessError::action_parse("Not implemented"))
///     }
/// }
/// ```
pub trait Action: Serialize + Clone + Send + Sync + PartialEq {
    /// Convert the action to a string representation
    ///
    /// This method should produce a human-readable representation of the action
    /// that can be displayed to users or used in LLM prompts.
    ///
    /// # Returns
    ///
    /// A string representation of the action
    fn to_string(&self) -> String;

    /// Parse an action from its string representation
    ///
    /// This method should parse a string representation of an action
    /// and return the corresponding Action instance.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if parsing succeeds
    /// - `Err(HarnessError)` if parsing fails
    fn from_string(s: &str) -> Result<Self>;
}

/// A wrapper type for actions that don't need custom string conversion
///
/// This type can be used to wrap any serializable, cloneable type
/// and provide a default Action implementation using JSON serialization.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BasicAction<T> {
    /// The inner action value
    pub data: T,
}

impl<T> BasicAction<T>
where
    T: Serialize + Clone + Send + Sync + PartialEq,
{
    /// Create a new BasicAction with the given data
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Action for BasicAction<T>
where
    T: Serialize + Clone + Send + Sync + PartialEq + for<'de> serde::Deserialize<'de>,
{
    fn to_string(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_default()
    }

    fn from_string(s: &str) -> Result<Self> {
        let data = serde_json::from_str(s)
            .map_err(|e| HarnessError::action_parse(format!("Failed to parse action: {}", e)))?;
        Ok(Self { data })
    }
}

impl<T> Display for BasicAction<T>
where
    T: Serialize + Clone + Send + Sync + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self.data).unwrap_or_default()
        )
    }
}

/// A collection of actions with utilities for deduplication
#[derive(Debug, Clone)]
pub struct ActionSet<A: Action> {
    actions: Vec<A>,
}

impl<A: Action> ActionSet<A> {
    /// Create a new empty ActionSet
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Create an ActionSet from a vector of actions
    pub fn from_vec(actions: Vec<A>) -> Self {
        Self { actions }
    }

    /// Add an action to the set
    ///
    /// If the action already exists in the set, it will not be added again.
    pub fn add(&mut self, action: A) {
        if !self.actions.contains(&action) {
            self.actions.push(action);
        }
    }

    /// Get all actions in the set
    pub fn actions(&self) -> &[A] {
        &self.actions
    }

    /// Get the number of unique actions in the set
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Clear all actions from the set
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Convert the action set to a vector of strings
    pub fn to_strings(&self) -> Vec<String> {
        self.actions.iter().map(|a| a.to_string()).collect()
    }
}

impl<A: Action> Default for ActionSet<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Action> IntoIterator for ActionSet<A> {
    type Item = A;
    type IntoIter = std::vec::IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

impl<A: Action> From<Vec<A>> for ActionSet<A> {
    fn from(actions: Vec<A>) -> Self {
        let mut set = Self::new();
        for action in actions {
            set.add(action);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Clone, PartialEq, serde::Deserialize)]
    enum TestAction {
        Move { x: i32, y: i32 },
        Attack { target: String },
    }

    impl Action for TestAction {
        fn to_string(&self) -> String {
            match self {
                TestAction::Move { x, y } => format!("Move to ({}, {})", x, y),
                TestAction::Attack { target } => format!("Attack {}", target),
            }
        }

        fn from_string(s: &str) -> Result<Self> {
            if s.starts_with("Move to (") {
                let coords: Vec<&str> = s
                    .trim_start_matches("Move to (")
                    .trim_end_matches(')')
                    .split(", ")
                    .collect();
                if coords.len() == 2 {
                    let x = coords[0]
                        .parse()
                        .map_err(|_| HarnessError::action_parse("Invalid x coordinate"))?;
                    let y = coords[1]
                        .parse()
                        .map_err(|_| HarnessError::action_parse("Invalid y coordinate"))?;
                    return Ok(TestAction::Move { x, y });
                }
            }
            Err(HarnessError::action_parse("Unknown action format"))
        }
    }

    #[test]
    fn test_action_to_string() {
        let action = TestAction::Move { x: 1, y: 2 };
        assert_eq!(Action::to_string(&action), "Move to (1, 2)");
    }

    #[test]
    fn test_action_from_string() {
        let action = TestAction::from_string("Move to (1, 2)").unwrap();
        assert_eq!(action, TestAction::Move { x: 1, y: 2 });
    }

    #[test]
    fn test_action_set_deduplication() {
        let mut set = ActionSet::new();
        set.add(TestAction::Move { x: 1, y: 2 });
        set.add(TestAction::Move { x: 1, y: 2 });
        set.add(TestAction::Attack {
            target: "enemy".to_string(),
        });

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_basic_action_roundtrip() {
        #[derive(Debug, Serialize, Clone, PartialEq, serde::Deserialize)]
        struct SimpleAction {
            name: String,
            value: i32,
        }

        let action = BasicAction::new(SimpleAction {
            name: "test".to_string(),
            value: 42,
        });

        let serialized = Action::to_string(&action);
        let deserialized: BasicAction<SimpleAction> = Action::from_string(&serialized).unwrap();

        assert_eq!(action.data.name, deserialized.data.name);
        assert_eq!(action.data.value, deserialized.data.value);
    }
}
