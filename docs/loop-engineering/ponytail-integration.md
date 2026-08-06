# Ponytail × Loop-Engineering 整合方案

**分析对象**：刚安装的 6 个 ponytail skills（MIT, 97k ⭐）与 AutoHarness 现有 loop-engineering 系统（Phase 1 完成）的深度整合。

**输出**：完整方案分析，不写代码。

**最后更新**：2026-08-06

---

## 1. 双方本质分析

### 1.1 ponytail 是什么

**一句话定位**：AI coding 的"最小化心智模型"——用 7 级 ladder 把 Senior Dev 的本能固化下来。

**核心机制**：

| 层级 | 问自己 | 行动 |
|------|--------|------|
| Rung 1 | 需要存在吗？ | YAGNI，skip |
| Rung 2 | 代码库里已有？ | 重用 |
| Rung 3 | stdlib 已有？ | 用 stdlib |
| Rung 4 | 平台原生功能？ | 用原生 |
| Rung 5 | 已安装依赖？ | 用它，别加新依赖 |
| Rung 6 | 能一行吗？ | 一行 |
| Rung 7 | 才写最小可工作代码 | — |

**6 个 skill 的分工**：

| Skill | 类型 | 作用 |
|-------|------|------|
| `/ponytail` | **持久模式** | 激活后每次回复都走 ladder，3 级强度（lite/full/ultra） |
| `/ponytail-review` | **一次性** | 审查一个 diff，输出 `L42: yagni: X` 格式一行式 |
| `/ponytail-audit` | **一次性** | 全仓库扫描，输出删除建议排名 |
| `/ponytail-debt` | **一次性** | 收割 `ponytail:` 注释 → ledger |
| `/ponytail-gain` | **一次性** | 显示 benchmark 基准数据（LOC▼54%, cost▼20%, speed▲3-6x） |
| `/ponytail-help` | **一次性** | 快速参考卡 |

**交付价值**：实证数据驱动的最小化编码规范。不是"建议"，是有基准的"已知正确方向"。

### 1.2 loop-engineering 是什么

**一句话定位**：把"一次性 AI Agent 提示词"提升为"持续编排、状态化、有安全门禁、可度量、可演进"的循环系统。

**核心机制**：

| 组件 | 作用 |
|------|------|
| 调度（Scheduling） | Cron / 手动触发心跳 |
| 隔离（Worktrees） | 一个 fix 一个 git worktree |
| 意图持久化（Skills） | SKILL.md 承载约束和经验 |
| 连接器（MCP） | GitHub/Slack 等可写连接 |
| Maker/Checker | Sub-agent 分离（AutoHarness 用 `HarnessType` 零成本复用） |
| 状态机 | STATE.md（运行态）↔ GOAL.md（目标态） |
| 安全门禁 | gate.yaml + loop-constraints.md |
| 模式（Pattern） | improvement-loop 等 7 个 Pattern |
| 成熟度 | L1 Report-only → L2 Assisted → L3 Unattended |
| 双轨 fitness | score.sh（代码质量）+ loop-doctor.sh（loop 健康度） |

### 1.3 核心洞察：不同维度的正交系统

```
┌─────────────────────────────────────────────────────────┐
│  loop-engineering  ←  结构性 / 编排层                   │
│  "WHEN to act, WHO acts, WHAT stays safe"              │
│                                                          │
│    ┌─────────────────────────────────────────────┐      │
│    │  ponytail  ←  生成/决策层                    │      │
│    │  "HOW to write the code that gets done"     │      │
│    └─────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────┘
```

**结论**：ponytail 管"代码怎么写"，loop-engineering 管"什么时候、谁来写什么"，**正交不冲突**。

---

## 2. 互补性分析：哪些地方互相强化

### 2.1 ponytail 给 loop-engineering 补了什么

**当前 loop-engineering 缺的东西，ponytail 直接给**：

