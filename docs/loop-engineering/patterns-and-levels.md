# Patterns 与 Readiness Levels

本文档列出 AutoHarness 专属的 7 个 Loop Pattern、L0-L3 成熟度等级、Pattern 注册表格式、北极星指标。

阅读对象：实施工程师、需要写新 Pattern 的人、评估 loop 健康度的 Tech Lead。

---

## 1. 7 个 AutoHarness 专属 Pattern

不照搬 loop-engineering 原版 7 个，按 AutoHarness 自身痛点定制：

| ID | 名称 | 目标 | Cadence | 起步 Level | Token/天 |
|---|---|---|---|---|---|
| `improvement-loop` | 改进循环 | 推高 `score.sh` 分数 | 1d | L1 | 50k |
| `synthesis-quality` | 合成质量巡检 | 监控 `src/engine/synthesis.rs` 输出稳定性 | 1d | L1 | 30k |
| `test-coverage` | 覆盖率达 100% | 找未覆盖分支并补测 | 1d | L1 | 30k |
| `doc-staleness` | 文档陈旧度 | DOCS.md / docs/ 与代码 diff | 3d | L1 | 20k |
| `clippy-fmt-watch` | 格式/lint 守护 | 防止新代码引入 fmt/clippy 问题 | on-PR | L1 | 10k |
| `release-drafter` | 发布说明草稿 | 从 commit 自动生成 CHANGELOG | on-tag | L1 | 15k |
| `dependency-watch` | 依赖巡检 | Cargo.lock CVE 检查 | 1w | L1 | 40k |

> 所有 Pattern 统一 L1 起步（Q3=A 数据驱动）。`clippy-fmt-watch` / `dependency-watch` 虽带"自动"能力，也必须先走 L1 观察期（clippy-fmt-watch：10 次 PR 触发；dependency-watch：4 次周跑）再评估升级，杜绝"起始即 L2"绕过门禁。

### 1.1 improvement-loop（首要 Pattern）

**目标**：把 `score.sh` 分数从现状推到 100/100。

**Cadence**：每个工作日早上 8 点（UTC）跑一次。

**L1 行为**：
1. 跑 `scripts/score.sh --json`
2. 解析 6 个 component 分（format / clippy / tests / docs / maintenance / safety；score.sh 满分 110 封顶 100，以 `total` 为准）
3. 找最低分项
4. 读 `iterations.jsonl` 最近 5 条，避开已尝试
5. 在 STATE.md "Watch List" 加一条："建议 Action X（参考 §action catalog）"
6. **不动代码**

**L2 行为**：在 L1 基础上，对 `docs/**`、`skills/**/SKILL.md`、`tests/**` 自动 commit；其余路径 escalate。

**复用现有 skill**：`/improvement-loop`（skills/goal-md/improvement-loop/SKILL.md）

### 1.2 synthesis-quality

**目标**：监控 `src/engine/synthesis.rs` 输出稳定性。

**L1 行为**：
1. 跑 cargo test --release
2. 收集合成耗时、收敛率、reward 分布
3. 写入 STATE.md "Metrics" 段
4. 检测 reward 是否有 ≥ 10% 异常波动

### 1.3 test-coverage

**目标**：测试覆盖率维持 100%。

**L1 行为**：
1. 跑 `cargo tarpaulin --out Stdout`
2. 找未覆盖的 `pub fn` 和 `match` 分支
3. 写"待补测"清单到 STATE.md

### 1.4 doc-staleness

**目标**：文档与代码同步。

**L1 行为**：
1. 比对 `docs/**` 与 `src/**` 最近修改时间
2. 若文档超过 30 天未更新且对应代码有变更，flag 到 STATE.md

### 1.5 clippy-fmt-watch（on-PR 型，L1 起步）

**目标**：防止 PR 引入新 lint 问题。

**触发**：`on: pull_request`

**L2 行为**：
1. checkout PR（worktree 隔离）
2. 跑 `cargo fmt -- --check` 和 `cargo clippy -- -D warnings`
3. 若失败：
   - 自动 `cargo fmt` 并 commit fix 到 PR 分支（PR 分支内可自动 commit；`src/**` 改动不自动 merge，需人审）
   - 若 clippy 失败则只在 PR 评论提示，不自动修

### 1.6 release-drafter

**目标**：自动生成 CHANGELOG 草稿。

**触发**：`on: push tags: ['v*']`

**L1 行为**：
1. 读最近 N 个 commit
2. 按 conventional commits 分类
3. 生成 `CHANGELOG_DRAFT.md`
4. 开 issue 等人类 review

### 1.7 dependency-watch

**目标**：CVE 与依赖新鲜度。

**Cadence**：每周一次。

**L2 行为**：
1. `cargo audit`
2. 比对 Cargo.lock 中过期依赖
3. 分类：patch / minor / major
4. patch 只开 PR（Cargo.lock 在 gate.yaml denylist，lockfile 改动必须人审合并）；minor/major 仅 STATE.md 提示

---

## 2. Pattern 注册表格式（`patterns/registry.yaml`）

每个 Pattern 都复用 loop-engineering 的标准字段，写入 `patterns/registry.yaml`：

