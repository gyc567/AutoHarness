---
name: loop-init-minimal-rust
description: Minimal fork-start kit for adding loop-engineering to a Rust project. Use when starting a new project or when user wants minimal config.
category: loops
parent: loop-engineering
---

# Loop Init Minimal Rust

最简版 fork 起点。**仅含** 6 个根目录配置文件 + 一段 SKILL 描述。

## 与 `/loop-init` 的差异

`/loop-init` 创建完整 31 个文件（含 src/loop/ 模块、4 个 GitHub Actions、scripts 等）。
`/loop-init-minimal-rust` 只创建最关键的 6 个配置文件，让其他 Rust 项目能 fork 后渐进补完。

## 适合场景

- 新 Rust 项目想接入 loop-engineering 方法论
- 不想一次性加 31 个文件
- 想按需逐步启用 Pattern

## 必备文件（6 个）

| 文件 | 用途 |
|---|---|
| `STATE.md` | 运行态 |
| `LOOP.md` | 活动 loop 清单 |
| `loop-budget.md` | 配额 |
| `loop-run-log.jsonl` | 日志 |
| `loop-constraints.md` | 硬约束 |
| `gate.yaml` | 路径门禁 |

## 可选文件（按需 fork 后补）

| 文件 | 何时需要 |
|---|
| `src/loop/` 模块 | 想要 `cargo run -- loop-*` 子命令 |
| `scripts/loop-doctor.sh` | 想要 bash 版本评分 |
| `patterns/registry.yaml` | 启用 ≥ 1 个 Pattern |
| `.github/workflows/loop-*.yml` | 启用 GitHub Actions 调度 |
| `examples/minimal-rust-loop/` | 自己的 fork 起点 |

## 用法

```
/loop-init-minimal-rust
```

会调用本目录的模板创建 6 个文件，输出 Loop Readiness Score（应为 R1，工具就绪）。

## 后续

1. 跑 `/loop-doctor` 确认 ≥ R1
2. 按需 fork `examples/minimal-rust-loop/` 中的其他文件
3. 选 1 个 Pattern 跑 ≥ 7 天 L1
4. 累积 accuracy 数据后申请 L2 解锁

## 关联文档

- [README.md](README.md)
- [docs/loop-engineering/integration-plan.md](../../docs/loop-engineering/integration-plan.md)
- [docs/loop-engineering/implementation-roadmap.md](../../docs/loop-engineering/implementation-roadmap.md)