| 缺口 | ponytail 怎么补 |
|------|----------------|
| improvement-loop 每次提案靠"经验"，无标准 | **7 级 ladder** 成改进提案的决策标准：propose-action 前必须过 ladder |
| L2 自动修代码没有质量规范 | **删除优先 + ponytail-debt 注释** 让每一次"偷懒"都有记录、可回查 |
| score.sh 关注"有没有问题"，不关注"是否过度" | **ponytail-audit** 专门扫描过工程化，与 score.sh 互补 |
| 人类对 loop 改的代码质量心里没底 | **ponytail-review** 在 L2 的 maker/checker 之后追加"复杂度 gate" |
| AGENTS.md 有规则但 AI 可能不遵守 | **ponytail 持久模式** 把"最小化"变成 agent 每次响应的默认过滤器 |

### 2.2 loop-engineering 给 ponytail 补了什么

| ponytail 缺的东西 | loop-engineering 怎么补 |
|------------------|------------------------|
| ponytail 是单次模式，无状态 | **STATE.md** 把 ponytail 的"跳过"和"简化"提议记录下来，变成待人类确认的 action |
| ponytail-review 是事后 review | **L2 maker/checker 门禁** 把 review 变成自动 gate，而不是事后通知 |
| ponytail-debt 收集了注释但没有消化机制 | **improvement-loop** 可以把 ledger 中的项变成正式 action，推动"够了就加回来" |
| ponytail-gain 的数据是通用基准，不是本仓库的 | **iterations.jsonl** 记录每个改动的 LOC 变化 → 可以算出本仓库 ponytail 实绩 |

### 2.3 最强互补点

**improvement-loop 的 propose-action 阶段 + ponytail ladder**：

```
当前 improvement-loop:
  1. score.sh → 最低分项 → 从 catalog 选 action → 执行

加入 ponytail 后:
  1. score.sh → 最低分项 → 用 7 级 ladder 评估 catalog 中的 action
     → 问：这个 action 是否本身就是 over-engineering 的结果？
     → 问：这个 action 能不能用更小的 diff 替代？
     → 选 ladder 最高位置的方案
  2. 执行前：用 ponytail-review 先扫一遍 diff
  3. 若有简化：加 `ponytail:` 注释，落地到 ponytail-debt ledger
  4. 结果写 STATE.md → 变成 improvement-loop 的 human review 项
```

---

## 3. 冲突与张力分析

### 3.1 持久模式 vs 模型无关性

**问题**：ponytail 主 skill 是"持久模式"——激活后每次回复都走 ladder。但 loop-engineering 设计为"模型无关"，不希望一个 skill 永久改变 agent 行为。

**分析**：这是**表层冲突，不是深层冲突**。

- 持久模式只在**人类的会话 session** 中有效
- loop-engineering 的 GitHub Actions workflow 是**无会话独立进程**，`/ponytail` 不会在 `cargo run -- loop run` 里激活
- 真正需要 ponytail 约束的是**人类驱动的改进**（improvement-loop 由人触发时），而不是 workflow 的自动扫描

**结论**：持久模式不需要改动。人类想用 `/ponytail` 随时激活；workflow 跑的是 skill 脚本，不走会话模式。

### 3.2 ponytail-debt vs STATE.md 职责重叠

**问题**：ponytail-debt 收集 `ponytail:` 注释到 ledger；STATE.md 也有待办列表。

**分析**：两者**语义不同，不重叠**：

- **STATE.md 待办**：loop 需要处理或等人类确认的项（优先级驱动）
- **ponytail-debt ledger**：被有意简化的技术债务（记录驱动）

**整合策略**：
- ponytail-debt 保持独立文件（`ponytail-debt.md`），每季度或人类主动触发时刷新
- 当 ledger 中某项达到触发条件（"upgrade when X" 中的 X 发生），**improvement-loop** 可以自动把它变成一个 action 提案
- 不强制合并两个系统，但建立引用关系

