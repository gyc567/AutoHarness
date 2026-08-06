---
name: loop-budget
description: Check token budget and decide whether loop can run. Use when /loop-budget is invoked or before a high-cost loop.
category: loops
parent: loop-engineering
---

# Loop Budget

读取 `loop-budget.md` 与 `loop-run-log.jsonl`，判断当前 token 用量，决定 loop 是 Ok / Downgrade / Halt / Red-line。

## 用法

```
/loop-budget [--pattern <id>]
```

## 行为

1. 读 `loop-budget.md` 配额表
2. 统计 `loop-run-log.jsonl` 今日 token 总和
3. 应用 [integration-plan.md §6 budget 规则](../../docs/loop-engineering/integration-plan.md)
4. 输出决策：
   - `ok` — 正常，可继续
   - `downgrade` — 切 L1 report-only
   - `halt` — 立即退出
   - `red-line` — 触发红线（见 [implementation-roadmap.md §3.3](../../docs/loop-engineering/implementation-roadmap.md#33-红线违反立即停-loop)）

## 关联文档

- [loop-budget.md](../../loop-budget.md) — 配额配置文件
- [patterns/registry.yaml](../../patterns/registry.yaml) — 单 Pattern `suggested_daily_cap`

## 示例

```bash
/loop-budget
# Output: budget=ok (82000/150000 tokens today)
/loop-budget --pattern improvement-loop
# Output: budget=downgrade (85000/100000 tokens today, 85%)
```