//! State trait definition for AutoHarness
//!
//! The State trait represents the current state of an environment.
//! It provides methods for converting state to prompts and validating state integrity.

use serde::Serialize;

use crate::core::error::Result;

/// Trait representing the state of an environment
///
/// Implementors of this trait represent the current state of an environment
/// (e.g., game state, code editor state, etc.). The state must be serializable
/// for persistence and cloneable for safe sharing across threads.
///
/// # Type Requirements
///
/// - `Serialize`: Required for serialization to JSON or other formats
/// - `Clone`: Required for creating copies of the state
/// - `Send + Sync`: Required for thread-safe sharing
///
/// # Example
///
/// ```rust
/// use autoharness::core::State;
/// use serde::Serialize;
///
/// #[derive(Serialize, Clone)]
/// struct GameState {
///     score: i32,
///     level: u32,
/// }
///
/// impl State for GameState {
///     fn to_prompt(&self) -> String {
///         format!("Score: {}, Level: {}", self.score, self.level)
///     }
///
///     fn validate(&self) -> Result<(), HarnessError> {
///         if self.score < 0 {
///             Err(HarnessError::state_validation("Score cannot be negative"))
///         } else {
///             Ok(())
///         }
///     }
/// }
/// ```
pub trait State: Serialize + Clone + Send + Sync {
    /// Convert the state to a prompt string for LLM consumption
    ///
    /// This method should produce a human-readable representation of the state
    /// that can be used as context for an LLM agent.
    ///
    /// # Returns
    ///
    /// A string representation of the state suitable for LLM prompts
    fn to_prompt(&self) -> String;

    /// Validate the state for integrity and correctness
    ///
    /// This method should check that the state is in a valid configuration.
    /// It is called before actions are proposed or evaluated.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the state is valid
    /// - `Err(HarnessError)` if the state is invalid
    fn validate(&self) -> Result<()>;
}

/// A wrapper type for states that don't need custom validation
///
/// This type can be used to wrap any serializable, cloneable type
/// and provide a default State implementation.
#[derive(Debug, Clone, Serialize)]
pub struct BasicState<T> {
    /// The inner state value
    pub data: T,
    /// Optional prompt formatter
    #[serde(skip)]
    prompt_formatter: Option<fn(&T) -> String>,
}

impl<T> BasicState<T>
where
    T: Serialize + Clone + Send + Sync,
{
    /// Create a new BasicState with the given data
    pub fn new(data: T) -> Self {
        Self {
            data,
            prompt_formatter: None,
        }
    }

    /// Create a new BasicState with a custom prompt formatter
    pub fn with_formatter(data: T, formatter: fn(&T) -> String) -> Self {
        Self {
            data,
            prompt_formatter: Some(formatter),
        }
    }
}

impl<T> State for BasicState<T>
where
    T: Serialize + Clone + Send + Sync,
{
    fn to_prompt(&self) -> String {
        match self.prompt_formatter {
            Some(formatter) => formatter(&self.data),
            None => serde_json::to_string(&self.data).unwrap_or_default(),
        }
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error::HarnessError;

    #[derive(Serialize, Clone)]
    struct TestState {
        value: i32,
    }

    impl State for TestState {
        fn to_prompt(&self) -> String {
            format!("Value: {}", self.value)
        }

        fn validate(&self) -> Result<()> {
            if self.value < 0 {
                Err(HarnessError::state_validation("Value must be non-negative"))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_state_to_prompt() {
        let state = TestState { value: 42 };
        assert_eq!(state.to_prompt(), "Value: 42");
    }

    #[test]
    fn test_state_validation_success() {
        let state = TestState { value: 42 };
        assert!(state.validate().is_ok());
    }

    #[test]
    fn test_state_validation_failure() {
        let state = TestState { value: -1 };
        assert!(state.validate().is_err());
    }

    #[test]
    fn test_basic_state_default_formatter() {
        let state = BasicState::new(TestState { value: 42 });
        let prompt = state.to_prompt();
        assert!(prompt.contains("42"));
    }

    #[test]
    fn test_basic_state_custom_formatter() {
        let state =
            BasicState::with_formatter(TestState { value: 42 }, |s| format!("Custom: {}", s.value));
        assert_eq!(state.to_prompt(), "Custom: 42");
    }
}
