# AutoHarness

**为 LLM Agent 自动合成代码测试工具 (Harness)**

AutoHarness 是一个 Rust 库，能够自动为 LLM Agent 生成和优化代码测试工具（Harness），采用了 [AutoHarness 论文](https://arxiv.org/abs/2603.03329) 中描述的方法。它使用树搜索结合 Thompson 采样来迭代优化 Harness 代码，平均仅需 14.5 次迭代即可达到 100% 合法行动率。

## 🚀 一句话快速开始

> **直接复制下面的句子到 OpenCode 或 CloudCode：**

```
Now use AutoHarness CLI (https://github.com/gyc567/AutoHarness) to design a Harness engineering system for this project.
```

> **或中文版本：**

```
现在用 AutoHarness 这个 CLI: https://github.com/gyc567/AutoHarness 对本项目进行设计 Harness 工程系统。
```

## ⚡ 一键安装

```bash
# 一行命令安装（推荐）
curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash

# 验证安装
autoharness --version
```

### 支持的平台

| 操作系统 | 架构 | 状态 |
|---------|------|------|
| macOS | Intel / Apple Silicon | ✅ 可用 |
| Linux | x86_64 | 🔨 需源码编译 |
| Windows | x86_64 | 🔨 需源码编译 |

## 🎯 核心特性

- **三种 Harness 模式**：Filter、Verifier 和 Policy
- **树搜索 + Thompson 采样**：高效探索代码空间
- **沙箱执行**：安全的代码执行，资源受限
- **自适应优化**：自动平衡探索与利用
- **高性能**：平均仅需 14.5 次迭代即可收敛

## 📦 安装为 Rust 依赖

添加到您的 `Cargo.toml`：

```toml
[dependencies]
autoharness = "0.1.0"
```

## 🚀 快速开始

### 基本用法

```rust
use autoharness::core::{State, Action, Harness, HarnessType};
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig};

let config = SynthesisConfig::new()
    .with_max_iterations(20)
    .with_convergence_threshold(0.95);

let mut engine = CodeSynthesisEngine::new(config);
let result = engine.synthesize(initial_code, &evaluator)?;
```

### CLI 用法

```bash
# 合成代码
autoharness synthesize --code "fn test() {}"

# 查看帮助
autoharness --help
```

## 🤖 GOAL.md - 自主改进

本项目使用 [GOAL.md](docs/goal-md/GOAL-md-融合方案.md) 模式实现代码自主改进。

### 快速查看分数

```bash
./scripts/score.sh
```

### 当前分数

```
AutoHarness 代码质量: 100 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 15 / 15 ✓
├── maintenance : 20 / 20 ✓
└── safety      :  7 / 10 ✓
```

### 中文教程

- [5 分钟快速开始](docs/goal-md/tutorial-cn/01-quick-start.md)
- [完整教程索引](docs/goal-md/tutorial-cn/README.md)

### 关键文件

| 文件 | 说明 |
|------|------|
| [GOAL.md](GOAL.md) | 项目改进目标 |
| [CLAUDE.md](CLAUDE.md) | Agent 指南 |
| [template/GOAL.md](template/GOAL.md) | GOAL.md 模板 |
| [docs/goal-md/tutorial-cn/](docs/goal-md/tutorial-cn/) | 中文教程 |

## 🧪 测试

```bash
cargo test
cargo test test_synthesis
cargo test test_sandbox
```

## 📊 性能

基于 AutoHarness 论文：

- **平均收敛迭代次数**：14.5
- **合法行动率**：100%（145 场 TextArena 游戏）
- **性能提升**：小模型 + Harness > 大模型无 Harness

## 🔒 安全

AutoHarness 实现了多项安全措施：

1. **沙箱执行**：所有生成的代码在隔离进程中运行
2. **资源限制**：内存、CPU 和文件描述符限制
3. **系统调用过滤**：仅允许必要的系统调用
4. **超时强制**：超过时间的进程将被终止
5. **输入验证**：代码执行前进行验证

## 📚 核心 API

### CodeSynthesisEngine

```rust
pub struct CodeSynthesisEngine {
    tree: SearchTree,
    config: SynthesisConfig,
    stats: SynthesisStats,
}

impl CodeSynthesisEngine {
    pub fn new(config: SynthesisConfig) -> Self;
    pub fn synthesize(&mut self, initial_code: &str, evaluator: &dyn Evaluator) 
        -> Result<String, SynthesisError>;
    pub fn get_best_code(&self) -> Option<&CodeNode>;
}
```

### SynthesisConfig

```rust
pub struct SynthesisConfig {
    pub max_iterations: u32,           // 默认: 50
    pub convergence_threshold: f64,    // 默认: 0.95
    pub max_depth: u32,               // 默认: 10
    pub mutations_per_node: usize,     // 默认: 3
    pub exploration_constant: f64,    // 默认: 1.414
    pub adaptive_sampling: bool,       // 默认: true
}
```

### SandboxExecutor

```rust
pub struct SandboxExecutor {
    config: SandboxConfig,
}

impl SandboxExecutor {
    pub fn new(config: SandboxConfig) -> Result<Self, SandboxError>;
    pub async fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError>;
}
```

## 🔧 配置示例

```rust
use autoharness::engine::SynthesisConfig;

let config = SynthesisConfig::new()
    .with_max_iterations(50)
    .with_convergence_threshold(0.99)
    .with_max_depth(15)
    .with_mutations_per_node(5)
    .with_exploration_constant(2.0)
    .with_adaptive_sampling(true);
```

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [AutoHarness 论文](https://arxiv.org/abs/2603.03329) - Xinghua Lou 等
- [TextArena](https://github.com/google-deepmind/arena) - 游戏环境
- [Thompson Sampling](https://en.wikipedia.org/wiki/Thompson_sampling) - 探索策略