### 3.3 ponytail 的"删除优先"vs loop-engineering 的"不能降低 score"

**问题**：ponytail ladder 优先删除代码；improvement-loop 不允许 score 降低。

**分析**：**不冲突，方向一致**。

- ponytail 删除的是"过度抽象、死代码、冗余依赖"
- score.sh 测的是 format/clippy/tests/docs/maintenance/safety
- 删除死代码 → 更少 maintenance 问题 → score 可能上升
- 若删除导致 test coverage 下降 → score 下降 → improvement-loop 感知到并 revert

**结论**：ponytail 删除是安全的，只要 Verifier（`HarnessType::Verifier`）跑 `cargo test` 验证无 regression。loop-engineering 的 maker/checker 结构天然保护这个边界。

### 3.4 ponytail 的"YAGNI skip" vs improvement-loop 的"必须选一个 action"

**问题**：ponytail Rung 1 是"不需要就别做"；improvement-loop 需要产出一个 action 提案。

**分析**：这其实是 improvement-loop **本来就有的规则**——如果 score 已经满分，就不需要 action。"noop" 是 improvement-loop 的合法 status（`loop-run-log.jsonl` 里 `status=noop` 是标准值）。

**结论**：不需要修改任何一方。improvement-loop 的 noop 结果恰好是 ponytail YAGNI 的体现。

---

## 4. 整合点地图

### 4.1 Skill 层：在哪里调用 ponytail skills

```
loop-engineering patterns/
├── improvement-loop/
│   ├── [L1] propose-action 阶段
│   │   └── 调用 /ponytail-review 扫描 action 提案
│   │   └── 若发现 YAGNI：在 STATE.md 记录"建议跳过"
│   ├── [L2] 执行阶段
│   │   └── 调用 /ponytail-review 扫描 diff（双 gate 第 3 关）
│   │   └── 加 `ponytail:` 注释
│   └── [any] 若无 action 可做
│       └── status=noop（YAGNI）
│
├── test-coverage/
│   └── [L1] 发现未覆盖代码
│       └── 先问：这段代码本身是否需要？→ ponytail YAGNI
│       └── 若需要才生成补测提案
│
├── clippy-fmt-watch/
│   └── [L2] 自动 fmt fix 后
│       └── 调用 /ponytail-review 检查 fix diff
│       └── 防止"fmt 修了一个问题引入另一个过度抽象"
│
└── synthesis-quality/
    └── [L1] 发现 reward 异常波动
        └── 不是修而是问：这个合成路径是否本身就是 over-engineering？
```

### 4.2 Action Catalog：ponytail 成为正式 action 类型

在 improvement-loop 的 Action Catalog（目前散落在 GOAL.md / patterns 里）中增加：

```markdown
### Type: Ponytail Refactor

适用条件：发现以下任一情况

- [ ] `stdlib:` 有 hand-rolled 实现（替换为 stdlib）
- [ ] `native:` 有依赖做平台原生功能（用原生）
- [ ] `yagni:` 有只有一个实现的抽象（inline 它）
- [ ] `shrink:` 有可以缩短的逻辑（缩短）
- [ ] `delete:` 有死代码或零调用函数（删除）

Action 格式：
  action_type: ponytail-refactor
  tag: stdlib | native | yagni | shrink | delete
  file: <path>
  change: <一行描述>
  ponytail_ceiling: <当前方案的性能/能力上限>
  upgrade_when: <触发升级的条件>

验收：
  1. cargo fmt + cargo clippy 通过
  2. cargo test 通过
  3. score.sh 不降低
```

### 4.3 约束层：ponytail 融入 loop-constraints.md

当前 `loop-constraints.md` 的"代码"章节扩展：

