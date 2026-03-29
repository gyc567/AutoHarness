//! Harness trait and types for AutoHarness
//!
//! This module defines the core Harness trait and related types.
//! A harness is responsible for evaluating actions and proposing valid actions
//! given a state.

use serde::{Deserialize, Serialize};

use crate::core::action::Action;
use crate::core::error::Result;
use crate::core::state::State;

/// The type of harness, determining its behavior
///
/// Different harness types serve different purposes in the AutoHarness framework:
///
/// - `Filter`: Proposes valid actions, LLM selects from them
/// - `Verifier`: LLM proposes actions, harness verifies them
/// - `Policy`: Pure code policy, no LLM at inference time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarnessType {
    /// Filter harness proposes valid actions for the LLM to select from
    ///
    /// This is useful when the action space is large but many actions
    /// are invalid in the current state. The harness filters out invalid
    /// actions before presenting them to the LLM.
    Filter,

    /// Verifier harness checks actions proposed by the LLM
    ///
    /// This is useful when the LLM can propose arbitrary actions
    /// and the harness needs to verify their validity.
    Verifier,

    /// Policy harness implements a pure code policy
    ///
    /// This is useful when the policy can be fully implemented in code
    /// without needing an LLM at inference time.
    Policy,

    /// Critic harness evaluates and scores proposed actions
    ///
    /// This is useful when you need detailed feedback on action quality
    /// beyond simple valid/invalid decisions.
    Critic,

    /// Refiner harness improves and iterates on existing harness code
    ///
    /// This is useful for self-improving systems that evolve their harnesses
    /// over time based on performance feedback.
    Refiner,

    /// Ensemble harness combines multiple harnesses for robust decision making
    ///
    /// This is useful when you want to leverage multiple perspectives
    /// and reduce variance in decision making.
    Ensemble,

    /// Adaptive harness dynamically adjusts behavior based on performance feedback
    ///
    /// This is useful for environments that change over time, requiring
    /// the harness to adapt its strategy.
    Adaptive,
}

impl HarnessType {
    /// Returns true if this harness type uses an LLM
    pub fn uses_llm(&self) -> bool {
        matches!(
            self,
            HarnessType::Filter
                | HarnessType::Verifier
                | HarnessType::Critic
                | HarnessType::Refiner
                | HarnessType::Ensemble
                | HarnessType::Adaptive
        )
    }

    /// Returns true if this harness type requires action proposals
    pub fn requires_proposals(&self) -> bool {
        matches!(self, HarnessType::Filter)
    }

    /// Returns true if this harness type requires action verification
    pub fn requires_verification(&self) -> bool {
        matches!(self, HarnessType::Verifier | HarnessType::Critic)
    }
}

impl std::fmt::Display for HarnessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HarnessType::Filter => write!(f, "Filter"),
            HarnessType::Verifier => write!(f, "Verifier"),
            HarnessType::Policy => write!(f, "Policy"),
            HarnessType::Critic => write!(f, "Critic"),
            HarnessType::Refiner => write!(f, "Refiner"),
            HarnessType::Ensemble => write!(f, "Ensemble"),
            HarnessType::Adaptive => write!(f, "Adaptive"),
        }
    }
}

