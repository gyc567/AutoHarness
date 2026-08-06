# Loop-Engineering 集成方案

> "Stop prompting. Design the loop."
> — Peter Steinberger / Boris Cherny

本目录包含把 [loop-engineering](https://github.com/cobusgreyling/loop-engineering) 方法论引入 AutoHarness 的**完整设计文档**。

AutoHarness 在引入此方案时**作为对外示例项目**（决策 Q1=B），目的是让其他 Rust / CLI 项目能 1:1 复制本仓库的 loop 配置。

---

## 文档地图

| 文档 | 用途 | 何时读 |
|---|---|---|
| [README.md](README.md) | 入口、决策摘要、文档导航 | 第一份读 |
| [integration-plan.md](integration-plan.md) | 完整方案：原则 + 架构 + 原语 + 安全门禁 + CLI 工具 | 设计阶段 |
| [patterns-and-levels.md](patterns-and-levels.md) | 7 个 Pattern + L0-L3 等级 + 北极星 | 实施前 |
| [implementation-roadmap.md](implementation-roadmap.md) | Phase 1-4 + 文件清单 + 验收 + 风险 | 实施时 |
| [audit-2026-08-06.md](audit-2026-08-06.md) | 全面审计：发现清单 + 修订映射（31 项） | 评审时 |

---

## 一句话总结

把 loop-engineering 的"调度心跳 + 安全门禁 + 模式目录 + 状态机"四件套，作为**方法论层**叠加到 AutoHarness 上，不动引擎；用 Rust-native 子命令实现 CLI 工具矩阵；用现有 `HarnessType::{Refiner, Verifier}` 零成本获得 maker/checker；前 2 周强制 L1 report-only；让"自进化"从口号变成可度量、可门禁、可追溯的工程机制。

---

## 三个关键决策（用户确认 2026-08-06）

| 编号 | 决策 | 选择 | 含义 |
|---|---|---|---|
| **Q1** | 项目定位 | **B. 完整版**（对外示例） | AutoHarness 作为 loop-engineering 的 Rust 示例项目；其他项目可 fork 复制 |
| **Q2** | 调度方式 | **A. GitHub Actions cron** | 不需要守护进程；用现有 CI 跑 loop |
| **Q3** | L2 解锁条件 | **A. 数据驱动** | L1 满 14 天 + accuracy ≥ 80% 才升级；需人工签字 |

---

## 核心原则（硬性约束）

1. **零侵入**：不改 `src/**` 任何 Rust 代码；不替换 GOAL.md / iterations.jsonl / PLANS.md / DOCS.md
2. **复用优先**：用现有 `score.sh` 作 fitness、复用 `HarnessType` 类型作 maker/checker、复用 `skills/` 目录结构
3. **Rust-native**：工具写成 `cargo run -- loop-*` 子命令，不复制 npm CLI
4. **L1 优先**：前 2 周只跑 L1 Report-only，绝不自动改代码
5. **人类永远是最终门**：`STATE.md` 里的待办必须可读、可审、可中断
6. **机器可读 + 人类可读**：YAML/JSONL 给机器，Markdown 给人类
7. **小步快跑**：用现有 `iterations.jsonl` 同样范式记录 loop 自身的演进

---

## 北极星指标（新增）

| 指标 | 定义 | 目标 |
|---|---|---|
| **Loop Readiness Score** | `loop-doctor.sh` 综合分 | ≥ 80（解锁 L3） |
| **L1 Accuracy** | 人工 review 确认 STATE.md 中"对"的比例 | ≥ 80%（解锁 L2） |
| **Token 效率** | 实际 token / 预算 token | ≤ 80%（强制降级） |
| **信噪比** | 需人介入的发现项 / 总发现项（越低越好） | ≤ 25% |

---

## 当前状态

| 维度 | 状态 |
|---|---|
| 方案设计 | ✅ 已完成 |
| 用户确认 | ✅ Q1=B, Q2=A, Q3=A |
| 全面审计 | ✅ 2026-08-06（[audit-2026-08-06.md](audit-2026-08-06.md)） |
| Phase 1 文件创建 | ⏸ 待开工（需用户说"可以开始"） |
| Loop 激活 | ⏸ Phase 2 之后 |

---

**最后更新**：2026-08-06（审计修订版）
**关联上游**：[cobusgreyling/loop-engineering](https://github.com/cobusgreyling/loop-engineering) @ 9.9k stars, 372 commits