```markdown
## Code
- 改前必跑 cargo fmt
- 改前必跑 cargo clippy
- 改前必跑 cargo test
- 每项 fix 最多 3 次尝试，超过 escalate

### Ponytail Code Norms（建议，永远不强制的软约束）
- 先走 7 级 ladder，再动手
- 删除优先于添加（YAGNI 先于 "加个配置项"）
- 用 `ponytail:` 注释标记有意省略：`# ponytail: global lock, per-account if throughput matters`
- 任何 action 执行前：用 ponytail-review 扫描 diff
```

**注意**：ponytail 在这里标注为"软约束"，因为不是所有人类都喜欢这个风格。loop-engineering 的强制约束（denylist、kill switch）不受影响。

### 4.4 状态层：ponytail-debt 与 STATE.md 引用关系

```
STATE.md 新增 section：

## Ponytail Debt（每季度或 /ponytail-debt 触发时刷新）
- 引用 `ponytail-debt.md`（外部文件）
- 若 ledger 中某项的 upgrade_when 条件已满足：
  → 升级为 improvement-loop action 提案
  → 从 ledger 中移除该行
```

### 4.5 指标层：ponytail-gain → 双轨 fitness 补充

| 现有指标 | 补充 |
|---------|------|
| score.sh（代码质量，6 维） | pon

ytail-debt 债务量（应趋近 0 或缓慢增长） |
| loop-doctor.sh（loop 健康度，10 维） | ponytail-debt ledger 存在且被引用 |
| iterations.jsonl（code 迭代） | 新增 `ponytail_tag` 字段（stdlib/native/yagni/shrink/delete）|

---

## 5. 模式（Pattern）级整合

### 5.1 新增 Pattern：`ponytail-audit-loop`

| 字段 | 值 |
|------|---|
| ID | `ponytail-audit` |
| 目标 | 消除代码库中的过工程化债务 |
| Cadence | 每月一次（token 密集） |
| 起步 Level | L1 Report-only（必须） |
| Token/天 | 50k（扫描全仓库） |
| Phase 1 行为 | 调用 `/ponytail-audit` → 写 STATE.md Ponytail Debt section |
| Phase 2 行为 | 对 allowlist 路径自动执行 `/ponytail-review` 建议 |
| Phase 3 行为 | 对 stdlib/native 替换开 PR（需人审） |

**理由**：这是最自然的 ponytail 与 loop-engineering 的整合——ponytail-audit 本身就是一个 scan-and-report 工具，与 L1 Report-only 天生匹配。

### 5.2 现有 Pattern 扩展

| Pattern | ponytail 注入点 |
|---------|----------------|
| improvement-loop | propose-action 前用 ladder 过滤；执行后用 ponytail-review 把关 |
| test-coverage | 补测前先问：这段代码本身是否 YAGNI |
| doc-staleness | 检查 docs 里的代码示例是否 over-engineered（用 ponytail lens） |
| clippy-fmt-watch | fmt fix 后追加 ponytail-review，防止引入新复杂度 |

---

## 6. L2/L3 的 maker/checker 与 ponytail

### 6.1 三关门禁模型（已扩展）

```
                    ┌─────────────────────────┐
  Refiner (maker)   │ 产出代码改动             │
                    └───────────┬─────────────┘
                                ▼
                    ┌─────────────────────────┐
  Gate 1: Verifier  │ cargo test + cargo clippy │
                    └───────────┬─────────────┘
                                ▼
                    ┌─────────────────────────┐
  Gate 2: Critic    │ score.sh 分数不降低      │  ← 现有
                    └───────────┬─────────────┘
                                ▼
                    ┌─────────────────────────┐
  Gate 3: Ponytail  │ ponytail-review diff    │  ← 新增
                    │ 过工程化扫描             │
                    └───────────┬─────────────┘
                                ▼
                    ┌─────────────────────────┐
  Gate 4: Human     │ STATE.md 待审清单        │  ← 现有
                    └─────────────────────────┘