/// The core trait for all harness implementations
///
/// A harness is responsible for:
/// - Evaluating whether an action is valid in a given state
/// - Proposing valid actions for a given state (for Filter harnesses)
///
/// # Type Parameters
///
/// - `S`: The state type, must implement the `State` trait
/// - `A`: The action type, must implement the `Action` trait
///
/// # Example
///
/// ```rust
/// use autoharness::core::{Harness, HarnessType, State, Action};
/// use autoharness::core::error::{HarnessError, Result};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Clone)]
/// struct MyState { value: i32 }
///
/// impl State for MyState {
///     fn to_prompt(&self) -> String {
///         format!("State: {}", self.value)
///     }
///     fn validate(&self) -> Result<()> {
///         Ok(())
///     }
/// }
///
/// #[derive(Serialize, Clone, PartialEq, Deserialize)]
/// struct MyAction { move_x: i32 }
///
/// impl Action for MyAction {
///     fn to_string(&self) -> String {
///         format!("{}", self.move_x)
///     }
///     fn from_string(s: &str) -> Result<Self> {
///         Ok(MyAction { move_x: s.parse().unwrap_or(0) })
///     }
/// }
///
/// struct MyHarness;
///
/// impl Harness<MyState, MyAction> for MyHarness {
///     fn harness_type(&self) -> HarnessType {
///         HarnessType::Filter
///     }
///
///     fn evaluate(&self, _state: &MyState, _action: &MyAction) -> Result<bool> {
///         Ok(true)
///     }
///
///     fn propose_actions(&self, _state: &MyState) -> Result<Vec<MyAction>> {
///         Ok(vec![])
///     }
/// }
/// ```
pub trait Harness<S: State, A: Action>: Send + Sync {
    /// Returns the type of this harness
    ///
    /// This determines how the harness will be used in the system.
    fn harness_type(&self) -> HarnessType;

    /// Evaluate whether an action is valid in the given state
    ///
    /// # Arguments
    ///
    /// * `state` - The current state
    /// * `action` - The action to evaluate
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if the action is valid
    /// - `Ok(false)` if the action is invalid
    /// - `Err(HarnessError)` if evaluation fails
    fn evaluate(&self, state: &S, action: &A) -> Result<bool>;

