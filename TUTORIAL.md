# AutoHarness Complete Tutorial

**A comprehensive guide from installation to advanced usage**

---

## Table of Contents

1. [What is AutoHarness?](#1-what-is-autoharness)
2. [Installation](#2-installation)
3. [Basic Concepts](#3-basic-concepts)
4. [Quick Start](#4-quick-start)
5. [CLI Commands Reference](#5-cli-commands-reference)
6. [Programming API](#6-programming-api)
7. [Harness Templates](#7-harness-templates)
8. [Advanced Features](#8-advanced-features)
9. [Best Practices](#9-best-practices)
10. [FAQ](#10-faq)

---

## 1. What is AutoHarness?

AutoHarness is a Rust library and CLI tool that automatically generates and optimizes **code harnesses** for LLM (Large Language Model) agents. 

### What is a "Harness"?

A harness is a piece of code that:
- **Filters**: Validates whether an LLM's output is valid/executable
- **Verifies**: Checks if the output meets certain conditions
- **Proposes**: Suggests actions based on the current state
- **Policies**: Defines strategies for the agent to follow

### Why Use AutoHarness?

- **Automated Optimization**: Uses tree search with Thompson sampling to iteratively improve harness code
- **Average 14.5 iterations** to reach 100% legal action rate
- **Secure Execution**: Runs generated code in sandboxed environments with resource limits
- **Multiple Harness Types**: Filter, Verifier, Policy, and more

### Use Cases

1. **Game AI**: Generate harnesses that validate legal moves in games
2. **Code Generation**: Create harnesses that verify LLM-generated code is valid
3. **Agent Systems**: Build policy harnesses that guide agent behavior
4. **Testing**: Automatically generate test case harnesses

---

## 2. Installation

### Method 1: One-Click Install (Recommended)

```bash
# Using raw GitHub URL
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# Or using jsDelivr CDN (faster)
curl -fsSL https://cdn.jsdelivr.net/gh/gyc567/AutoHarness@main/install/install.sh | bash
```

### Method 2: Clone and Install

```bash
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness/install
chmod +x install.sh
./install.sh
```

### Method 3: Build from Source

```bash
# Ensure you have Rust installed
cargo build --release

# Install the binary
cargo install --path .
```

### Verify Installation

```bash
autoharness --version
```

Expected output:
```
autoharness 0.1.0
```

### Adding to PATH

If `autoharness` is not found, add it to your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"

# Add to ~/.bashrc or ~/.zshrc to persist
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

---

## 3. Basic Concepts

### 3.1 Core Traits

AutoHarness defines three core traits that you need to implement:

#### State Trait

Represents the current state of an environment.

```rust
pub trait State: Serialize + Clone + Send + Sync {
    fn to_prompt(&self) -> String;
    fn validate(&self) -> Result<()>;
}
```

- `to_prompt()`: Convert state to string for LLM
- `validate()`: Validate state consistency

#### Action Trait

Represents actions that can be taken.

```rust
pub trait Action: Serialize + Clone + Send + Sync + PartialEq {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Result<Self>;
}
```

- `to_string()`: Convert action to string
- `from_string()`: Parse action from string

#### Harness Trait

Core interface for all harness types.

```rust
pub trait Harness<S: State, A: Action>: Send + Sync {
    fn harness_type(&self) -> HarnessType;
    fn evaluate(&self, state: &S, action: &A) -> Result<bool>;
    fn propose_actions(&self, state: &S) -> Result<Vec<A>>;
}
```

### 3.2 Harness Types

AutoHarness supports multiple harness types:

| Type | Description | Use Case |
|------|-------------|----------|
| **Filter** | Filters invalid actions | Validate LLM outputs |
| **Verifier** | Verifies conditions | Check code correctness |
| **Policy** | Defines action strategies | Guide agent behavior |
| **Ensemble** | Combines multiple harnesses | Complex scenarios |
| **Adaptive** | Self-adjusting harnesses | Dynamic environments |
| **Critic** | Evaluates action quality | Score-based selection |

### 3.3 Tree Search + Thompson Sampling

AutoHarness uses:
- **Tree Search**: Explores code variations systematically
- **Thompson Sampling**: Balances exploration vs exploitation
- **Adaptive Optimization**: Self-adjusts based on feedback

---

## 4. Quick Start

### 4.1 Using the CLI

#### Step 1: Synthesize Your First Harness

```bash
# Create a simple harness file
cat > my_harness.py << 'EOF'
def is_legal_action(state, action):
    # TODO: Implement validation logic
    return True

def propose_action(state):
    return ["MoveUp", "MoveDown", "MoveLeft", "MoveRight"]
EOF

# Synthesize optimized harness
autoharness synthesize --file my_harness.py --max-iterations 20
```

#### Step 2: Evaluate Harness Quality

```bash
autoharness evaluate --file my_harness.py --detailed
```

#### Step 3: Run in Sandbox

```bash
autoharness run --file my_harness.py --input "test_state"
```

### 4.2 Using the Programming API

Create a new Rust project:

```bash
cargo new my_harness_project
cd my_harness_project
```

Add dependency:

```toml
# Cargo.toml
[dependencies]
autoharness = "0.1.0"
```

Write your harness code:

```rust
// src/main.rs
use autoharness::core::{Action, HarnessError, State};
use autoharness::engine::{CodeSynthesisEngine, Evaluator, SynthesisConfig};
use serde::{Deserialize, Serialize};

// Define your game state
#[derive(Debug, Clone, Serialize)]
struct GameState {
    board: Vec<Vec<i32>>,
    score: i32,
}

impl State for GameState {
    fn to_prompt(&self) -> String {
        format!("Board: {:?}, Score: {}", self.board, self.score)
    }

    fn validate(&self) -> autoharness::core::Result<()> {
        Ok(())
    }
}

// Define your actions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum GameAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Action for GameAction {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn from_string(s: &str) -> autoharness::core::Result<Self> {
        match s {
            "MoveUp" => Ok(GameAction::MoveUp),
            "MoveDown" => Ok(GameAction::MoveDown),
            "MoveLeft" => Ok(GameAction::MoveLeft),
            "MoveRight" => Ok(GameAction::MoveRight),
            _ => Err(HarnessError::action_parse("Unknown action")),
        }
    }
}

// Create a custom evaluator
struct GameEvaluator;

impl Evaluator for GameEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::core::Result<f64> {
        let mut score = 0.5;

        // Check for key components
        if code.contains("is_legal_action") {
            score += 0.2;
        }
        if code.contains("propose_action") {
            score += 0.15;
        }
        if code.contains("board") && code.contains("action") {
            score += 0.1;
        }

        Ok(score.max(0.0).min(1.0))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure synthesis
    let config = SynthesisConfig::new()
        .with_max_iterations(20)
        .with_convergence_threshold(0.95);

    // Create engine
    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = GameEvaluator;

    // Initial harness code
    let initial_code = r#"
        def is_legal_action(state, action):
            return True

        def propose_action(state):
            return ["MoveUp", "MoveDown"]
    "#;

    // Synthesize optimized harness
    let optimized_code = engine.synthesize(initial_code, &evaluator, None)?;

    println!("Optimized harness:\n{}", optimized_code);
    println!("\nStatistics:");
    println!("  Iterations: {}", engine.iteration());

    Ok(())
}
```

Run it:

```bash
cargo run
```

---

## 5. CLI Commands Reference

### 5.1 synthesize - Synthesize Optimized Harness

```bash
autoharness synthesize [OPTIONS]

Options:
  --code <CODE>          Initial code to optimize (string)
  --file <FILE>          Initial code to optimize (file)
  --max-iterations <N>   Maximum iterations (default: 50)
  --convergence <FLOAT> Convergence threshold 0.0-1.0 (default: 0.95)
  --max-depth <N>       Maximum tree depth (default: 10)
  --stats               Show statistics
  --format <FORMAT>     Output format: text, json (default: text)
```

Example:

```bash
# Synthesize from file
autoharness synthesize --file initial.py --max-iterations 30 --stats

# Synthesize from string
autoharness synthesize --code "def foo(): return 1" --convergence 0.99

# JSON output for scripting
autoharness synthesize --file initial.py --format json
```

### 5.2 evaluate - Evaluate Harness Quality

```bash
autoharness evaluate [OPTIONS]

Options:
  --code <CODE>    Code to evaluate (string)
  --file <FILE>   Code to evaluate (file)
  --detailed      Show detailed analysis
  --format <FORMAT> Output format: text, json (default: text)
```

Example:

```bash
autoharness evaluate --file my_harness.py --detailed
```

Output:

```
Score: 0.8500/1.0
Valid: true

--- Detailed Analysis ---
Braces: 10 open, 10 close - balanced
Parentheses: 5 open, 5 close - balanced
Contains 'fn': true
Length: 256 chars
```

### 5.3 run - Execute in Sandbox

```bash
autoharness run [OPTIONS]

Options:
  --code <CODE>        Code to execute (string)
  --file <FILE>        Code to execute (file)
  --input <INPUT>      Input to pass to code
  --memory-limit <N>  Memory limit in MB (default: 256)
  --time-limit <N>     Time limit in ms (default: 5000)
  --format <FORMAT>    Output format: text, json (default: text)
```

Example:

```bash
autoharness run --file test.py --input "hello" --time-limit 10000
```

### 5.4 benchmark - Run Performance Tests

```bash
autoharness benchmark [OPTIONS]

Options:
  --iterations <N>   Number of iterations (default: 100)
  --output <FILE>    Output file for results
```

Example:

```bash
autoharness benchmark --iterations 1000 --output results.json
```

### 5.5 config - Configuration Management

```bash
autoharness config <ACTION>

Actions:
  show                 Show current configuration
  validate --file <F> Validate configuration file
  init --output <F>   Create default configuration
```

Example:

```bash
# Show current config
autoharness config show

# Create default config file
autoharness config init --output my_config.toml

# Validate config
autoharness config validate --file my_config.toml
```

---

## 6. Programming API

### 6.1 Engine Module

#### CodeSynthesisEngine

Main engine for code synthesis.

```rust
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig, Evaluator};

// Create configuration
let config = SynthesisConfig::new()
    .with_max_iterations(50)
    .with_convergence_threshold(0.95)
    .with_max_depth(10);

// Create engine
let mut engine = CodeSynthesisEngine::new(config);

// Synthesize
let result = engine.synthesize(initial_code, &evaluator, None)?;
```

#### SynthesisConfig

Configuration options:

```rust
pub struct SynthesisConfig {
    pub max_iterations: u32,           // Default: 50
    pub convergence_threshold: f64,    // Default: 0.95
    pub max_depth: u32,                // Default: 10
    pub mutations_per_node: usize,     // Default: 3
    pub exploration_constant: f64,    // Default: 1.414
    pub adaptive_sampling: bool,       // Default: true
    pub target_iterations: u32,        // Default: 20
    pub min_improvement: f64,          // Default: 0.01
    pub max_nodes: usize,              // Default: 1000
}
```

### 6.2 Sandbox Module

#### SandboxExecutor

Execute code securely in sandbox.

```rust
use autoharness::sandbox::{SandboxConfig, SandboxExecutor};

// Create config
let config = SandboxConfig::new()
    .with_memory_limit(256)    // MB
    .with_time_limit(5000);    // ms

// Create executor
let executor = SandboxExecutor::new(config)?;

// Execute
let result = executor.execute(code).await?;
```

#### SandboxConfig

```rust
pub struct SandboxConfig {
    pub memory_limit_mb: u64,          // Default: 256
    pub time_limit_ms: u64,            // Default: 5000
    pub max_file_descriptors: u32,     // Default: 64
    pub max_output_size: usize,        // Default: 10MB
    pub enable_network: bool,          // Default: false
    pub working_directory: Option<PathBuf>,
    pub environment_variables: HashMap<String, String>,
}
```

---

## 7. Harness Templates

### 7.1 Filter Template

Filters invalid actions based on rules.

```rust
use autoharness::templates::FilterTemplate;

let template = FilterTemplate::new();
let code = template.generate(&config)?;
```

Generated code validates actions before execution.

### 7.2 Verifier Template

Verifies conditions are met.

```rust
use autoharness::templates::VerifierTemplate;

let template = VerifierTemplate::new();
let code = template.generate(&config)?;
```

### 7.3 Policy Template

Defines action selection strategy.

```rust
use autoharness::templates::PolicyTemplate;

let template = PolicyTemplate::new();
let code = template.generate(&config)?;
```

### 7.4 Ensemble Template

Combines multiple harnesses.

```rust
use autoharness::templates::EnsembleTemplate;

let template = EnsembleTemplate::new()
    .add_harness(filter_template)
    .add_harness(verifier_template);
let code = template.generate(&config)?;
```

---

## 8. Advanced Features

### 8.1 Configuration File

Create `autoharness.toml`:

```toml
# AutoHarness Configuration

[engine]
max_iterations = 50
convergence_threshold = 0.95
max_depth = 10
mutations_per_node = 3
exploration_constant = 1.414
adaptive_sampling = true

[sandbox]
memory_limit_mb = 256
time_limit_ms = 5000
enable_network = false

[logging]
level = "info"
```

Use it:

```bash
autoharness synthesize --file code.py
```

### 8.2 Custom Evaluator

Implement your own evaluation logic:

```rust
struct MyEvaluator;

impl Evaluator for MyEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::core::Result<f64> {
        // Your custom logic here
        let mut score = 0.0;

        // Check for specific patterns
        if code.contains("fn ") {
            score += 0.3;
        }
        if code.contains("match ") {
            score += 0.2;
        }
        // ... more checks

        Ok(score)
    }
}
```

### 8.3 Memory System

AutoHarness includes a memory system for persistent storage:

```rust
use autoharness::memory::{MemoryStore, MemoryConfig};

let config = MemoryConfig::default();
let store = MemoryStore::new(config)?;

// Store harness
store.put("my_harness", &harness_code)?;

// Retrieve later
let code = store.get("my_harness")?;
```

### 8.4 Logging

Enable detailed logging:

```bash
autoharness --verbose synthesize --file code.py
```

Or set in code:

```rust
tracing_subscriber::fmt()
    .with_env_filter("debug")
    .init();
```

---

## 9. Best Practices

### 9.1 Writing Good Initial Code

**Good:**
```python
def is_legal_action(state, action):
    # Check bounds
    if action.x < 0 or action.x >= state.width:
        return False
    if action.y < 0 or action.y >= state.height:
        return False
    return True

def propose_action(state):
    # Consider score
    return sorted(state.legal_actions, key=lambda a: a.score, reverse=True)
```

**Bad:**
```python
def is_legal_action(state, action):
    return True  # Too simple, no optimization possible
```

### 9.2 Tuning Parameters

| Scenario | max_iterations | convergence | max_depth |
|----------|---------------|-------------|-----------|
| Quick test | 10 | 0.80 | 5 |
| Production | 50 | 0.95 | 10 |
| Complex logic | 100 | 0.99 | 15 |

### 9.3 Security

- Always run untrusted code in sandbox
- Set appropriate memory/time limits
- Disable network in sandbox when not needed

---

## 10. FAQ

### Q: What is the difference between Filter and Verifier?

**Filter** checks if an action is valid (yes/no), while **Verifier** checks if a condition is met (can return more details).

### Q: How many iterations do I need?

Start with the default (50). If results are poor, increase to 100. For complex logic, 100-200 iterations may be needed.

### Q: Can I use AutoHarness with Python?

Yes! Use the CLI or embed as a library. The generated harnesses can be in any language.

### Q: Is it safe to run generated code?

Yes, always use the sandbox executor with appropriate limits. Never run untrusted code directly.

### Q: How does Thompson sampling work?

It balances exploration (trying new variations) vs exploitation (using known good variations). This leads to faster convergence than random search.

### Q: Where can I learn more?

- [AutoHarness Paper](https://arxiv.org/abs/2603.03329)
- [GitHub Repository](https://github.com/gyc567/AutoHarness)
- Rust documentation: `cargo doc --open`

---

## Quick Reference Card

```bash
# Installation
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# Quick commands
autoharness synthesize --file code.py --stats
autoharness evaluate --file code.py --detailed
autoharness run --file code.py --input "test"
autoharness config show

# Common options
--max-iterations, -i    Max iterations (default: 50)
--convergence, -c       Convergence threshold (default: 0.95)
--max-depth, -d         Max tree depth (default: 10)
--stats                 Show statistics
--verbose, -v           Verbose output
```

---

**Happy Harnessing! 🚀**

For more help, open an issue on GitHub or consult the API documentation.