```yaml
- id: improvement-loop
  name: Improvement Loop
  file: patterns/improvement-loop.md
  project: autoharness            # Q1=B 标识
  cadence: 1d
  risk: low
  tools: [github-actions]
  skills:
    - skills/loops/new-goal-loop/SKILL.md
    - skills/loops/loop-triage/SKILL.md
  state: STATE.md
  phases:
    - triage-score
    - propose-action
    - verify
    - commit
  human_gates:
    - score-regression
    - multi-file-changes
  starter: examples/minimal-rust-loop/
  week_one_mode: L1
  token_cost: medium
  cost:
    tokens_noop: 5000
    tokens_report: 50000
    tokens_action: 100000    # 不得 > suggested_daily_cap（单次 run 不能超日预算）
    stable_fraction: 0.35
    suggested_daily_cap: 100000
    early_exit_required: false
```

---

## 3. L0-L3 成熟度等级

### 3.1 等级定义

**Pattern 成熟度等级（每个 Pattern 独立演进；勿与 §3.2 的 R0-R3 混淆）**：

| 等级 | 描述 | 启用条件 | 行为 |
|---|---|---|---|
| **L0** Draft | 只有意图文档 | 刚创建 Pattern | 不跑 |
| **L1** Report | 扫描 → 写 STATE，不动代码 | 注册后立即（Phase 2 起） | 仅告警 |
| **L2** Assisted | 小修 + maker/checker 双签 | L1 满 14 天 + accuracy ≥ 80% | allowlist 路径自动 commit |
| **L3** Unattended | 无人值守 | L2 满 7 天 + Ready Score ≥ 80 | 全自动（含 PR 开） |

### 3.2 Loop Readiness Score（0-100）

`scripts/loop-doctor.sh` 评估 10 个维度：

| 维度 | 满分 | 怎么评 |
|---|---|---|
| LOOP.md 存在且完整 | 15 | 文件 + 活动 loop 表 |
| STATE.md 存在且最新 | 15 | 24h 内更新过 |
| loop-budget.md 存在 | 10 | 文件 + 配额表 |
| loop-run-log.jsonl 存在且 7 天内有记录 | 10 | 文件 + 最近 run |
| loop-constraints.md 存在 | 10 | 文件存在 |
| gate.yaml 存在且合法 | 10 | YAML 可解析 + 路径项 |
| patterns/registry.yaml 存在 | 10 | YAML 可解析 |
| 至少 1 个 Pattern 跑过 ≥ 3 次 L1 | 10 | log 统计 |
| 至少有 1 处 maker/checker 分离 | 5 | 引用 `HarnessType::{Refiner, Verifier, Critic}`（maker + 双 gate） |
| kill switch 可用 | 5 | STATE.md 中有 pause 标志 |

**Readiness 映射（R0-R3，系统级，与 Pattern 的 L0-L3 无关）**：

```
R0: < 40   # 基础设施不完整，禁止任何 loop run
R1: 40-59  # 工具就绪，可跑 L1 report-only
R2: 60-79  # 健康，允许 L2 候选
R3: ≥ 80   # 成熟，解锁 L3 候选
```

### 3.3 双轨 Fitness：score.sh vs loop-doctor.sh

| 维度 | score.sh | loop-doctor.sh |
|---|---|---|
| 答的问题 | 代码质量如何 | loop 系统能否跑起来 |
| 满分 | 100（6 维，110 封顶 100） | 100（10 维） |
| 频率 | 每次 commit | 每天 |
| 谁跑 | CI + 人类 | Actions workflow |
| 数据源 | format / clippy / tests / docs / maintenance / safety | 10 维：文件存在性 + 校验 + 心跳 |

**两个都要 100，loop 才算"自进化"成熟**。

### 3.4 L2 解锁条件（Q3=A 数据驱动）

**门 1**：L1 累计运行 ≥ 14 天

**门 2**：累计 runs ≥ 10

**门 3**：L1 accuracy ≥ 80%

**门 4**：人类在 STATE.md 签字 `"L2-unlock-approved: <date> <signer>"`

**accuracy 计算**（在 `scripts/loop-accuracy.sh`）：

```bash
# 解析 STATE.md 中的 accuracy 段：
## Accuracy Tracking
- 2026-08-06 review: 5 items, 4 hit, 1 miss → 80%
- 2026-08-05 review: 3 items, 3 hit, 0 miss → 100%

# 输出解锁条件：
# L2 unlock: 14 天内 L1 accuracy ≥ 80% 且 runs ≥ 10
```

**漏报惩罚更重**（未来可升级为 F-score 替代 accuracy）：
- 漏报权重 2x
- 误报权重 1x

### 3.5 L3 解锁条件

L2 满 7 天 + Ready Score ≥ 80 + 0 次 score 退化事件。

---

## 4. 北极星指标（新增到 `docs/architecture/north-star-metrics.md`）

