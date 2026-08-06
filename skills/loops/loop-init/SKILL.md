---
name: loop-init
description: Initialize loop-engineering scaffolding in a project. Use when user asks to "/loop-init" or wants to add loop support to a new repo.
category: loops
parent: loop-engineering
---

# Loop Init

在项目根目录创建 loop-engineering 方法论层所需的 6 个配置文件。

## 用法

```
/loop-init [--pattern <id>] [--tool grok|claude-code|codex|github-actions]
```

## 创建的文件

| 文件 | 用途 |
|---|---|
| `STATE.md` | 运行态（loop 自动写） |
| `LOOP.md` | 活动 loop 清单（loop 自动写） |
| `loop-budget.md` | Token 配额 + kill switch |
| `loop-run-log.jsonl` | 追加式运行日志 |
| `loop-constraints.md` | 强制约束 |
| `gate.yaml` | 路径黑/白名单 |

## 行为

1. 检查目录是否已有 loop 文件（避免覆盖）
2. 从模板创建缺失文件（见 [integration-plan.md §3.4](../../docs/loop-engineering/integration-plan.md#34-机器文件-schema-与模板)）
3. 创建 `patterns/registry.yaml` 默认 7 个 Pattern
4. 创建 `src/loop/` Rust 模块骨架（如果是 Rust 项目）
5. 创建 `scripts/loop-doctor.sh`
6. 输出 Loop Readiness Score

## 关联文档

- [integration-plan.md §3.1 目录布局](../../docs/loop-engineering/integration-plan.md#31-顶层目录布局增量新增不动现有结构)
- [implementation-roadmap.md §8 SOP](../../docs/loop-engineering/implementation-roadmap.md#8-实施-sopphase-1-开工后第一步)
- 上游等价：`npx @cobusgreyling/loop init .`

## 示例

```bash
# AutoHarness 仓库
cargo run -- loop init .

# 全新空目录
cd /tmp/new-project && /loop-init
```