# AutoHarness AGENTS.md

**Guidelines for AI agents operating in this repository**

---

## 0. Before You Start (Mandatory)

Before writing any code or modifying any files, you MUST follow this workflow:

### Step 1: Restate Your Understanding

First, explain in your own words:
- **What problem are we solving?** (Deliverables, goals)
- **What is the expected output?**
- **Mark any assumptions** you're making that you're not sure about

### Step 2: Ask Clarifying Questions

Ask **maximum 3 questions at a time** to clarify:
1. **True Goal**: What is the actual goal (not just what's written)?
2. **Constraints**: Technical stack, performance requirements, existing code compatibility, what cannot be changed
3. **Implementation Plan**: Core approach and why this方案

### Step 3: Wait for Confirmation

**DO NOT write any code or modify any files** until you receive explicit confirmation:
> "可以开始" / "can start"

---

## 1. Build, Test & Development Commands

### Core Commands

```bash
# Build the project
cargo build

# Build with all features
cargo build --all-features

# Release build
cargo build --release

# Run the CLI
cargo run -- --help

# Run a specific CLI command
cargo run -- synthesize --code "fn test() {}"
```

### Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_state_serialization
cargo test test_action_parse_error
cargo test test_synthesis_convergence_early

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Run with coverage (requires tarpaulin)
cargo tarpaulin --output --tests
```

### Linting & Formatting

```bash
# Format code
cargo fmt

# Check formatting (without making changes)
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all warnings as errors
cargo clippy -- -D warnings

# Run clippy for workspace
cargo clippy --workspace -- -D warnings
```

### Documentation

```bash
# Build documentation
cargo doc

# Build documentation (open in browser)
cargo doc --open

# Build documentation without dependencies
cargo doc --no-deps
```

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench -- synthesis
```

---

## 2. Code Style Guidelines

### General Principles

- **Rust Edition**: 2021
- **Maximum line length**: 100 characters
- **Indentation**: 4 spaces (not tabs)
- **Trailing commas**: Always use trailing commas in multi-line contexts

### Design Principles

- **移除过时的路径**: 只发布当前代码
- **简单满足需求**: 使用最简单的代码来满足当前需求
- **分层构建**: 先发布最小的工作版本。然后在工作产品基础上添加
- **保持模块分离**: 职责清晰
- **优先成熟库**: 优先选择成熟的库来简化或稳定
- **先检查现有资源**: 先检查现有的依赖、文档和类型
- **长期设计**: 为长期设计
- **研究验证产品**: 研究经过验证的产品。采用它们的模式

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Variables | snake_case | `let max_iterations = 50;` |
| Functions | snake_case | `fn synthesize_code()` |
| Structs | PascalCase | `struct CodeSynthesisEngine` |
| Enums | PascalCase | `enum HarnessType` |
| Enum Variants | PascalCase | `Filter, Verifier, Policy` |
| Constants | SCREAMING_SNAKE_CASE | `const MAX_DEPTH: u32 = 10;` |
| Modules | snake_case | `mod engine;` |
| Traits | PascalCase | `trait State` |
| Types | PascalCase | `type Result<T> = ...` |

### Import Organization

Organize imports in this order with blank lines between groups:

```rust
// 1. Standard library
use std::collections::HashMap;
use std::path::PathBuf;

// 2. External crates
use serde::{Deserialize, Serialize};
use tokio::fs;

// 3. Internal crate modules
use autoharness::core::{Action, State};
use autoharness::engine::SynthesisConfig;

// 4. Module-local
use super::error::Result;
```

### Error Handling

- Use `thiserror` for defining error enums
- Use the `HarnessError` type from `crate::core::error`
- Always return `Result<T>` for fallible operations
- Never use `unwrap()` in production code
- Use `?` operator for error propagation
- Provide meaningful error messages

```rust
// Good
pub fn evaluate(&self, code: &str) -> Result<f64> {
    if code.is_empty() {
        return Err(HarnessError::evaluation("Code cannot be empty"));
    }
    // ...
}

// Bad - don't do this
pub fn evaluate(&self, code: &str) -> f64 {
    code.len() as f64 // No error handling!
}
```

### Type Annotations

- Always annotate function return types
- Use explicit types for public APIs
- Prefer type inference for local variables when obvious

```rust
// Good
pub fn new() -> Self {
    let config = SynthesisConfig::default();
    Self { config }
}

// Good - obvious inference
let items = vec![1, 2, 3]; // Vec<i32> is obvious
```

### Documentation

- Document all public APIs with doc comments (`///`)
- Include examples in doc comments
- Use meaningful descriptions, not just "Creates a new instance"

```rust
/// Creates a new synthesis engine with the given configuration.
///
/// # Arguments
///
/// * `config` - The synthesis configuration
///
/// # Returns
///
/// A new `CodeSynthesisEngine` instance
///
/// # Example
///
/// ```
/// let config = SynthesisConfig::new();
/// let engine = CodeSynthesisEngine::new(config);
/// ```
pub fn new(config: SynthesisConfig) -> Self {
    // ...
}
```

### Testing

- All public functions should have tests
- Follow naming convention: `#[test] fn test_<functionality>_<expected_behavior>()`
- Use descriptive test names
- Test both success and failure cases

```rust
#[test]
fn test_synthesis_config_builder() {
    let config = SynthesisConfig::new()
        .with_max_iterations(100)
        .with_convergence_threshold(0.99);
    
    assert_eq!(config.max_iterations, 100);
    assert_eq!(config.convergence_threshold, 0.99);
}
```

### Module Structure

- One public trait/type per module minimum
- Use `pub mod` for module visibility
- Group related functionality together
- Keep modules focused and cohesive

---

## 3. Development Principles (Mandatory)

**所有代码修复和新功能开发必须遵循以下原则：**

### 1. 保持 KISS 设计原则，保持代码整洁
- **KISS 原则**: Keep code simple and clean
- 代码简洁明了，易于理解和维护
- 避免过度设计和复杂实现
- 优先考虑可读性，而非炫技式代码

### 2. 高内聚，低耦合，用精简的设计模式
- **高内聚 (High Cohesion)**: 相关功能放在同一模块内
- **低耦合 (Low Coupling)**: 模块间依赖最小化
- **精简设计模式**: 只使用必要的设计模式，避免过度工程化
- 遵循单一职责原则：每个模块只做一件事

### 3. 所有新增功能代码都要测试，保证 100% 测试覆盖率
- **TDD 方法**: 先写测试，再写实现
- **覆盖率目标**: 所有新增代码必须达到 100% 测试覆盖
- 使用 `cargo tarpaulin` 验证覆盖率
- 测试用例需覆盖：正常路径、边界条件、错误处理

### 4. 不能影响其他无关的功能
- **隔离原则**: 改动必须不影响其他功能
- **回归测试**: 每次提交前运行完整测试套件
- **最小化改动**: 只改必要的代码，不做大规模重构
- **增量验证**: 每次小改动后立即验证

### 5. 保留所有测试用例代码，同时给出测试报告
- **测试代码永久保留**: 所有测试代码必须提交到仓库
- **测试报告**: 每次功能完成后生成测试报告，包含：
  - 测试用例总数、通过率
  - 覆盖率统计
  - 性能基准对比
  - **报告格式**: 参照 `docs/guides/backtesting.md` 中的模板

### 7. 提案流程
- **新功能**: 使用 **openspec** 生成功能提案
- **Bug 修复**: 使用 **openspec** 生成修复提案
- **任何代码改动**: 必须包含上述所有要求

---

## 4. Code Quality Checklist

Before completing a task, verify:

- [ ] Code is readable with clear naming
- [ ] Functions are concise (< 50 lines)
- [ ] Files are focused (< 800 lines)
- [ ] No deep nesting (> 4 levels)
- [ ] Error handling is complete
- [ ] No println!/print! statements in production
- [ ] No hardcoded values
- [ ] Uses immutable patterns where possible
---

### Code Norms (Ponytail)

Before writing any code, run the **7-level ladder** — stop at the first rung that holds:

1. **Does this need to exist?** YAGNI → skip, say so in one line.
2. **Already in this codebase?** → reuse it.
3. **Stdlib does it?** → use it.
4. **Native platform feature?** → use it (CSS over JS, DB constraint over app code).
5. **Already-installed dependency?** → use it, never add a new one.
6. **Can it be one line?** → one line.
7. **Only then:** the minimum code that works.

**Rules**:
- No unrequested abstractions (no interface with one impl, no factory for one product).
- Deletion over addition. Boring over clever.
- Mark deliberate simplifications with `ponytail:` comment: `// ponytail: global lock, per-account locks if throughput matters`.
- Shortest working diff wins — but only once you understand the problem. The smallest change in the wrong place isn't lazy, it's a second bug.
- Never simplify away: input validation at trust boundaries, error handling that prevents data loss, security measures.

> **Source**: [ponytail](https://github.com/DietrichGebert/ponytail) (MIT, /ponytail skill installed)
> **Invoke**: `/ponytail` activates the ladder for the session; `/ponytail-review` reviews a diff; `/ponytail-audit` audits the whole repo.

---

## 5. Rust Coding Standards (基于 Microsoft Rust Guidelines)

本项目遵循 [Microsoft Rust Guidelines](https://github.com/microsoft/rust-guidelines) 规范。

### 5.1 上游规范参考

在编写 Rust 代码时，应同时遵循以下上游规范：

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html) - API 设计规范
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) - 代码风格指南
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns//intro.html) - 设计模式
- [Rust Reference - Undefined Behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) - 未定义行为

### 5.2 静态验证工具

项目必须使用以下静态验证工具：

| 工具 | 用途 | 命令 |
|------|------|------|
| `rustfmt` | 代码格式化 | `cargo fmt` |
| `clippy` | Lint 检查 | `cargo clippy` |
| `cargo-audit` | 安全漏洞扫描 | `cargo audit` |
| `cargo-hack` | 特性组合验证 | `cargo hack` |
| `cargo-udeps` | 未使用依赖检测 | `cargo udeps` |

### 5.3 Clippy 配置建议

在 `Cargo.toml` 中启用以下 lint 配置：

```toml
[lints.clippy]
cargo = { level = "warn", priority = -1 }
complexity = { level = "warn", priority = -1 }
correctness = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
perf = { level = "warn", priority = -1 }
style = { level = "warn", priority = -1 }
suspicious = { level = "warn", priority = -1 }

# 强制规范
allow_attributes_without_reason = "warn"
clone_on_ref_ptr = "warn"
empty_drop = "warn"
undocumented_unsafe_blocks = "warn"
```

### 5.4 Correctness (正确性)

#### Panic 语义
- **Panic 意味着停止程序**，不是异常处理
- Panic 用于编程错误，不用于错误通信
- 不要假设 panic 会被捕获

```rust
// ✅ 正确：用于编程错误
x.expect("must never happen");

// ❌ 错误：不要用 panic 处理业务错误
if value.is_none() {
    panic!("Value is none"); // 使用 Result/Option 代替
}
```

#### Unsafe 代码
- **必须有正当理由**才能使用 `unsafe`：
  - 新型抽象（如新的智能指针）
  - 性能优化（需基准测试证明）
  - FFI 调用
- **禁止**：
  - 使用 `transmute` 简化安全的 Rust 代码
  - 绕过 `Send`/`Sync` 约束
- **必须**：
  - 附有纯文本安全推理说明
  - 通过 Miri 验证
  - 遵循 [Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)

### 5.5 Performance (性能)

| 规范 | 说明 |
|------|------|
| **M-THROUGHPUT** | 优化热路径，减少分配 |
| **M-HOTPATH** | 热路径避免动态分发 |
| **M-MEM-REUSE** | 优先复用内存而非分配 |
| **M-AVOID-INDIRECTION** | 避免不必要的间接层 |
| **M-INITIAL-CAPACITY** | 预分配容器容量 |
| **M-FAST-HASHER** | 使用快速哈希算法 |

### 5.6 AI 友好的代码规范

为 AI Agent 优化代码可读性：

- **单一路径原则**：每个公开项只通过一条路径可见
- **强类型**：避免原始类型偏执，使用 Newtype 模式
- **全面文档**：为所有模块和公共项提供文档
- **完整示例**：提供可直接使用的示例代码
- **可测试设计**：设计支持单元测试的 API
- **Rust 问题用 Rust 方案**：跨语言移植时不 1:1 复制

### 5.7 Documentation (文档)

| 规范 | 要求 |
|------|------|
| **M-FIRST-DOC-SENTENCE** | 首句简洁，一行概括 |
| **M-MODULE-DOCS** | 模块级文档说明用途 |
| **M-CANONICAL-DOCS** | 文档与代码同步更新 |
| **M-DOC-INLINE** | 内联文档清晰准确 |

---

## 6. Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |

---

## 7. Planning & Documentation System

### 7.1 Planning Documents (PLANS.md & plans/)

**Purpose**:沉淀施工计划，确保每次改动都有据可查。

```bash
# 创建计划文件
plans/<date>_<short-description>.md

# 示例
plans/2026-03-29_frontend_optimization.md
```

**工作流**:
1. **开工前检查**: 必须在 `plans/` 中创建计划文件，并在 `PLANS.md` 建立索引
2. **施工中**: 按照计划执行，记录进度和变更
3. **完工后**: 删除 `plans/xxx.md`，仅保留 Git commit log 作为历史档案

**PLANS.md 索引格式**:
```markdown
# PLANS.md

## 索引

| 日期 | 计划 | 状态 | 关联 Commit |
|------|------|------|-------------|
| 2026-03-29 | 前端性能优化 | 已完成 | abc1234 |
```

> ⚠️ **注意**: 计划文件完工后必须删除，防止文档膨胀。只通过 Git history 追溯。

---

### 7.2 Development Documents (DOCS.md & docs/)

**Purpose**: 在推进开发的过程中自动沉淀文档，积累项目知识。

```bash
# 创建文档文件
docs/<category>/<topic>.md

# 示例层级
docs/
├── architecture/       # 架构文档
│   ├── overview.md
│   └── modules.md
├── api/               # API 文档
│   └── cli.md
├── guides/            # 指南文档
│   └── getting-started.md
└── internals/         # 内部实现
    └── synthesis-engine.md
```

**工作流**:
1. 开发过程中遇到重要决策、技术方案、调试经验时，立即写入 `docs/`
2. 在 `DOCS.md` 建立索引，支持二级甚至三级索引防止单文件膨胀
3. **文档不删除**: 与 PLANS 不同，DOCS 是长期积累，必须保留

**DOCS.md 多级索引格式**:
```markdown
# DOCS.md

## 索引

### 一级分类

#### 二级分类

##### 三级分类
- [主题A](docs/path/to/topic-a.md)
- [主题B](docs/path/to/topic-b.md)

#### 二级分类 B
- [主题C](docs/path/to/topic-c.md)
```

**防止文件膨胀的规则**:
- 单个文档文件建议不超过 500 行
- 超过时自动拆分，子主题建立新文件
- 使用多级索引组织，避免单一 README 无限膨胀

---

### 7.3 Specialized Standards (专项规范索引)

对于有明确反馈的改进领域，必须撰写专项规范并在 AGENTS.md 或 PLANS.md 中建立索引。

**专项规范类别**:

| 领域 | 规范文件 | 索引位置 |
|------|----------|----------|
| 前端优化 | `docs/guides/frontend-optimization.md` | [AGENTS.md - 前端优化规范](#前端优化规范) |
| 回测评估 | `docs/guides/backtesting.md` | [AGENTS.md - 回测评估规范](#回测评估规范) |
| 性能调优 | `docs/guides/performance-tuning.md` | [AGENTS.md - 性能调优规范](#性能调优规范) |
| 代码审查 | `docs/guides/code-review.md` | [AGENTS.md - 代码审查规范](#代码审查规范) |

**规范模板**:
```markdown
# <领域>优化规范

## 目标
明确该领域的优化目标和验收标准。

## 反馈机制
描述如何获取该领域的改进反馈（如浏览器观察、性能指标等）。

## 实践指南
1. 第一步：...
2. 第二步：...
3. 第三步：...

## 验收检查清单
- [ ] 检查点 A
- [ ] 检查点 B
```

> 💡 **提示**: 专项规范需要在上方索引表中注册，便于快速查找。

---

### 7.4 North Star Metrics (北极星指标)

**北极星指标**是衡量项目成功的核心指标，AI 必须按照北极星指标推进工作，确保所有决策都与最终目标对齐。

#### 当前北极星指标

| 指标名称 | 定义 | 测量方法 | 目标值 |
|----------|------|----------|--------|
| **代码合成成功率** | 成功合成的测试用例数 / 总请求数 | 统计 synthesis 命令执行结果 | ≥ 85% |
| **平均合成时间** | 单次合成任务的平均耗时 | Benchmark 测量 | < 500ms |
| **测试覆盖率** | 被测试代码行数 / 总代码行数 | `cargo tarpaulin` | ≥ 80% |
| **CLI 响应时间** | 用户输入到首字符输出的延迟 | 手动/自动化测试 | < 100ms |

#### 使用北极星指标指导工作

1. **决策前**: 该决策是否对北极星指标有正向影响？
2. **执行中**: 当前工作是否在推进北极星指标？
3. **完成后**: 验证北极星指标是否得到改善

#### 指标仪表盘

```bash
# 查看当前指标状态
cargo run -- metrics

# 更新指标记录
cargo run -- metrics --update
```

#### 指标演进

北极星指标不是一成不变的，当项目阶段目标变化时，通过以下流程更新：
1. 在 `docs/architecture/north-star-metrics.md` 记录变更原因
2. 在 `PLANS.md` 中创建变更计划
3. 变更完成后在 Git commit 中标注

---

## 8. Loop-Engineering 操作（2026-08-06 起）

本项目已集成 [loop-engineering](https://github.com/cobusgreyling/loop-engineering) 方法论层。详见 [`docs/loop-engineering/`](docs/loop-engineering/README.md)。

### 关键命令

```bash
# 检查 Loop Readiness Score（10 维 0-100，R0-R3）
cargo run -- loop doctor .
bash scripts/loop-doctor.sh            # bash 等价

# 跑一次 loop（Phase 2 才真正激活；Phase 1 为 mock）
cargo run -- loop run --pattern improvement-loop --level L1

# 路径门禁
cargo run -- loop gate check --paths "Cargo.lock,src/core/harness.rs"

# Accuracy 检查（L2 解锁 4 门）
bash scripts/loop-accuracy.sh --check
```

### 强制约束（任何 AI agent 必须遵守）

- **永不改** `.env` / `Cargo.lock` / `src/core/*` / `src/engine/*` / `src/main.rs`（denylist）
- **永不让** loop 自动 merge 到 main
- **永不让** loop 修改 `GOAL.md` / `PLANS.md` / `DOCS.md`
- **永远**先开 draft PR，等人类 review
- score.sh 分数**不得降低**（0 容忍硬约束）
- 每项 fix 最多 **3 次**尝试，超过 escalate 到 STATE.md Human Inbox

完整约束见 [`loop-constraints.md`](loop-constraints.md) + [`gate.yaml`](gate.yaml)。

### Loop Readiness vs Score 双轨

- `bash scripts/score.sh` — **code quality** fitness（满分 100）
- `bash scripts/loop-doctor.sh` — **loop system** readiness（满分 100）

两个都 100，loop 才算"自进化"成熟。详见 [`docs/loop-engineering/patterns-and-levels.md`](docs/loop-engineering/patterns-and-levels.md) §3.3。

### 当前状态（2026-08-06 Phase 1 完成）

- ✅ 31 个新文件 + 6 个修改
- ✅ Loop Readiness R3（88/100）
- ✅ Code Quality 100/100
- ⏸ **不**激活任何 GitHub Actions workflow（Phase 2 才激活）

---

**Last Updated**: 2026-08-06
**Version**: 0.3.0
