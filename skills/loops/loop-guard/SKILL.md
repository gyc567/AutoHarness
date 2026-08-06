---
name: loop-guard
description: Enforce loop-constraints.md and gate.yaml. Use before any loop modifies files. Blocks denylist path writes and budget violations.
category: loops
parent: loop-engineering
---

# Loop Guard

**强制约束**。每次 loop 修改文件前**必须**调用。

## 用法

```
/loop-guard check --paths <file1,file2,...>
/loop-guard check --budget <pattern>
```

## 行为

1. 读 `loop-constraints.md` 解析硬约束
2. 读 `gate.yaml` 解析路径黑白名单
4. 检查路径是否在 denylist
5. 检查 token 预算是否足够
6. 检查 score 是否会退化
7. 输出 pass / fail + 原因

## 失败时

- 命中 denylist：halt + escalate 到 STATE.md Human Inbox
- 超预算：自动降级 L1 或 halt（见 [integration-plan.md §6](../../docs/loop-engineering/integration-plan.md)）
- score 退化风险：halt

## 关联文档

- [loop-constraints.md](../../loop-constraints.md)
- [gate.yaml](../../gate.yaml)
- [integration-plan.md §6 安全门禁](../../docs/loop-engineering/integration-plan.md#6-安全门禁)

## 三层写入规则

1. PR 分支内可自动 commit
2. allowlist 路径可自动 merge
3. main **永不**自动 merge

loop-bot 的 L1 报告 push（仅 STATE.md / LOOP.md / loop-run-log.jsonl）是唯一自动 push 例外。