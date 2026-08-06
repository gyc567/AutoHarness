# AutoHarness

**Automatically synthesize code harnesses for LLM agents**

---

> 🤖 **Dogfooding [loop-engineering](https://github.com/cobusgreyling/loop-engineering)** — Loop Ready: 88/100 (R3) · Patterns: 7 · 详见 [`docs/loop-engineering/`](docs/loop-engineering/README.md)

---

## 📚 Documentation

### 🌍 Languages / 语言

| Language | Description |
|----------|-------------|
| [English](README.md) | Main documentation |
| [中文](README_zh-CN.md) | 中文文档 / Chinese docs |

### 🚀 Quick Start

| Guide | Description |
|-------|-------------|
| [5-Minute Quick Start](docs/goal-md/tutorial/01-quick-start.md) | Get started in 5 minutes |
| [中文快速开始](docs/goal-md/tutorial-cn/01-quick-start.md) | 5 分钟快速开始 |

### 📖 GOAL.md Tutorials (Complete Guide)

#### English Tutorials

| Document | Description |
|----------|-------------|
| [Overview](docs/goal-md/tutorial/00-overview.md) | GOAL.md concept introduction |
| [Fitness Function](docs/goal-md/tutorial/02a-fitness-function.md) | Scoring script design |
| [Action Catalog](docs/goal-md/tutorial/02b-action-catalog.md) | Action catalog design |
| [Create Your First GOAL.md](docs/goal-md/tutorial/03-create-goal.md) | Complete example |
| [Multi-Agent Collaboration](docs/goal-md/tutorial/04-multi-agent.md) | Team collaboration |
| [Advanced Patterns](docs/goal-md/tutorial/05-advanced-patterns-1.md) | Advanced techniques |
| [Troubleshooting](docs/goal-md/tutorial/06-troubleshooting.md) | FAQ and solutions |

### 🤖 Skills Tutorials (Agent Capabilities)

| Document | Description |
|----------|-------------|
| [Overview](docs/skills/tutorial/00-overview.md) | Skills overview |
| [Installation](docs/skills/tutorial/01-installation.md) | Setup guide |
| [Setup GOAL.md](docs/skills/tutorial/02-setup-goal.md) | Initialize system |
| [Score Check](docs/skills/tutorial/03-score-check.md) | Check score |
| [Examples](docs/skills/tutorial/05-examples.md) | Real examples |

#### 中文教程

| 文档 | 说明 |
|------|------|
| [概述](docs/skills/tutorial-cn/00-overview.md) | Skills 概述 |
| [安装](docs/skills/tutorial-cn/01-installation.md) | 安装指南 |
| [示例](docs/skills/tutorial-cn/05-examples.md) | 使用示例 |
| [GOAL.md 概述](docs/goal-md/tutorial-cn/00-overview.md) | GOAL.md 概念介绍 |
| [适应度函数](docs/goal-md/tutorial-cn/02a-fitness-function.md) | 评分脚本设计 |
| [行动目录](docs/goal-md/tutorial-cn/02b-action-catalog.md) | 行动目录设计 |
| [创建第一个 GOAL.md](docs/goal-md/tutorial-cn/03-create-goal.md) | 完整示例 |
| [多 Agent 协作](docs/goal-md/tutorial-cn/04-multi-agent.md) | 团队协作 |
| [进阶模式](docs/goal-md/tutorial-cn/05-advanced-patterns.md) | 高级技巧 |
| [常见问题](docs/goal-md/tutorial-cn/06-troubleshooting.md) | 故障排除 |

### 📁 Reference Files

| File | Description |
|------|-------------|
| [GOAL.md](GOAL.md) | Project improvement goals |
| [CLAUDE.md](CLAUDE.md) | AI Agent instructions |
| [template/GOAL.md](template/GOAL.md) | GOAL.md template |
| [examples/](examples/) | Complete examples |

### 🤖 Claude Code Skills (Agent Capabilities)

Integrate with AI Builder Club's skills ecosystem for enhanced agent capabilities:

| Skill | Description |
|-------|-------------|
| `/setup-goal` | Initialize GOAL.md system for any project |
| `/score-check` | Check current project score |
| `/improvement-loop` | Run one improvement iteration |
| `/new-goal-loop` | Create a new workstream/loop |
| `/context-audit` | Audit agent context quality |
| `/flow-diagram` | Generate flow diagrams |

**Installation:**
```bash
# Clone and use directly
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness/skills

# Or use the skills directory path in your agent
export CLAUDE_SKILLS_DIR="./AutoHarness/skills"
```

See [Skills Integration Plan](docs/goal-md/skills-integration/) for detailed design.

### 📊 Current Score

```bash
./scripts/score.sh
```

```
AutoHarness Code Quality: 100 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 15 / 15 ✓
├── maintenance : 20 / 20 ✓
└── safety      :  7 / 10 ✓
```

---

AutoHarness is a Rust library that automatically generates and optimizing code harnesses for LLM agents, following the approach described in the [AutoHarness paper](https://arxiv.org/abs/2603.03329). It uses tree search with Thompson sampling to iteratively refine harness code, achieving an average of 14.5 iterations to reach 100% legal action rate.

## 🚀 One-Sentence Quick Start (For OpenCode/CloudCode)

> **Copy and paste the sentence below directly into OpenCode or CloudCode to start:**

```
Now use AutoHarness CLI (https://github.com/gyc567/AutoHarness) to design a Harness engineering system for this project.
```

> **Or in Chinese (中文):**

```
现在用 AutoHarness 这个 CLI:https://github.com/gyc567/AutoHarness 对本项目进行设计 Harness 工程系统。
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
