use autoharness::core::error::HarnessError;
use autoharness::core::{Action, State};
use autoharness::engine::synthesis::{
    CachedEvaluator, CodeSynthesisEngine, Evaluator, ParallelEvaluator, SimpleEvaluator,
    SynthesisConfig,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct TestState {
    value: i32,
    name: String,
}

impl State for TestState {
    fn to_prompt(&self) -> String {
        format!("State: {} ({})", self.name, self.value)
    }

    fn validate(&self) -> autoharness::core::Result<()> {
        if self.value < 0 {
            return Err(HarnessError::state_validation("value must be non-negative"));
        }
        Ok(())
    }
}

#[derive(Serialize, Clone, PartialEq, Deserialize)]
struct TestAction {
    move_x: i32,
    move_y: i32,
}

impl Action for TestAction {
    fn to_string(&self) -> String {
        format!("move({}, {})", self.move_x, self.move_y)
    }

    fn from_string(s: &str) -> autoharness::core::Result<Self> {
        let s = s.trim_start_matches("move(").trim_end_matches(')');
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 2 {
            return Err(HarnessError::action_parse("invalid action format"));
        }
        Ok(TestAction {
            move_x: parts[0]
                .parse()
                .map_err(|_| HarnessError::action_parse("invalid x"))?,
            move_y: parts[1]
                .parse()
                .map_err(|_| HarnessError::action_parse("invalid y"))?,
        })
    }
}

#[test]
fn test_state_serialization() {
    let state = TestState {
        value: 42,
        name: "test".to_string(),
    };

    let prompt = state.to_prompt();
    assert!(prompt.contains("test"));
    assert!(prompt.contains("42"));

    assert!(state.validate().is_ok());
}

#[test]
fn test_state_validation_fail() {
    let state = TestState {
        value: -1,
        name: "negative".to_string(),
    };

    assert!(state.validate().is_err());
}

#[test]
fn test_action_serialization() {
    let action = TestAction {
        move_x: 1,
        move_y: 2,
    };
    let s = action.to_string();
    assert_eq!(s, "move(1, 2)");

    let parsed = TestAction::from_string("move(3, 4)").unwrap();
    assert_eq!(parsed.move_x, 3);
    assert_eq!(parsed.move_y, 4);
}

#[test]
fn test_action_parse_error() {
    let result = TestAction::from_string("invalid");
    assert!(result.is_err());
}

#[test]
fn test_synthesis_config_builder() {
    let config = SynthesisConfig::new()
        .with_max_iterations(100)
        .with_convergence_threshold(0.99)
        .with_max_depth(15)
        .with_mutations_per_node(5)
        .with_adaptive_sampling(false);

    assert_eq!(config.max_iterations, 100);
    assert_eq!(config.convergence_threshold, 0.99);
    assert_eq!(config.max_depth, 15);
    assert_eq!(config.mutations_per_node, 5);
    assert!(!config.adaptive_sampling);
}

#[test]
fn test_synthesis_convergence_early() {
    let config = SynthesisConfig::new()
        .with_max_iterations(10)
        .with_convergence_threshold(0.99);

    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = SimpleEvaluator::new();

    let result = engine.synthesize("fn test() { 1 + 1 }", &evaluator, None);
    assert!(result.is_ok());
}

#[test]
fn test_engine_stats() {
    let config = SynthesisConfig::new().with_max_iterations(5);
    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = SimpleEvaluator::new();

    let _ = engine.synthesize("fn main() {}", &evaluator, None);
    let stats = engine.stats();

    assert!(stats.nodes_explored > 0);
    assert!(stats.best_score >= 0.0);
}

#[test]
fn test_cached_evaluator() {
    let inner = SimpleEvaluator::new();
    let cached = CachedEvaluator::new(inner, 100);

    let code = "fn test() { 1 }";
    let score1 = cached.evaluate(code).unwrap();
    let score2 = cached.evaluate(code).unwrap();

    assert_eq!(score1, score2);
    assert_eq!(cached.cache_size(), 1);
}

#[test]
fn test_parallel_evaluator_batch() {
    let inner = SimpleEvaluator::new();
    let parallel = ParallelEvaluator::new(inner, 4);

    let codes = vec![
        "fn a() {}".to_string(),
        "fn b() {}".to_string(),
        "fn c() {}".to_string(),
    ];

    let results = parallel.evaluate_batch(&codes);
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.is_ok());
    }
}

#[test]
fn test_evaluator_is_valid() {
    let evaluator = SimpleEvaluator::new();

    assert!(evaluator.is_valid("fn test() {}"));
    assert!(!evaluator.is_valid(""));
}

#[test]
fn test_synthesis_reset() {
    let config = SynthesisConfig::new()
        .with_convergence_threshold(0.99)
        .with_max_iterations(10);
    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = SimpleEvaluator::new();

    let _ = engine.synthesize("fn test() {}", &evaluator, None);
    let initial_iterations = engine.iteration();

    if initial_iterations > 0 {
        engine.reset();
        assert_eq!(engine.iteration(), 0);
        assert!(!engine.has_converged());
    }
}
