# Ponytail × Loop-Engineering 整合方案

**分析对象**：6 个 ponytail skills 与 AutoHarness 现有 loop-engineering 系统（Phase 1 完成）的深度整合。

**输出**：完整方案分析，不写代码。

**最后更新**：2026-08-06

---

## 1. 审计发现（用 ponytail lens）

原方案 507 行，有以下问题：

| # | 问题 | 类型 | ponytail 原则违反 |
|---|------|------|------------------|
| 1 | **ponytail-audit-loop 单独建 Pattern** | 过度抽象 | 一件事做了两个 Pattern：improvement-loop 本身就能跑 scan+report，多加一个 Pattern 是 layer with one caller（yagni: abstraction） |
| 2 | **4-gate 模型有歧义** | 概念不清 | §6.1 画了 4 个 gate，但 Gate 1/2/4 是在 Rust 代码里的，Gate 3 ponytail-review 是在 skill 层的，串在一起描述容易误导"要在 Rust 代码里加 gate" |
| 3 | **Phase 3 "ponytail-debt 消化机制"8 周才能跑** | YAGNI 过度设计 | Phase 2 已经把 ponytail-review 集成进去了，Phase 3 的消化链（6 步 + upgrade_when 触发 + 注释删除 + ledger 移除）是一个还没看到真实需求的复杂机制 |
| 4 | **§7 北极星指标表有未完成行** | 格式错误 | `pon` 换行残留 |
| 5 | **§5.2 "synthesis-quality 用 ponytail lens 质疑合成路径"** | 模糊提议 | 描述的是"质疑"而不是具体 action，不可执行 |
| 6 | **§5.2 "doc-staleness 用 ponytail 检查 docs 代码示例"** | 越界 | doc-staleness Pattern 管的是"文档是否陈旧"，不是"文档里的代码示例是否过度" |
| 7 | **§10 R-score 计算混淆了 Pattern 名和文件名** | 数据错误 | 把"ponytail-audit-loop" 注册当加分项混在 loop-budget.md 里说，分数来源不明 |
| 8 | **三处重复内容** | 冗余 | §2.3/§4.1/§5 最强互补点重复；§6.1/§10 架构图重复；ponytail 7 级 ladder 在 §1 和 §3.1 重复 |

---

## 2. 修正后的核心整合点（精简到 3 个）

### 2.1 整合点 A：improvement-loop 内置 ponytail ladder

**在哪里**：improvement-loop SKILL.md 的 propose-action 阶段。

**怎么改**：在选 action 前加一行——"先用 7 级 ladder 自检：有没有 over-engineering 的替代方案？"

**不需要改**：
- 不需要新建 Pattern
- 不需要在 State 里加新 section
- 不需要改 loop-run-log schema

### 2.2 整合点 B：L2 improvement-loop 加 Gate 3

**在哪里**：improvement-loop L2 执行阶段，在 Verifier + Critic 之后追加 ponytail-review。

**说明**：这不是 Rust 代码里的新 gate，是 improvement-loop SKILL.md 里 L2 流程中增加的一个 skill 调用步骤。表述为"第 3 关"是为了形象，实际只是一个 action 步骤。

**不需要改**：
- 不需要修改 `src/loop/runner.rs`
- 不需要改 `HarnessType`
- 只改 SKILL.md 的 L2 流程描述

### 2.3 整合点 C：ponytail-debt 作为 Ponytail Refactor action 的副产品

**在哪里**：improvement-loop 的 Action Catalog 增加一个类型。

**ponytail-debt.md 不单独维护**——当 improvement-loop 执行了一个 Ponytail Refactor action（stdlib 替换、yagni inline、shrink），就在该 action 的迭代记录里加 `ponytail_tag` 字段。ponytail-debt 的可见性通过 iterations.jsonl 的 ponytail_tag 过滤实现，不需要独立文件。

**若要独立可见**：在 STATE.md 加一行，引用 iterations.jsonl 里 tag=delete/yagni/stdlib 的最新 5 条记录即可。

---

## 3. 修正后的实施路线（4 → 2 阶段）

### Phase 0（即时，零风险）

**目标**：把 ponytail 变成 improvement-loop 的一部分，不增加任何基础设施。

1. 在 `skills/goal-md/improvement-loop/SKILL.md` 的 propose-action 步骤前加一条：
   > 提示：先过 7 级 ladder 自检——YAGNI → 重用 → stdlib → 原生 → 已有依赖 → 一行 → 才动手。

2. 在 improvement-loop Action Catalog 增加"Type: Ponytail Refactor"条目（见 §4）。

3. 在 `AGENTS.md` 的"Code Norms"章节引用 ponytail 7 级 ladder（软约束）。