| 指标 | 定义 | 目标 | 测量 |
|---|---|---|---|
| **Loop Readiness Score** | `loop-doctor.sh` 综合分 | ≥ 80（解锁 L3） | scripts/loop-doctor.sh |
| **L1 Accuracy** | 人工 review 确认 STATE.md 中"对"的比例 | ≥ 80%（解锁 L2） | scripts/loop-accuracy.sh |
| **Token 效率** | 实际 token / 预算 token | ≤ 80%（强制降级） | loop-budget.md |
| **信噪比** | 需人介入的发现项 / 总发现项（越低越好） | ≤ 25% | scripts/loop-accuracy.sh |

### 4.1 现有北极星指标（不变）

| 指标 | 目标 |
|---|---|
| 代码合成成功率 | ≥ 85% |
| 平均合成时间 | < 500ms |
| 测试覆盖率 | ≥ 80% |
| CLI 响应时间 | < 100ms |

---

## 5. Pattern 设计 Checklist（开新 Pattern 前必读）

仿 loop-engineering 的 loop-design-checklist.md，AutoHarness 版。

> 开新 Pattern 前先读 §2 注册表格式与 §3.1 L 等级定义；checklist 中"Maker/Checker"指 `Refiner`（maker）+ `Verifier/Critic`（checker 双 gate）。

### 5.1 目的与范围

- [ ] **单一清晰目标**：一句话说清这个 loop 干什么
- [ ] **明确非目标**：这个 loop **不**干什么
- [ ] **监视范围**：哪些文件 / 路径 / 文件夹
- [ ] **阶段化放量**：report-only → 小修 → 自动
- [ ] **歧义输入处理**：模糊项 escalate 而非猜测

### 5.2 调度

- [ ] **Cadence 选择**：间隔匹配紧急程度
- [ ] **立即触发**：start 时是否立即跑
- [ ] **持久化**：跨 Actions run 是否需要状态恢复
- [ ] **小时行为**：夜间降频或暂停

### 5.3 Skills

- [ ] **Triage skill** 存在且输出格式紧凑
- [ ] **Action skills** 匹配项目约定
- [ ] **Skill 描述** 朴素具体（好触发）
- [ ] **Build/test 命令** 文档化在 skills 或 AGENTS.md

### 5.4 Maker/Checker

- [ ] **Implementer 与 Verifier** 分离（agent / model / instructions）
- [ ] Implementer **不能**自标 "done"
- [ ] Verifier 跑 **测试** 隔离（worktree）
- [ ] 停止条件由 **fresh model** 判断（适用时）

### 5.5 状态

- [ ] **STATE.md** 或 board schema 文档化
- [ ] Loop **读** 启动前 state
- [ ] Loop **写** 结果、时间戳、最后动作
- [ ] **剪枝** 已解决/合并/关闭项
- [ ] 人类 override 记录在 state

### 5.6 人类交接

- [ ] **Escalation 触发器** 显式（max attempts / risk paths / ambiguity）
- [ ] **Denylist 路径** — auth, payments, secrets, infra
- [ ] **通知规则** — 仅在需要动作时通知
- [ ] **收件箱** — 歧义项落 STATE.md section

### 5.7 连接器（MCP）

- [ ] 连接器最小权限（read vs write）
- [ ] Loop 能 **开/更新 PR** 或 ticket（如 acting）
- [ ] Bot identity 清晰（"Loop Engineering — PR Babysitter"）

### 5.8 成本与限制

- [ ] **Token 预算** 已估算
- [ ] **loop-budget.md** 有日上限 + kill switch
- [ ] **loop-run-log.jsonl** 追加历史
- [ ] **每次 run 检查** 配额
- [ ] **每次 item max iterations**
- [ ] **每日 max auto-PRs**
- [ ] **Pause/kill** 准则定义

### 5.9 可观察性

- [ ] **每次 run log**：开始、发现项、动作数、escalations
- [ ] **成功指标** 已选
- [ ] 团队能 **检查 STATE.md** 不读聊天日志

### 5.10 安全

- [ ] 无 auto-merge 无显式 allowlist
- [ ] secrets/env 文件在 denylist
- [ ] flake handling — 不以重试"修"间歇测试

---

## 6. 多 Loop 协调矩阵（来自 loop-engineering multi-loop.md）

| 组合 | 规则 |
|---|---|
| improvement-loop + clippy-fmt-watch | clippy-fmt-watch 在 PR 上跑；improvement-loop 不重复 fmt |
| improvement-loop + synthesis-quality | 共享 score 触发器；synthesis-quality 失败则 improvement-loop 暂停 |
| improvement-loop + dependency-watch | dependency-watch 仅 patch 自动；major 等人工 |
| release-drafter + 其他 | release-drafter 只读；不冲突 |

**优先级**（冲突时，基于本仓库 7 个 Pattern）：
1. 任何 Pattern 发现 CI 红 → 立即阻塞，进 STATE.md Human Inbox
2. clippy-fmt-watch（PR 阻塞，时效性最高）
3. improvement-loop（分数是北极星，优先于其他巡检）
4. dependency-watch（CVE 有时效性）
5. synthesis-quality / test-coverage / doc-staleness / release-drafter（非阻塞巡检）

---

**主方案** → [integration-plan.md](integration-plan.md)
**实施路线** → [implementation-roadmap.md](implementation-roadmap.md)

**最后更新**：2026-08-06