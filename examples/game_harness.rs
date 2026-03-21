use autoharness::core::{Action, HarnessError, State};
use autoharness::engine::{CodeSynthesisEngine, Evaluator, SynthesisConfig, SynthesisError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
struct Game2048State {
    board: [[i32; 4]; 4],
    score: i32,
}

impl State for Game2048State {
    fn to_prompt(&self) -> String {
        let mut prompt = String::from("2048 Game State:\n");
        for row in &self.board {
            for cell in row {
                prompt.push_str(&format!("{:4} ", cell));
            }
            prompt.push('\n');
        }
        prompt.push_str(&format!("Score: {}", self.score));
        prompt
    }

    fn validate(&self) -> autoharness::core::Result<()> {
        if self.score < 0 {
            return Err(HarnessError::state_validation("Score cannot be negative"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum GameAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Action for GameAction {
    fn to_string(&self) -> String {
        match self {
            GameAction::MoveUp => "MoveUp".to_string(),
            GameAction::MoveDown => "MoveDown".to_string(),
            GameAction::MoveLeft => "MoveLeft".to_string(),
            GameAction::MoveRight => "MoveRight".to_string(),
        }
    }

    fn from_string(s: &str) -> autoharness::core::Result<Self> {
        match s {
            "MoveUp" => Ok(GameAction::MoveUp),
            "MoveDown" => Ok(GameAction::MoveDown),
            "MoveLeft" => Ok(GameAction::MoveLeft),
            "MoveRight" => Ok(GameAction::MoveRight),
            _ => Err(HarnessError::action_parse(format!("Unknown action: {}", s))),
        }
    }
}

struct GameHarnessEvaluator;

impl Evaluator for GameHarnessEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::core::Result<f64> {
        let mut score: f64 = 0.5;

        if code.contains("is_legal_action") {
            score += 0.2;
        }

        if code.contains("propose_action") {
            score += 0.15;
        }

        if code.contains("board") && code.contains("action") {
            score += 0.1;
        }

        if code.contains("match") || code.contains("if") {
            score += 0.05;
        }

        Ok(score.max(0.0).min(1.0))
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("AutoHarness Example: Game Harness Synthesis");
    println!("===========================================\n");

    let config = SynthesisConfig::new()
        .with_max_iterations(15)
        .with_convergence_threshold(0.9)
        .with_max_depth(8);

    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = GameHarnessEvaluator;

    let initial_code = r#"
def is_legal_action(state, action):
    return True

def propose_action(state):
    return ["MoveUp", "MoveDown", "MoveLeft", "MoveRight"]
"#;

    println!("Initial harness code:");
    println!("{}\n", initial_code);

    println!("Starting synthesis...");
    let result = engine.synthesize(initial_code, &evaluator);

    match result {
        Ok(optimized_code) => {
            println!("Synthesis completed successfully!");
            println!("\nOptimized harness code:");
            println!("{}", optimized_code);
            println!("\nStatistics:");
            println!("  Iterations: {}", engine.iteration());
            if let Some(best) = engine.get_best_code() {
                println!("  Best score: {:.2}", best.score);
            }
        }
        Err(e) => {
            println!("Synthesis failed: {}", e);
        }
    }

    Ok(())
}
