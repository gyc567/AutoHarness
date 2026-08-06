---
name: loop-doctor
description: Run Loop Readiness Score (R0-R3) check. Use when /loop-doctor invoked or to verify loop system health.
category: loops
parent: loop-engineering
---

# Loop Doctor

10 维度评估 Loop Readiness Score（0-100），输出 R0-R3 等级与 Top-3 改进项。

## 用法

```
/loop-doctor [--json]
```

## 评估维度（满分 100）

| 维度 | 满分 | 检查内容 |
|---|---|---|
| LOOP.md | 15 | 文件 + Active Loops 段 |
| STATE.md | 15 | 24h 内更新 |
| loop-budget.md | 10 | 文件存在 |
| loop-run-log.jsonl | 10 | 文件 + 7 天内有记录 |
| loop-constraints.md | 10 | 文件存在 |
| gate.yaml | 10 | YAML 可解析 |
| patterns/registry.yaml | 10 | 文件存在 |
| ≥ 3 次 L1 runs | 10 | log 统计 |
| maker/checker 分离 | 5 | Refiner + Verifier + Critic 引用 |
| kill switch | 5 | STATE.md 有 pause 标志 |

## 等级映射

- **R0** (< 40): 基础设施不完整，禁止 loop run
- **R1** (40-59): 工具就绪，可跑 L1 report-only
- **R2** (60-79): 健康，允许 L2 候选
- **R3** (≥ 80): 成熟，解锁 L3 候选

## 输出

```bash
$ cargo run -- loop doctor
Loop Readiness Score: 88 / 100 (readiness: R3)

Dimensions:
  LOOP.md                    15/15   exists and has Active Loops section
  STATE.md                   15/15   exists and updated within 24h
  loop-budget.md             10/10   exists
  ...

Top actions:
  • Add loop-run-log.jsonl (5 → 10)
```

## 等价命令

```bash
bash scripts/loop-doctor.sh            # 人类可读
bash scripts/loop-doctor.sh --json     # 机器可读
cargo run -- loop doctor               # Rust 实现（更准确）
```

## 关联文档

- [patterns-and-levels.md §3.2](../../docs/loop-engineering/patterns-and-levels.md#32-loop-readiness-score0-100)
- [audit-2026-08-06.md §B2](../../docs/loop-engineering/audit-2026-08-06.md) — 维度数 8 vs 10 已统一为 10
- 上游等价：`npx @cobusgreyling/loop doctor .`