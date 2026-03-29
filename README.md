# AutoHarness

**Automatically synthesize code harnesses for LLM agents**

AutoHarness is a Rust library that automatically generates and optimizing code harnesses for LLM agents, following the approach described in the [AutoHarness paper](https://arxiv.org/abs/2603.03329). It uses tree search with Thompson sampling to iteratively refine harness code, achieving an average of 14.5 iterations to reach 100% legal action rate.

## 🚀 One-Sentence Quick Start (For OpenCode/CloudCode)

> **Copy and paste the sentence below directly into OpenCode or CloudCode to start:**

```
Now use AutoHarness CLI to design a Harness engineering system for this project.
```

> **Or in Chinese (中文):**

```
现在用 AutoHarness 这个 CLI 对本项目进行设计 Harness 工程系统。
```

This will automatically initiate the AutoHarness system to design and generate a Harness engineering system for your project.

## ⚡ Quick Install (One-Click)

```bash
# One-line install (recommended)
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# Or use jsDelivr CDN (faster)
curl -fsSL https://cdn.jsdelivr.net/gh/gyc567/AutoHarness@main/install/install.sh | bash

# Verify
autoharness --version
```

### Alternative: Clone & Install

```bash
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness/install
chmod +x install.sh
./install.sh
```

### Installation Options

| Command | Description |
|---------|-------------|
| `./install.sh` | Install |
| `./install.sh install` | Install (same) |
| `./install.sh uninstall` | Uninstall |
| `./install.sh --help` | Show help |

### Installation Location

- Default: `~/.local/bin/autoharness`
- Add to PATH: `export PATH="$HOME/.local/bin:$PATH"`

### Supported Platforms

| OS | Architecture | Status |
|-----|--------------|--------|
| macOS | Intel (x86_64) | ✅ Available |
| macOS | Apple Silicon (ARM) | ⬅️ Uses x86_64 binary |
| Linux | x86_64 | 🔨 Build from source |
| Windows | x86_64 | 🔨 Build from source |

## 🎯 Key Features

- **Three Harness Modes**: Filter, Verifier, and Policy harnesses
- **Tree Search + Thompson Sampling**: Efficient exploration of code space
- **Sandboxed Execution**: Secure code execution with resource limits
- **Adaptive Optimization**: Self-adjusting exploration vs exploitation
- **High Performance**: Average 14.5 iterations to convergence

## 📦 Installation (Cargo)

Add this to your `Cargo.toml`:

```toml
[dependencies]
autoharness = "0.1.0"
```

## 🚀 Quick Start

### Basic Usage

```rust
use autoharness::core::{State, Action, Harness, HarnessType};
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig, Evaluator};
use autoharness::sandbox::{SandboxExecutor, SandboxConfig};

// Define your state
#[derive(Debug, Clone, serde::Serialize)]
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

// Define your action
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
            _ => Err(autoharness::core::HarnessError::action_parse("Unknown action")),
        }
    }
}

// Create a custom evaluator
struct GameEvaluator;

impl Evaluator for GameEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::engine::Result<f64> {
        // Evaluate the harness code
        // Return a score between 0.0 and 1.0
        if code.contains("is_legal_action") {
            Ok(0.8)
        } else {
            Ok(0.2)
        }
    }
}

// Synthesize a harness
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SynthesisConfig::new()
        .with_max_iterations(20)
        .with_convergence_threshold(0.95);

    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = GameEvaluator;

    let initial_code = r#"
        def is_legal_action(state, action):
            # TODO: Implement validation logic
            return True
    "#;

    let optimized_code = engine.synthesize(initial_code, &evaluator)?;
    println!("Optimized harness:\n{}", optimized_code);

    Ok(())
}
```

## 🏗️ Architecture

### Core Components

```
┌──────────────────────────────────────────────────────────────┐
│                    AutoHarness Architecture                   │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐       │
│  │   Core      │    │   Engine    │    │   Sandbox   │       │
│  │   Module    │    │   Module    │    │   Module    │       │
│  └─────────────┘    └─────────────┘    └─────────────┘       │
│         │                  │                  │               │
│         ▼                  ▼                  ▼               │
│  ┌─────────────────────────────────────────────────────┐     │
│  │              Feedback Module                         │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### Module Overview

- **`core`**: Core data models (State, Action, Harness traits)
- **`engine`**: Code synthesis engine with tree search
- **`sandbox`**: Secure code execution environment
- **`feedback`**: Feedback collection and consolidation

## 📚 API Documentation

### Core Module

#### `State` Trait

Represents the current state of an environment.

```rust
pub trait State: Serialize + Clone + Send + Sync {
    fn to_prompt(&self) -> String;
    fn validate(&self) -> Result<()>;
}
```

#### `Action` Trait

Represents an action that can be taken in an environment.

```rust
pub trait Action: Serialize + Clone + Send + Sync + PartialEq {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Result<Self>;
}
```

#### `Harness` Trait

Core interface for all harness types.

```rust
pub trait Harness<S: State, A: Action>: Send + Sync {
    fn harness_type(&self) -> HarnessType;
    fn evaluate(&self, state: &S, action: &A) -> Result<bool>;
    fn propose_actions(&self, state: &S) -> Result<Vec<A>>;
}
```

### Engine Module

#### `CodeSynthesisEngine`

Main synthesis engine that orchestrates the search process.

```rust
pub struct CodeSynthesisEngine {
    tree: SearchTree,
    config: SynthesisConfig,
    stats: SynthesisStats,
}

impl CodeSynthesisEngine {
    pub fn new(config: SynthesisConfig) -> Self;
    pub fn synthesize(&mut self, initial_code: &str, evaluator: &dyn Evaluator) -> Result<String, SynthesisError>;
    pub fn get_best_code(&self) -> Option<&CodeNode>;
}
```

#### `SynthesisConfig`

Configuration for the synthesis engine.

```rust
pub struct SynthesisConfig {
    pub max_iterations: u32,           // Default: 50
    pub convergence_threshold: f64,    // Default: 0.95
    pub max_depth: u32,                // Default: 10
    pub mutations_per_node: usize,     // Default: 3
    pub exploration_constant: f64,     // Default: 1.414
    pub adaptive_sampling: bool,       // Default: true
    pub target_iterations: u32,        // Default: 20
    pub min_improvement: f64,          // Default: 0.01
    pub max_nodes: usize,              // Default: 1000
}
```

### Sandbox Module

#### `SandboxExecutor`

Secure code execution with resource limits.

```rust
pub struct SandboxExecutor {
    config: SandboxConfig,
}

impl SandboxExecutor {
    pub fn new(config: SandboxConfig) -> Result<Self, SandboxError>;
    pub async fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError>;
    pub async fn execute_with_input(&self, code: &str, input: &str) -> Result<ExecutionResult, SandboxError>;
}
```

#### `SandboxConfig`

Configuration for sandbox execution.

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

## 🔧 Configuration Examples

### Basic Configuration

```rust
use autoharness::engine::SynthesisConfig;

let config = SynthesisConfig::new()
    .with_max_iterations(20)
    .with_convergence_threshold(0.95)
    .with_max_depth(10);
```

### Advanced Configuration

```rust
use autoharness::engine::SynthesisConfig;

let config = SynthesisConfig::new()
    .with_max_iterations(50)
    .with_convergence_threshold(0.99)
    .with_max_depth(15)
    .with_mutations_per_node(5)
    .with_exploration_constant(2.0)
    .with_adaptive_sampling(true)
    .with_target_iterations(30)
    .with_min_improvement(0.005)
    .with_max_nodes(2000);
```

### Sandbox Configuration

```rust
use autoharness::sandbox::SandboxConfig;

let config = SandboxConfig::new()
    .with_memory_limit(512)
    .with_time_limit(10000)
    .with_max_file_descriptors(128)
    .with_max_output_size(20 * 1024 * 1024)  // 20MB
    .with_network(false);
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run specific tests:

```bash
cargo test test_synthesis
cargo test test_sandbox
```

## 📊 Performance

Based on the AutoHarness paper:

- **Average iterations to convergence**: 14.5
- **Legal action rate**: 100% (145 TextArena games)
- **Performance improvement**: Small model + harness > Large model without harness

## 🔒 Security

AutoHarness implements several security measures:

1. **Sandboxed Execution**: All generated code runs in isolated processes
2. **Resource Limits**: Memory, CPU, and file descriptor limits
3. **System Call Filtering**: Only necessary syscalls are allowed
4. **Timeout Enforcement**: Processes are killed if they exceed time limits
5. **Input Validation**: Code is validated before execution

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [AutoHarness Paper](https://arxiv.org/abs/2603.03329) by Xinghua Lou et al.
- [TextArena](https://github.com/google-deepmind/arena) for game environments
- [Thompson Sampling](https://en.wikipedia.org/wiki/Thompson_sampling) for exploration strategy
