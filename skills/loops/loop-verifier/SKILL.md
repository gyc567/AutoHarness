---
name: loop-verifier
description: Independent verification of loop-implementer output. Maker/Checker pattern: Refiner produces, Verifier+Critic check. Never let implementer mark its own work done.
category: loops
parent: loop-engineering
---

# Loop Verifier

**检查者**（Checker 双 gate）：在 loop 修改代码后独立验证，不让 implementer 自评。

## 用法

```
/loop-verifier check --run-id <id> [--level L2]
```

## 双 Gate

| Gate | 类型 | 检查内容 |
|---|---|---|
| **Gate 1** | `HarnessType::Verifier` | action 是否合法（valid/invalid） |
| **Gate 2** | `HarnessType::Critic` | action 质量评分（reward / score） |

任何一 gate 失败 → 整体 fail → revert。

## 行为

1. 在隔离 worktree 中跑 `cargo fmt` / `cargo clippy` / `cargo test`
2. 比对 score.sh run-before vs run-after；下降即 fail
3. 检查 diff 命中 denylist
4. 输出 verdict：pass / fail + 详细原因

## 失败处理

- score 下降 → `git revert` 该 commit + 写 loop-run-log.jsonl
- 命中 denylist → halt + escalate
- 测试失败 → 等 flake 规则（同测试 ≥ 2 次才确认是 bug）

## 关联文档

- [integration-plan.md §4.1 Maker/Checker](../../docs/loop-engineering/integration-plan.md#41-makerchecker-的零成本复用)
- [loop-constraints.md §Code](../../loop-constraints.md)
- [patterns-and-levels.md §6 多 Loop 协调](../../docs/loop-engineering/patterns-and-levels.md#6-多-loop-协调矩阵来自-loop-engineering-multi-loopmd)

## 实现

```rust
// src/core/ harness.rs 已有：
// - Refiner: maker (产出补丁)
// - Verifier: gate 1 (valid/invalid)
// - Critic: gate 2 (reward 评分)
//
// Loop Verifier skill 调用 harness 顺序：
Refiner.propose() -> Verifier.check() -> Critic.score() -> verdict
```