    /// Propose valid actions for the given state
    ///
    /// This method is primarily used by Filter harnesses to provide
    /// a list of valid actions for the LLM to choose from.
    ///
    /// # Arguments
    ///
    /// * `state` - The current state
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<A>)` containing valid actions
    /// - `Err(HarnessError)` if proposal fails
    fn propose_actions(&self, state: &S) -> Result<Vec<A>>;
}

/// A boxed harness type for dynamic dispatch
pub type BoxedHarness<S, A> = Box<dyn Harness<S, A>>;

/// Metadata for a harness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessMetadata {
    /// The name of the harness
    pub name: String,
    /// The type of the harness
    pub harness_type: HarnessType,
    /// Description of what the harness does
    pub description: String,
    /// Version of the harness
    pub version: String,
}

impl HarnessMetadata {
    /// Create new harness metadata
    pub fn new(
        name: impl Into<String>,
        harness_type: HarnessType,
        description: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            harness_type,
            description: description.into(),
            version: version.into(),
        }
    }
}

/// Result of a harness evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    /// Whether the action was valid
    pub valid: bool,
    /// Optional message explaining the result
    pub message: Option<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl EvaluationResult {
    /// Create a new valid result
    pub fn valid() -> Self {
        Self {
            valid: true,
            message: None,
            confidence: 1.0,
        }
    }

    /// Create a new invalid result
    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            message: Some(message.into()),
            confidence: 0.0,
        }
    }

    /// Create a result with confidence
    pub fn with_confidence(valid: bool, confidence: f64) -> Self {
        Self {
            valid,
            message: None,
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    /// Returns true if the evaluation was valid
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// A composite harness that combines multiple harnesses
///
/// This harness evaluates actions against all child harnesses
/// and returns valid only if all harnesses agree.
pub struct CompositeHarness<S: State, A: Action> {
    harnesses: Vec<Box<dyn Harness<S, A>>>,
    harness_type: HarnessType,
}

impl<S: State, A: Action> CompositeHarness<S, A> {
    /// Create a new composite harness
    pub fn new(harness_type: HarnessType) -> Self {
        Self {
            harnesses: Vec::new(),
            harness_type,
        }
    }

    /// Add a harness to the composite
    pub fn add_harness<H: Harness<S, A> + 'static>(&mut self, harness: H) {
        self.harnesses.push(Box::new(harness));
    }

    /// Returns the number of harnesses in the composite
    pub fn len(&self) -> usize {
        self.harnesses.len()
    }

    /// Returns true if the composite is empty
    pub fn is_empty(&self) -> bool {
        self.harnesses.is_empty()
    }
}

impl<S: State, A: Action> Harness<S, A> for CompositeHarness<S, A> {
    fn harness_type(&self) -> HarnessType {
        self.harness_type
    }

    fn evaluate(&self, state: &S, action: &A) -> Result<bool> {
        for harness in &self.harnesses {
            if !harness.evaluate(state, action)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn propose_actions(&self, state: &S) -> Result<Vec<A>> {
        // For composite harnesses, we intersect the proposals from all harnesses
        let mut all_actions: Vec<Vec<A>> = Vec::new();

        for harness in &self.harnesses {
            let actions = harness.propose_actions(state)?;
            all_actions.push(actions);
        }

        if all_actions.is_empty() {
            return Ok(Vec::new());
        }

        // Start with the first set and filter by all others
        let mut result: Vec<A> = all_actions[0].clone();

        for actions in &all_actions[1..] {
            result.retain(|a| actions.contains(a));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::action::Action;
    use crate::core::error::HarnessError;
    use crate::core::state::State;
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    struct TestState {
        value: i32,
    }

    impl State for TestState {
        fn to_prompt(&self) -> String {
            format!("Value: {}", self.value)
        }

        fn validate(&self) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Serialize, Clone, PartialEq, serde::Deserialize)]
    struct TestAction {
        name: String,
    }

    impl Action for TestAction {
        fn to_string(&self) -> String {
            self.name.clone()
        }

        fn from_string(s: &str) -> Result<Self> {
            Ok(Self {
                name: s.to_string(),
            })
        }
    }

    struct AlwaysValidHarness;

    impl Harness<TestState, TestAction> for AlwaysValidHarness {
        fn harness_type(&self) -> HarnessType {
            HarnessType::Filter
        }

        fn evaluate(&self, _state: &TestState, _action: &TestAction) -> Result<bool> {
            Ok(true)
        }

        fn propose_actions(&self, _state: &TestState) -> Result<Vec<TestAction>> {
            Ok(vec![
                TestAction {
                    name: "action1".to_string(),
                },
                TestAction {
                    name: "action2".to_string(),
                },
            ])
        }
    }

    struct AlwaysInvalidHarness;

    impl Harness<TestState, TestAction> for AlwaysInvalidHarness {
        fn harness_type(&self) -> HarnessType {
            HarnessType::Verifier
        }

        fn evaluate(&self, _state: &TestState, _action: &TestAction) -> Result<bool> {
            Ok(false)
        }

        fn propose_actions(&self, _state: &TestState) -> Result<Vec<TestAction>> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn test_harness_type_uses_llm() {
        assert!(HarnessType::Filter.uses_llm());
        assert!(HarnessType::Verifier.uses_llm());
        assert!(!HarnessType::Policy.uses_llm());
    }

    #[test]
    fn test_harness_type_display() {
        assert_eq!(HarnessType::Filter.to_string(), "Filter");
        assert_eq!(HarnessType::Verifier.to_string(), "Verifier");
        assert_eq!(HarnessType::Policy.to_string(), "Policy");
    }

    #[test]
    fn test_evaluation_result() {
        let valid = EvaluationResult::valid();
        assert!(valid.is_valid());
        assert_eq!(valid.confidence, 1.0);

        let invalid = EvaluationResult::invalid("test error");
        assert!(!invalid.is_valid());
        assert_eq!(invalid.message, Some("test error".to_string()));
    }

    #[test]
    fn test_composite_harness_evaluate() {
        let mut composite = CompositeHarness::new(HarnessType::Filter);
        composite.add_harness(AlwaysValidHarness);

        let state = TestState { value: 42 };
        let action = TestAction {
            name: "test".to_string(),
        };

        assert!(composite.evaluate(&state, &action).unwrap());
    }

    #[test]
    fn test_composite_harness_evaluate_invalid() {
        let mut composite = CompositeHarness::new(HarnessType::Filter);
        composite.add_harness(AlwaysValidHarness);
        composite.add_harness(AlwaysInvalidHarness);

        let state = TestState { value: 42 };
        let action = TestAction {
            name: "test".to_string(),
        };

        assert!(!composite.evaluate(&state, &action).unwrap());
    }
}
