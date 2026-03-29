# AutoHarness 完全教程

**从安装到高级使用的完整指南**

---

## 目录

1. [什么是 AutoHarness？](#1-什么是-autoharness)
2. [安装](#2-安装)
3. [基本概念](#3-基本概念)
4. [快速开始](#4-快速开始)
5. [CLI 命令参考](#5-cli-命令参考)
6. [编程 API](#6-编程-api)
7. [Harness 模板](#7-harness-模板)
8. [高级功能](#8-高级功能)
9. [最佳实践](#9-最佳实践)
10. [常见问题](#10-常见问题)

---

## 1. 什么是 AutoHarness？

AutoHarness 是一个 Rust 库和 CLI 工具，用于自动为 LLM（大语言模型）代理生成和优化**代码 harness**。

### 什么是 "Harness"？

Harness 是一段代码，用于：
- **过滤（Filter）**：验证 LLM 输出是否有效/可执行
- **验证（Verify）**：检查输出是否满足特定条件
- **提议（Propose）**：根据当前状态建议动作
- **策略（Policy）**：定义代理的行为策略

### 为什么要用 AutoHarness？

- **自动化优化**：使用树搜索和 Thompson 采样迭代改进 harness 代码
- **平均 14.5 次迭代**即可达到 100% 合法动作率
- **安全执行**：在沙箱环境中运行生成的代码，带有资源限制
- **多种 Harness 类型**：支持 Filter、Verifier、Policy 等

### 使用场景

1. **游戏 AI**：生成验证游戏中合法移动的 harness
2. **代码生成**：创建验证 LLM 生成代码是否有效的 harness
3. **代理系统**：构建指导代理行为的策略 harness
4. **测试**：自动生成测试用例 harness

---

## 2. 安装

### 方法 1：一键安装（推荐）

```bash
# 使用 raw GitHub URL
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# 或使用 jsDelivr CDN（更快）
curl -fsSL https://cdn.jsdelivr.net/gh/gyc567/AutoHarness@main/install/install.sh | bash
```

### 方法 2：克隆并安装

```bash
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness/install
chmod +x install.sh
./install.sh
```

### 方法 3：从源码构建

```bash
# 确保已安装 Rust
cargo build --release

# 安装二进制文件
cargo install --path .
```

### 验证安装

```bash
autoharness --version
```

预期输出：
```
autoharness 0.1.0
```

### 添加到 PATH

如果找不到 `autoharness`，添加到 PATH：

```bash
export PATH="$HOME/.local/bin:$PATH"

# 持久化到 ~/.bashrc 或 ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

---

## 3. 基本概念

### 3.1 核心 Trait

AutoHarness 定义了三个核心 trait，需要你自己实现：

#### State Trait

表示环境的当前状态。

```rust
pub trait State: Serialize + Clone + Send + Sync {
    fn to_prompt(&self) -> String;
    fn validate(&self) -> Result<()>;
}
```

- `to_prompt()`：将状态转换为字符串供 LLM 使用
- `validate()`：验证状态的一致性

#### Action Trait

表示可以执行的动作。

```rust
pub trait Action: Serialize + Clone + Send + Sync + PartialEq {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Result<Self>;
}
```

- `to_string()`：将动作转换为字符串
- `from_string()`：从字符串解析动作

#### Harness Trait

所有 harness 类型的核心接口。

```rust
pub trait Harness<S: State, A: Action>: Send + Sync {
    fn harness_type(&self) -> HarnessType;
    fn evaluate(&self, state: &S, action: &A) -> Result<bool>;
    fn propose_actions(&self, state: &S) -> Result<Vec<A>>;
}
```

### 3.2 Harness 类型

AutoHarness 支持多种 harness 类型：

| 类型 | 描述 | 使用场景 |
|------|------|----------|
| **Filter** | 过滤无效动作 | 验证 LLM 输出 |
| **Verifier** | 验证条件 | 检查代码正确性 |
| **Policy** | 定义动作策略 | 指导代理行为 |
| **Ensemble** | 组合多个 harness | 复杂场景 |
| **Adaptive** | 自适应 harness | 动态环境 |
| **Critic** | 评估动作质量 | 基于分数的选择 |

### 3.3 树搜索 + Thompson 采样

AutoHarness 使用：
- **树搜索**：系统地探索代码变体
- **Thompson 采样**：平衡探索与利用
- **自适应优化**：根据反馈自我调整

---

## 4. 快速开始

### 4.1 使用 CLI

#### 步骤 1：合成你的第一个 Harness

```bash
# 创建简单的 harness 文件
cat > my_harness.py << 'EOF'
def is_legal_action(state, action):
    # TODO: 实现验证逻辑
    return True

def propose_action(state):
    return ["MoveUp", "MoveDown", "MoveLeft", "MoveRight"]
EOF

# 合成优化后的 harness
autoharness synthesize --file my_harness.py --max-iterations 20
```

#### 步骤 2：评估 Harness 质量

```bash
autoharness evaluate --file my_harness.py --detailed
```

#### 步骤 3：在沙箱中运行

```bash
autoharness run --file my_harness.py --input "test_state"
```

### 4.2 使用编程 API

创建新 Rust 项目：

```bash
cargo new my_harness_project
cd my_harness_project
```

添加依赖：

```toml
# Cargo.toml
[dependencies]
autoharness = "0.1.0"
```

编写你的 harness 代码：

```rust
// src/main.rs
use autoharness::core::{Action, HarnessError, State};
use autoharness::engine::{CodeSynthesisEngine, Evaluator, SynthesisConfig};
use serde::{Deserialize, Serialize};

// 定义你的游戏状态
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

// 定义你的动作
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

// 创建自定义评估器
struct GameEvaluator;

impl Evaluator for GameEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::core::Result<f64> {
        let mut score = 0.5;

        // 检查关键组件
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
    // 配置合成参数
    let config = SynthesisConfig::new()
        .with_max_iterations(20)
        .with_convergence_threshold(0.95);

    // 创建引擎
    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = GameEvaluator;

    // 初始 harness 代码
    let initial_code = r#"
        def is_legal_action(state, action):
            return True

        def propose_action(state):
            return ["MoveUp", "MoveDown"]
    "#;

    // 合成优化后的 harness
    let optimized_code = engine.synthesize(initial_code, &evaluator, None)?;

    println!("优化后的 harness:\n{}", optimized_code);
    println!("\n统计信息:");
    println!("  迭代次数: {}", engine.iteration());

    Ok(())
}
```

运行：

```bash
cargo run
```

---

## 5. CLI 命令参考

### 5.1 synthesize - 合成优化后的 Harness

```bash
autoharness synthesize [OPTIONS]

选项：
  --code <CODE>          初始代码（字符串）
  --file <FILE>          初始代码（文件）
  --max-iterations <N>   最大迭代次数（默认：50）
  --convergence <FLOAT>  收敛阈值 0.0-1.0（默认：0.95）
  --max-depth <N>        最大树深度（默认：10）
  --stats                显示统计信息
  --format <FORMAT>      输出格式：text, json（默认：text）
```

示例：

```bash
# 从文件合成
autoharness synthesize --file initial.py --max-iterations 30 --stats

# 从字符串合成
autoharness synthesize --code "def foo(): return 1" --convergence 0.99

# JSON 输出用于脚本
autoharness synthesize --file initial.py --format json
```

### 5.2 evaluate - 评估 Harness 质量

```bash
autoharness evaluate [OPTIONS]

选项：
  --code <CODE>      要评估的代码（字符串）
  --file <FILE>     要评估的代码（文件）
  --detailed        显示详细分析
  --format <FORMAT> 输出格式：text, json（默认：text）
```

示例：

```bash
autoharness evaluate --file my_harness.py --detailed
```

输出：

```
Score: 0.8500/1.0
Valid: true

--- Detailed Analysis ---
Braces: 10 open, 10 close - balanced
Parentheses: 5 open, 5 close - balanced
Contains 'fn': true
Length: 256 chars
```

### 5.3 run - 在沙箱中执行

```bash
autoharness run [OPTIONS]

选项：
  --code <CODE>        要执行的代码（字符串）
  --file <FILE>        要执行的代码（文件）
  --input <INPUT>      传递给代码的输入
  --memory-limit <N>  内存限制 MB（默认：256）
  --time-limit <N>    时间限制 ms（默认：5000）
  --format <FORMAT>   输出格式：text, json（默认：text）
```

示例：

```bash
autoharness run --file test.py --input "hello" --time-limit 10000
```

### 5.4 benchmark - 运行性能测试

```bash
autoharness benchmark [OPTIONS]

选项：
  --iterations <N>  迭代次数（默认：100）
  --output <FILE>   结果输出文件
```

示例：

```bash
autoharness benchmark --iterations 1000 --output results.json
```

### 5.5 config - 配置管理

```bash
autoharness config <ACTION>

操作：
  show                 显示当前配置
  validate --file <F> 验证配置文件
  init --output <F>   创建默认配置
```

示例：

```bash
# 显示当前配置
autoharness config show

# 创建默认配置文件
autoharness config init --output my_config.toml

# 验证配置
autoharness config validate --file my_config.toml
```

---

## 6. 编程 API

### 6.1 引擎模块

#### CodeSynthesisEngine

代码合成的主引擎。

```rust
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig, Evaluator};

// 创建配置
let config = SynthesisConfig::new()
    .with_max_iterations(50)
    .with_convergence_threshold(0.95)
    .with_max_depth(10);

// 创建引擎
let mut engine = CodeSynthesisEngine::new(config);

// 合成
let result = engine.synthesize(initial_code, &evaluator, None)?;
```

#### SynthesisConfig

配置选项：

```rust
pub struct SynthesisConfig {
    pub max_iterations: u32,           // 默认: 50
    pub convergence_threshold: f64,    // 默认: 0.95
    pub max_depth: u32,                // 默认: 10
    pub mutations_per_node: usize,     // 默认: 3
    pub exploration_constant: f64,      // 默认: 1.414
    pub adaptive_sampling: bool,       // 默认: true
    pub target_iterations: u32,        // 默认: 20
    pub min_improvement: f64,          // 默认: 0.01
    pub max_nodes: usize,              // 默认: 1000
}
```

### 6.2 沙箱模块

#### SandboxExecutor

在沙箱中安全执行代码。

```rust
use autoharness::sandbox::{SandboxConfig, SandboxExecutor};

// 创建配置
let config = SandboxConfig::new()
    .with_memory_limit(256)    // MB
    .with_time_limit(5000);     // ms

// 创建执行器
let executor = SandboxExecutor::new(config)?;

// 执行
let result = executor.execute(code).await?;
```

#### SandboxConfig

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

---

## 7. Harness 模板

### 7.1 Filter 模板

根据规则过滤无效动作。

```rust
use autoharness::templates::FilterTemplate;

let template = FilterTemplate::new();
let code = template.generate(&config)?;
```

生成的代码在执行前验证动作。

### 7.2 Verifier 模板

验证条件是否满足。

```rust
use autoharness::templates::VerifierTemplate;

let template = VerifierTemplate::new();
let code = template.generate(&config)?;
```

### 7.3 Policy 模板

定义动作选择策略。

```rust
use autoharness::templates::PolicyTemplate;

let template = PolicyTemplate::new();
let code = template.generate(&config)?;
```

### 7.4 Ensemble 模板

组合多个 harness。

```rust
use autoharness::templates::EnsembleTemplate;

let template = EnsembleTemplate::new()
    .add_harness(filter_template)
    .add_harness(verifier_template);
let code = template.generate(&config)?;
```

---

## 8. 高级功能

### 8.1 配置文件

创建 `autoharness.toml`：

```toml
# AutoHarness 配置

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

使用：

```bash
autoharness synthesize --file code.py
```

### 8.2 自定义评估器

实现你自己的评估逻辑：

```rust
struct MyEvaluator;

impl Evaluator for MyEvaluator {
    fn evaluate(&self, code: &str) -> autoharness::core::Result<f64> {
        // 在这里写你的自定义逻辑
        let mut score = 0.0;

        // 检查特定模式
        if code.contains("fn ") {
            score += 0.3;
        }
        if code.contains("match ") {
            score += 0.2;
        }
        // ... 更多检查

        Ok(score)
    }
}
```

### 8.3 内存系统

AutoHarness 包含用于持久化存储的内存系统：

```rust
use autoharness::memory::{MemoryStore, MemoryConfig};

let config = MemoryConfig::default();
let store = MemoryStore::new(config)?;

// 存储 harness
store.put("my_harness", &harness_code)?;

// 后续检索
let code = store.get("my_harness")?;
```

### 8.4 日志记录

启用详细日志：

```bash
autoharness --verbose synthesize --file code.py
```

或在代码中设置：

```rust
tracing_subscriber::fmt()
    .with_env_filter("debug")
    .init();
```

---

## 9. 最佳实践

### 9.1 编写好的初始代码

**好的示例：**
```python
def is_legal_action(state, action):
    # 检查边界
    if action.x < 0 or action.x >= state.width:
        return False
    if action.y < 0 or action.y >= state.height:
        return False
    return True

def propose_action(state):
    # 考虑分数
    return sorted(state.legal_actions, key=lambda a: a.score, reverse=True)
```

**差的示例：**
```python
def is_legal_action(state, action):
    return True  # 太简单，无法优化
```

### 9.2 调优参数

| 场景 | max_iterations | convergence | max_depth |
|------|---------------|-------------|-----------|
| 快速测试 | 10 | 0.80 | 5 |
| 生产环境 | 50 | 0.95 | 10 |
| 复杂逻辑 | 100 | 0.99 | 15 |

### 9.3 安全性

- 始终在沙箱中运行不受信任的代码
- 设置适当的内存/时间限制
- 不需要时在沙箱中禁用网络

---

## 10. 常见问题

### Q：Filter 和 Verifier 有什么区别？

**Filter** 检查动作是否有效（是/否），而 **Verifier** 检查条件是否满足（可以返回更多详细信息）。

### Q：我需要多少次迭代？

从默认值（50）开始。如果结果不好，增加到 100。对于复杂逻辑，可能需要 100-200 次迭代。

### Q：AutoHarness 可以和 Python 一起使用吗？

可以！使用 CLI 或嵌入为库。生成的 harness 可以是任何语言。

### Q：运行生成的代码安全吗？

安全，始终使用带适当限制的沙箱执行器。永远不要直接运行不受信任的代码。

### Q：Thompson 采样是如何工作的？

它平衡探索（尝试新变体）与利用（使用已知好的变体）。这比随机搜索收敛更快。

### Q：想了解更多？

- [AutoHarness 论文](https://arxiv.org/abs/2603.03329)
- [GitHub 仓库](https://github.com/gyc567/AutoHarness)
- Rust 文档：`cargo doc --open`

---

## 快速参考卡

```bash
# 安装
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# 快速命令
autoharness synthesize --file code.py --stats
autoharness evaluate --file code.py --detailed
autoharness run --file code.py --input "test"
autoharness config show

# 常用选项
--max-iterations, -i    最大迭代次数（默认：50）
--convergence, -c       收敛阈值（默认：0.95）
--max-depth, -d        最大树深度（默认：10）
--stats                 显示统计信息
--verbose, -v           详细输出
```

---

**祝您使用愉快！🚀**

如需更多帮助，请在 GitHub 上提交问题或查阅 API 文档。
