# AutoHarness 作为 loop-engineering 示例

> **Q1=B** 决策：AutoHarness 作为 [loop-engineering](https://github.com/cobusgreyling/loop-engineering) 方法论在 Rust 项目中的**对外示例项目**。

本文档说明：AutoHarness 是怎么把自己跑成 loop 的。其他 Rust / CLI 项目可按本文档复制配置。

---

## TL;DR

```bash
# 复制 AutoHarness 的 loop 配置到你的项目
cp STATE.md LOOP.md loop-budget.md loop-run-log.jsonl \
   loop-constraints.md gate.yaml \
   /path/to/your-project/

# 注册 7 个 Pattern
mkdir -p /path/to/your-project/patterns
cp patterns/registry.yaml /path/to/your-project/patterns/

# 启用循环评分
cp scripts/loop-doctor.sh scripts/loop-accuracy.sh \
   /path/to/your-project/scripts/

# 检查
cd /path/to/your-project
bash scripts/loop-doctor.sh
```

---

## 5 个要点（AutoHarness 是怎么做的）

### 1. 利用现有 HarnessType（零成本 maker/checker）

AutoHarness 已有 `HarnessType::{Refiner, Verifier, Critic, Ensemble, Adaptive}` 7 个变体。

我们直接把 L2 的 maker/checker 分离映射为：

| 角色 | 类型 | 任务 |
|---|---|---|
| Implementer (maker) | `Refiner` | 产出改进补丁 |
| Gate 1 (checker) | `Verifier` | 检查合法性（valid/invalid） |
| Gate 2 (checker) | `Critic` | 评分质量 |

**为什么这是杀手锏**：loop-engineering 上游需要 spawn 子 agent / 换 model context。我们直接用 Rust 类型系统，无需新依赖。

### 2. 复用 score.sh 作 fitness（不引入新 fitness）

AutoHarness 已有 `scripts/score.sh`（6 维 code fitness，110 封顶 100）。我们**不新建** fitness function，直接调用：

```bash
bash scripts/score.sh --json
```

输出 JSON 被 loop-triage skill 解析，写入 STATE.md。

### 3. 双轨 Fitness（score.sh vs loop-doctor.sh）

| 维度 | score.sh | loop-doctor.sh |
|---|---|---|
| 答的问题 | 代码质量如何 | loop 系统能否跑起来 |
| 满分 | 100（110 封顶） | 100（10 维） |
| 频率 | 每次 commit | 每天 |

**两个都 100，loop 才算"自进化"成熟**。

### 4. 用 Rust-native 子命令（不复制 npm CLI）

loop-engineering 上游用 `npx @cobusgreyling/loop-*` 工具分发。我们用 `cargo run -- loop-*`：

```bash
cargo run -- loop init .
cargo run -- loop doctor .
cargo run -- loop run --pattern improvement-loop --level L1
cargo run -- loop gate check --paths "Cargo.lock,src/core/harness.rs"
```

9 个子命令 → 9 个 `src/loop/` 模块：

| 子命令 | 模块 |
|---|---|
| `loop init` / `loop status` | `state.rs` + `constraints.rs` |
| `loop doctor` | `audit.rs` |
| `loop run` | `runner.rs` |
| `loop cost` | `budget.rs` |
| `loop context --check` | `runner.rs` |
| `loop gate check` | `gate.rs` |
| `loop worktree create` | `worktree.rs` |
| `loop sync` | `state.rs` |

### 5. Q3=A 数据驱动 L2 解锁

L2 unlock 不靠"我觉得够了"——靠 4 门：

1. L1 跑满 14 天
2. 累计 ≥ 10 次 L1 runs
3. Accuracy ≥ 80%（基于 `STATE.md` 的 `## Accuracy Tracking` 段）
4. 人类签字 `L2-unlock-approved: <date> <signer>`

`scripts/loop-accuracy.sh --check` 自动检查 1-3 门。

---

## 与 loop-engineering 上游的差异

| 维度 | 上游 | AutoHarness |
|---|---|---|
| 语言 | TypeScript | Rust |
| 调度 | `/loop` 命令 + Claude cron | GitHub Actions |
| 工具分发 | npm | cargo |
| 评分 | `loop-audit` v1.7（10 维） | `loop-doctor.sh` + `loop-doctor` Rust（10 维） |
| Pattern 目录 | 7 个标准 | 7 个 AutoHarness 定制 |
| Maker/Checker | spawn 子 agent | `HarnessType::Refiner + Verifier + Critic` |
| Kill switch | label + flag | label + STATE.md flag + `pause-all` 标志 |

**核心不变**：5 大原语 + Memory + L0-L3 渐进放量。

---

## 14 天 L1 起步经验（待 Phase 2 启动后回填）

预填：Phase 2 启动后，本节会被回填"前 14 天观察到的实际数据"：
- 命中准确率（accuracy）
- Token 实际开销 vs 预算
- 多 loop 冲突次数
- 信噪比（需人介入 / 总发现项）

---

## 关联文档

- [README.md](../README.md) — 入口
- [integration-plan.md](integration-plan.md) — 主方案
- [patterns-and-levels.md](patterns-and-levels.md) — 7 Pattern + L0-L3
- [implementation-roadmap.md](implementation-roadmap.md) — Phase 1-4 + 文件清单
- [audit-2026-08-06.md](audit-2026-08-06.md) — 全面审计（5 事实错误 + 12 矛盾 + 14 缺口）

---

*最后更新：2026-08-06*