**改动范围**：1 个 skill + 1 个章节，零 Rust 代码，零新文件。

### Phase 1（约 2 周）

**目标**：L2 improvement-loop 追加 ponytail-review Gate。

1. 在 improvement-loop SKILL.md 的 L2 执行流程中，在 `cargo test` + `cargo clippy` 之后、STATE.md 更新之前，插入 `ponytail-review` 调用。

2. 若 ponytail-review 发现 `stdlib:` 或 `native:`，在 PR 评论里留一条提示，不阻塞 merge。

3. Ponytail Refactor action 的迭代记录加 `ponytail_tag` 字段到 iterations.jsonl。

**改动范围**：improvement-loop SKILL.md + iterations.jsonl schema 注释。

### 不要做的事（删掉的 Phase 2/3）

| 原计划 | 为什么删除 |
|--------|-----------|
| ponytail-audit-loop 单独建 Pattern | yagni：improvement-loop 本身就能做 scan+report，多一个 Pattern 只有坏处（多一个 L1 观察期、占 budget、占 registry slot） |
| ponytail-debt.md 独立文件 | yagni：ledger 的可见性用 iterations.jsonl 的 ponytail_tag 过滤实现，零额外文件 |
| Phase 3 "消化机制" | over-engineering：6 步追踪链没有真实需求支撑，先跑起来再迭代 |
| ponytail-audit 单独跑 | yagni：improvement-loop 每次 run 都是一次审计，不需要额外的月度扫描 |
| gate.yaml 新增约束 | 过早：等 ponytail 风格被团队接受后再固化 |

---

## 4. Action Catalog：Ponytail Refactor 类型

在 improvement-loop Action Catalog 中增加：

```markdown
### Type: Ponytail Refactor

当 ponytail ladder 在代码库里发现以下情况时触发：

- `stdlib:` hand-rolled 实现，替换为 stdlib
- `native:` 依赖做平台原生功能，用原生替代
- `yagni:` 只有一个实现的抽象，inline 它
- `shrink:` 可以缩短的逻辑，缩短
- `delete:` 死代码或零调用函数，删除

执行格式（记录到 iterations.jsonl）：
  "action": "ponytail-refactor",
  "tag": "stdlib|native|yagni|shrink|delete",
  "file": "<path>",
  "summary": "<一行描述>"

验收（与所有 action 相同）：
  1. cargo fmt && cargo clippy 通过
  2. cargo test 通过
  3. score.sh total 不低于 before
```

---

## 5. 北极星指标（精简到 1 行）

在现有指标表末尾追加一行：

| 指标 | 定义 | 目标 | 测量 |
|------|------|------|------|
| **Ponytail-tagged Actions** | iterations.jsonl 中 tag 含 ponytail 的条目占比 | 越高越好（说明在主动消除债务） | `jq 'select(.action == "ponytail-refactor")' iterations.jsonl \| wc -l` |

---

## 6. 风险分析（精简到 2 条）

| 风险 | 可能性 | 影响 | 缓解 |
|------|--------|------|------|
| ponytail 风格与团队习惯冲突 | 中 | 低 | Phase 0 纯软约束，随时可撤 |
| iterations.jsonl ponytail_tag 字段没人填 | 高 | 低 | 在 improvement-loop SKILL.md 里写死格式，模板驱动 |

---

## 7. 与原方案的 diff

```
删除：
- ponytail-audit-loop 单独 Pattern（yagni）
- ponytail-debt.md 独立文件（yagni）
- Phase 3 消化机制（over-engineering）
- gate.yaml ponytail 约束（过早）
- §2.2 loop-engineering 给 ponytail 补的四条（对偶重复）
- §5.2 doc-staleness / synthesis-quality 的 ponytail 注入（越界+模糊）
- §7 北极星指标表残留换行（格式错误）
- §10 R-score 计算表（数据混淆）
- §11 最终架构图（与 §6.1 重复）

保留：
- §1 ponytail 7 级 ladder（核心机制，必须有）
- §2.1 ponytail 给 loop 补什么（精简后）
- §2.3 最强互补点（精简后）
- §3.1-3.4 冲突分析（正确，保留）
- §4 Action Catalog（保留，格式修正）
- §6.2 Ponytail Comment 追踪链（改为注释而非机制）

合并：
- 原 §4 Skill 层 + §5 Pattern 级整合 → 新的 §2（3 个整合点）
- 原 §8-11 → 精简为 §6-7（风险+北极星）

行数：507 → ~250
```

---

**主方案** → [docs/loop-engineering/integration-plan.md](docs/loop-engineering/integration-plan.md)
**Patterns 详情** → [docs/loop-engineering/patterns-and-levels.md](docs/loop-engineering/patterns-and-levels.md)
**Ponytail 源码** → [DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail)
