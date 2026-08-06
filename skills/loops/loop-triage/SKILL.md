---
name: loop-triage
description: Run a triage scan and write findings to STATE.md. Use when user asks to "/loop-triage" or wants to check loop status.
category: loops
parent: loop-engineering
---

# Loop Triage

读取项目状态（score.sh、STATE.md、loop-run-log.jsonl），生成优先级清单，写入 STATE.md 的对应段。

## 用法

```
/loop-triage [--pattern <id>] [--level L1|L2|L3]
```

## 行为

1. 跑 `bash scripts/score.sh --json`（code quality baseline）
2. 读 `STATE.md` 当前内容
3. 读 `loop-run-log.jsonl` 最近 5 条
4. 找最低分项（按 score.sh components）
6. 生成 finding：
   - 高优先级项 → `## High Priority`
   - 监视项 → `## Watch List`
   - 噪音 → `## Recent Noise`
7. **不修改代码**（L1 默认）

## L1 vs L2

- **L1**：只写 STATE.md；不 commit，不 push
- **L2+**：commit 到 PR 分支；走 `loop-gate check`

## 关联文档

- [integration-plan.md §5 STATE.md 模板](../../docs/loop-engineering/integration-plan.md#5-状态机statemd-vs-goalmd)
- [patterns-and-levels.md §3.1 L0-L3](../../docs/loop-engineering/patterns-and-levels.md#31-等级定义)
- [../improvement-loop/SKILL.md](../improvement-loop/SKILL.md) — /improvement-loop 是 loop-triage 的特例

## 示例

```bash
# 在 repo 根目录
/loop-triage --pattern improvement-loop --level L1
```