```

**Gate 3 细节**：

- 只在 L2 及以

上激活
- `ponytail-review` 发现 `delete:` 或 `yagni:` → 自动在 diff 里加 `ponytail:` 注释，不阻塞 merge
- 发现 `stdlib:` 或 `native:` → 在 PR 评论里提示，escalate 到 STATE.md Human Inbox
- 发现 `shrink:` → 尝试自动 shrink（已在 allowlist 路径内）

### 6.2 Ponytail Comment 追踪链

```
ponytail-review 发现简化机会
    ↓
action diff 里加 `ponytail:` 注释
    ↓
ponytail-debt 收割 → ponytail-debt.md
    ↓
STATE.md Ponytail Debt section 引用 ledger
    ↓
upgrade_when 触发 → improvement-loop 读取 → 升级为 action
    ↓
升级后删除 `ponytail:` 注释
    ↓
从 ledger 移除该行
```

---

## 7. 北极星指标补充

| 指标 | 定义 | 目标 | 测量 |
|------|------|------|------|
| **Ponytail Debt Size** | ponytail-debt.md 记录条数 | ≤ 10 且不增长 | `/ponytail-debt` |
| **Ponytail-tagged Actions** | iterations.jsonl 中带 ponytail_tag 的条目占比 | 越高越好（说明主动消除债务） | 解析 jsonl |
| **Audit Frequency** | ponytail-audit-loop 扫描间隔 | ≥ 1/月 | loop-run-log.jsonl |
| **Complexity-free Diff %** | ponytail-review 发现 0 问题的 diff 比例 | ≥ 60% | PR 统计 |

---

## 8. 实施路线

### Phase 0（立即，零风险）

**目标**：让 ponytail 融入团队日常，不改动任何 loop 基础设施。

1. 在 AGENTS.md 新增一章"Code Norms"，引用 ponytail 7 级 ladder
2. 在 improvement-loop SKILL.md 添加：`propose-action 前建议先 /ponytail lite`
3. 在 improvement-loop Action Catalog 增加"Type: Ponytail Refactor"（如 §4.2）
4. 创建 `ponytail-debt.md`（空文件），作为债务 ledger
5. 在 STATE.md 新增 Ponytail Debt section（引用 ledger）

**改动范围**：4 个文件，全是文档，无 Rust 代码，无 workflow。

### Phase 1（约 2 周）

**目标**：ponytail-audit-loop 作为独立 Pattern 上线（L1 only）。

1. 创建 `skills/loops/ponytail-audit-loop/SKILL.md`
2. 在 `patterns/registry.yaml` 注册 `ponytail-audit-loop`
3. 配置 GitHub Actions cron：每月第一个周一跑
4. pon

在 `loop-budget.md` 增加 `ponytail-audit-loop` token 预算

**改动范围**：1 个新 skill + registry.yaml + workflow yaml + budget 更新。

### Phase 2（约 4 周）

**目标**：L2 improvement-loop 集成 ponytail-review Gate 3。

1. 在 improvement-loop L2 阶段加入 `ponytail-review` diff 扫描
2. 在 `loop-constraints.md` 增加 ponytail code norms（软约束）
3. 扩展 `loop-run-log.jsonl` schema 增加 `ponytail_tag` 字段
4. 在 `scripts/loop-accuracy.sh` 增加 ponytail-review 准确率追踪

**改动范围**：improvement-loop skill + constraints + 2 个脚本。

### Phase 3（约 8 周）

**目标**：ponytail-debt 消化机制上线。

1. 在 improvement-loop 的 propose-action 阶段增加"检查 ponytail-debt.md upgrade_when"逻辑
2. 实现升级后的 action 类型标记
3. 建立 ponytail-debt.md → improvement-loop → STATE.md 的引用流
4. 完善 ponytail-gain 的本仓库版（基于 iterations.jsonl 实际数据）

---

## 9. 风险分析

### 9.1 风险清单

| 风险 | 可能性 | 影响 | 缓解 |
|------|--------|------|------|
| ponytail 风格与团队coding style冲突 | 中 | 低 | Phase 0 只做软约束，不强制 |
| ponytail-debt.md 无限膨胀没人清理 | 中 | 中 | Phase 3 实现消化机制；设硬上限（≤10 条） |
| Gate 3 ponytail-review 增加 L2 延迟 | 低 | 低 | 并行执行，不串在关键路径上 |
| ponytail-audit-loop 月度扫描 token 超预算 | 低 | 低 | 50k/次上限；超限自动 noop |
| "删除优先"与 safety-critical 代码冲突 | 极低 | 高 | gate.yaml denylist 保护 `src/core/*` 和 `src/engine/*` |

### 9.2 最重要的一个决策

**ponytail 注释约定**必须严格遵循：

```
# ponytail: <简化描述>, ceiling: <上限>, upgrade_when: <触发条件>
```

无 upgrade_when 的 `ponytail:` 注释视为 rot risk，在 ponytail-debt 输出里加 `no-trigger` 标记，引起注意。

---

## 10. 整合后的 Loop Readiness Score

加入 ponytail 后，10 维中的影响项：

| 维度 | 当前 | 整合后变化 |
|------|------|-----------|
| LOOP.md 完整 | 依赖 | 无变化 |
| STATE.md 最新 | 依赖 | Ponytail Debt section 加分 |
| loop-budget.md | 依赖 | pon

添加 ponytail-audit-loop token 预算后可能加 2 分 |
| loop-run-log.jsonl 有记录 | 依赖 | ponytail-audit-loop 产生额外日志条目 |
| loop-constraints.md | 依赖 | ponytail code norms 加 1-2 分 |
| gate.yaml 合法 | 依赖 | 无变化 |
| patterns/registry.yaml | 依赖 | ponytail-audit-loop 注册后加 1 分 |
| ≥3 次 L1 运行 | 依赖 | 无变化 |
| maker/checker 分离 | 依赖 | Gate 3 ponytail-review 强化分离 |

ponytail-audit-loop 注册后加 1 分，≥3 次 L1 运行无变化，maker/checker 分离通过 Gate 3 ponytail-review 强化，加 1 分。kill switch 可用性无变化。**预计 R-score +4 分**（现有 R3 88/100 → 92/100）。

---

## 11. 最终架构图

```
              ┌──────────────────────────────────────────┐
              │  Human / CLI / GitHub Actions            │
              └────────────────┬─────────────────────────┘
                                 │
              ┌─────────────────▼──────────────────────────┐
              │  loop-engineering 编排层                     │
              │  (Scheduling · Worktree · State · Gate)    │
              └────────────────┬──────────────────────────┘
                               │
         ┌─────────────────────┼─────────────────────┐
         │                     │                     │
         ▼                     ▼                     ▼
  improvement-loop      ponytail-audit-loop    [其他 5 个 Pattern]
  ┌─────────────┐       ┌──────────────┐
  │ ponytail    │       │ /ponytail    │
  │ ladder 在   │       │ -audit       │
  │ propose-    │       │ (scan only)  │
  │ action 前  │       └──────┬───────┘
  └──────┬──────┘              │
         │  L2+ Gate 3:        │
         └──────▶ /ponytail   │ 写
                    -review    ▼
                    diff  ──▶ ponytail-debt.md
                               │
                               ▼
                         STATE.md Ponytail Debt
                               │
                               ▼
                    improvement-loop 读取
                    upgrade_when 触发
                    → 升级为正式 action
```

---

**主方案** → [docs/loop-engineering/integration-plan.md](docs/loop-engineering/integration-plan.md)
**Patterns 详情** → [docs/loop-engineering/patterns-and-levels.md](docs/loop-engineering/patterns-and-levels.md)
**Ponytail 源码** → [DietrichGebert/ponytail](https://github.com/DietrichGebert/ponytail)
