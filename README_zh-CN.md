# AutoHarness

**自动为 LLM 代理合成代码 harness**

AutoHarness 是一个 Rust 库，用于自动为 LLM 代理生成和优化代码 harness，采用 [AutoHarness 论文](https://arxiv.org/abs/2603.03329) 中描述的方法。它使用树搜索结合 Thompson 采样来迭代优化 harness 代码，平均只需 14.5 次迭代即可达到 100% 合法动作率。

## ⚡ 快速安装（一键安装）

```bash
# 一键安装（推荐）
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# 或使用 jsDelivr CDN（更快）
curl -fsSL https://cdn.jsdelivr.net/gh/gyc567/AutoHarness@main/install/install.sh | bash

# 验证
autoharness --version
```

### 备选方案：克隆并安装

```bash
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness/install
chmod +x install.sh
./install.sh
```

### 安装选项

| 命令 | 描述 |
|-----|------|
| `./install.sh` | 安装 |
| `./install.sh install` | 安装（同上） |
| `./install.sh uninstall` | 卸载 |
| `./install.sh --help` | 查看帮助 |

### 安装位置

- 默认安装到: `~/.local/bin/autoharness`
- 添加到 PATH: `export PATH="$HOME/.local/bin:$PATH"`

### 支持平台

| 操作系统 | 架构 | 状态 |
|---------|------|------|
| macOS | Intel (x86_64) | ✅ 可用 |
| macOS | Apple Silicon (ARM) | ⬅️ 使用 x86_64 兼容版 |
| Linux | x86_64 | 🔨 需自行编译 |
| Windows | x86_64 | 🔨 需自行编译 |

## 主要特性

- **三种 Harness 模式**：过滤器、验证器、策略 harness
- **树搜索 + Thompson 采样**：高效探索代码空间
- **沙箱执行**：安全的代码执行和资源限制
- **自适应优化**：自我调整探索与利用平衡
- **高性能**：平均 14.5 次迭代收敛

## 安装

添加到你的 `Cargo.toml`:

```toml
[dependencies]
autoharness = "0.1.0"
```

## 快速开始

### 基础用法

```rust
use autoharness::core::{State, Action, Harness, HarnessType};
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig, Evaluator};
use autoharness::sandbox::{SandboxExecutor, SandboxConfig};

// 定义你的状态
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

// 定义你的动作
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

// 创建自定义评估器
struct GameEvaluator;

impl Evaluator for GameEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::engine::Result<f64> {
        // 评估 harness 代码
        // 返回 0.0 到 1.0 之间的分数
        if code.contains("is_legal_action") {
            Ok(0.8)
        } else {
            Ok(0.2)
        }
    }
}

// 合成 harness
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SynthesisConfig::new()
        .with_max_iterations(20)
        .with_convergence_threshold(0.95);

    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = GameEvaluator;

    let initial_code = r#"
        def is_legal_action(state, action):
            # TODO: 实现验证逻辑
            return True
    "#;

    let optimized_code = engine.synthesize(initial_code, &evaluator)?;
    println!("优化的 harness:\n{}", optimized_code);

    Ok(())
}
```

## 架构

### 核心组件

```
┌──────────────────────────────────────────────────────────────┐
│                    AutoHarness 架构                           │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐       │
│  │   Core      │    │   Engine    │    │   Sandbox   │       │
│  │   模块      │    │   模块      │    │   模块      │       │
│  └─────────────┘    └─────────────┘    └─────────────┘       │
│         │                  │                  │               │
│         ▼                  ▼                  ▼               │
│  ┌─────────────────────────────────────────────────────┐     │
│  │              Feedback 模块                           │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### 模块概览

- **`core`**: 核心数据模型（State、Action、Harness trait）
- **`engine`**: 带有树搜索的代码合成引擎
- **`sandbox`**: 安全的代码执行环境
- **`feedback`**: 反馈收集与整合

## API 文档

### Core 模块

#### `State` Trait

表示环境的当前状态。

```rust
pub trait State: Serialize + Clone + Send + Sync {
    fn to_prompt(&self) -> String;
    fn validate(&self) -> Result<()>;
}
```

#### `Action` Trait

表示可以在环境中执行的动作。

```rust
pub trait Action: Serialize + Clone + Send + Sync + PartialEq {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Result<Self>;
}
```

#### `Harness` Trait

所有 harness 类型的核心接口。

```rust
pub trait Harness<S: State, A: Action>: Send + Sync {
    fn harness_type(&self) -> HarnessType;
    fn evaluate(&self, state: &S, action: &A) -> Result<bool>;
    fn propose_actions(&self, state: &S) -> Result<Vec<A>>;
}
```

### Engine 模块

#### `CodeSynthesisEngine`

协调搜索过程的主合成引擎。

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

合成引擎的配置。

```rust
pub struct SynthesisConfig {
    pub max_iterations: u32,           // 默认: 50
    pub convergence_threshold: f64,   // 默认: 0.95
    pub max_depth: u32,                // 默认: 10
    pub mutations_per_node: usize,    // 默认: 3
    pub exploration_constant: f64,    // 默认: 1.414
    pub adaptive_sampling: bool,       // 默认: true
    pub target_iterations: u32,        // 默认: 20
    pub min_improvement: f64,          // 默认: 0.01
    pub max_nodes: usize,              // 默认: 1000
}
```

### Sandbox 模块

#### `SandboxExecutor`

带资源限制的安全代码执行。

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

沙箱执行配置。

```rust
pub struct SandboxConfig {
    pub memory_limit_mb: u64,          // 默认: 256
    pub time_limit_ms: u64,            // 默认: 5000
    pub max_file_descriptors: u32,     // 默认: 64
    pub max_output_size: usize,        // 默认: 10MB
    pub enable_network: bool,          // 默认: false
    pub working_directory: Option<PathBuf>,
    pub environment_variables: HashMap<String, String>,
}
```

## 配置示例

### 基础配置

```rust
use autoharness::engine::SynthesisConfig;

let config = SynthesisConfig::new()
    .with_max_iterations(20)
    .with_convergence_threshold(0.95)
    .with_max_depth(10);
```

### 高级配置

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

### 沙箱配置

```rust
use autoharness::sandbox::SandboxConfig;

let config = SandboxConfig::new()
    .with_memory_limit(512)
    .with_time_limit(10000)
    .with_max_file_descriptors(128)
    .with_max_output_size(20 * 1024 * 1024)  // 20MB
    .with_network(false);
```

## 测试

运行测试套件：

```bash
cargo test
```

运行特定测试：

```bash
cargo test test_synthesis
cargo test test_sandbox
```

## 性能

基于 AutoHarness 论文：

- **平均收敛迭代次数**: 14.5
- **合法动作率**: 100%（145 场 TextArena 游戏）
- **性能提升**: 小模型 + harness > 大模型无 harness

## 安全

AutoHarness 实现了多项安全措施：

1. **沙箱执行**：所有生成的代码在隔离进程中运行
2. **资源限制**：内存、CPU 和文件描述符限制
3. **系统调用过滤**：仅允许必要的系统调用
4. **超时强制**：超时的进程将被终止
5. **输入验证**：执行前验证代码

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 许可证

本项目基于 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [AutoHarness 论文](https://arxiv.org/abs/2603.03329) - Xinghua Lou 等
- [TextArena](https://github.com/google-deepmind/arena) - 游戏环境
- [Thompson 采样](https://en.wikipedia.org/wiki/Thompson_sampling) - 探索策略
