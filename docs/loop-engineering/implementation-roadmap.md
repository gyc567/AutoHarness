# 实施路线图

本文档是 Loop-Engineering 引入 AutoHarness 的**实施手册**：Phase 1-4 划分、完整文件清单、验收标准、风险、不做的事。

阅读对象：实施工程师（Phase 1 开工者）、Reviewer。

---

## 1. Phase 划分

### Phase 1：基础设施（不开 loop，只把工具准备好）

工期估算：3 天

**任务清单**（30 个新增文件 + 1 个修改项；`examples/minimal-rust-loop/` 目录内含 2 个文件，故 §2.1 新增清单计 31 个）：

| 序号 | 文件 | 内容 |
|---|---|---|
| 1 | `STATE.md` | 空模板，注释说明 |
| 2 | `LOOP.md` | 活动 loop 表占位 |
| 3 | `loop-budget.md` | 配额表占位 |
| 4 | `loop-run-log.jsonl` | 空文件 |
| 5 | `loop-constraints.md` | 硬约束 |
| 6 | `gate.yaml` | 路径黑白名单 |
| 7 | `patterns/registry.yaml` | 7 个 Pattern 注册 |
| 8 | `src/loop/mod.rs` | trait 定义 |
| 9 | `src/loop/state.rs` | STATE.md 读写 |
| 10 | `src/loop/log.rs` | loop-run-log.jsonl 追加 |
| 11 | `src/loop/constraints.rs` | 解析 loop-constraints.md |
| 12 | `src/loop/budget.rs` | 配额检查 |
| 13 | `src/loop/gate.rs` | 路径门禁 |
| 14 | `src/loop/audit.rs` | Readiness Score 计算 |
| 15 | `src/loop/runner.rs` | Loop 执行器（仅 trait + mock） |
| 16 | `src/loop/worktree.rs` | git worktree 封装 |
| 17 | `src/main.rs`（修改） | 加 `loop` 子命令树 |
| 18 | `scripts/loop-doctor.sh` | 10 维评分脚本（维度见 patterns §3.2） |
| 19 | `scripts/loop-accuracy.sh` | L2 解锁条件计算 |
| 20 | `skills/loops/loop-triage/SKILL.md` | /loop-triage skill |
| 21 | `skills/loops/loop-budget/SKILL.md` | /loop-budget skill |
| 22 | `skills/loops/loop-guard/SKILL.md` | /loop-guard skill |
| 23 | `skills/loops/loop-verifier/SKILL.md` | /loop-verifier skill |
| 24 | `skills/loops/loop-init/SKILL.md` | /loop-init skill（脚手架） |
| 25 | `skills/loops/loop-doctor/SKILL.md` | /loop-doctor skill |
| 26 | `.github/workflows/loop-daily-triage.yml` | Actions cron |
| 27 | `.github/workflows/loop-clippy-watch.yml` | on PR |
| 28 | `.github/workflows/loop-release-drafter.yml` | on tag |
| 29 | `.github/workflows/loop-dependency-watch.yml` | cron weekly |
| 30 | `docs/loop-engineering/autoharness-example.md` | Q1=B 对外示例文档 |
| 31 | `examples/minimal-rust-loop/` | Q1=B fork 起点 |

**修改文件**：

| 文件 | 修改内容 |
|---|---|
| `AGENTS.md` | 加"Loop 操作"章节（只读 + 强制约束引用） |
| `README.md` | 加 loop-engineering 徽章 + 简介 |
| `DOCS.md` | 加 loop-engineering 索引（已完成） |
| `PLANS.md` | 加本次计划条目 |
| `src/main.rs` | 加 `loop` 子命令树 |
| `docs/architecture/north-star-metrics.md` | 加 4 个新指标 |

**Phase 1 完成后静态验证**：

```bash
cargo build --release
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
bash scripts/loop-doctor.sh    # 应该输出 L0 或 L1 分数
```

**Phase 1 完成 DoD**：

- [ ] 全部 31 个文件创建
- [ ] `cargo build` 通过
- [ ] `cargo fmt --check` 通过
- [ ] `cargo clippy -D warnings` 通过
- [ ] `cargo test` 通过
- [ ] `loop-doctor.sh` 输出有效 JSON
- [ ] 所有改动已 git commit

### Phase 2：跑第一个 L1（report-only，前 2 周只观察）

工期：14 天（含 review 缓冲）

**触发器**：`.github/workflows/loop-daily-triage.yml`（cron `0 8 * * 1-5`）

**Pattern**：`improvement-loop`，Level L1

**行为**：
1. 读 score.sh 分数
2. 找最低分项
3. 写到 STATE.md "Watch List"
4. **不修改任何代码**

**验证**：
- 14 天人工 review STATE.md
- accuracy 记录在 STATE.md "Accuracy Tracking" 段
- 准确率 ≥ 80% 准备升 L2；< 80% 调 skill prompt 继续 L1

**Q3=A 数据驱动**：14 天 + runs ≥ 10 + accuracy ≥ 80% + 人类签字才解锁 L2。

### Phase 3：升 L2（小修 + maker/checker）

工期：7 天起步

**启用**：复用现有 `HarnessType::{Refiner, Verifier}`（零成本）

**路径限制**：
- ✅ 自动：`docs/**`、`skills/**/SKILL.md`、`tests/**`、`scripts/**`
- ❌ escalate：`src/**`、`Cargo.toml`、`Cargo.lock`、`gate.yaml`、`STATE.md`

**触发条件**：Phase 2 满足 Q3=A 四项门

### Phase 4：加更多 Pattern（独立推进）

每个 Pattern 单独走 L1 → L2 路径：

| Pattern | Phase 4 顺序 | 备注 |
|---|---|---|
| `clippy-fmt-watch` | 第 1 个 | 唯一 on-PR Pattern，最容易量化 |
| `synthesis-quality` | 第 2 个 | 依赖 `cargo bench` 基线 |
| `test-coverage` | 第 3 个 | 依赖 tarpaulin |
| `release-drafter` | 第 4 个 | on-tag 触发，独立 |
| `dependency-watch` | 第 5 个 | 涉及 `cargo audit`，需补依赖 |
| `doc-staleness` | 第 6 个 | 纯 L1，可最后 |

每个 Pattern 上线前必须：
- 在 `patterns/registry.yaml` 注册
- 在 STATE.md 加条目
- 单独跑 ≥ 7 天 L1
- 人类 review 通过

---

## 2. 完整文件清单（31 新增 + 6 修改 = 37 个）

### 2.1 新增文件（31 个）

**根目录配置（6 个）**：
1. `STATE.md`
2. `LOOP.md`
3. `loop-budget.md`
4. `loop-run-log.jsonl`
5. `loop-constraints.md`
6. `gate.yaml`

**Rust 模块（9 个）**：
7. `src/loop/mod.rs`
8. `src/loop/state.rs`
9. `src/loop/log.rs`
10. `src/loop/constraints.rs`
11. `src/loop/budget.rs`
12. `src/loop/gate.rs`
13. `src/loop/audit.rs`
14. `src/loop/runner.rs`
15. `src/loop/worktree.rs`

**Skills（6 个，复用现有 loops category，两级目录：`skills/<category>/<name>/SKILL.md`）**：
16. `skills/loops/loop-triage/SKILL.md`
17. `skills/loops/loop-budget/SKILL.md`
18. `skills/loops/loop-guard/SKILL.md`
19. `skills/loops/loop-verifier/SKILL.md`
20. `skills/loops/loop-init/SKILL.md`
21. `skills/loops/loop-doctor/SKILL.md`

**GitHub Actions（4 个）**：
22. `.github/workflows/loop-daily-triage.yml`
23. `.github/workflows/loop-clippy-watch.yml`
24. `.github/workflows/loop-release-drafter.yml`
25. `.github/workflows/loop-dependency-watch.yml`

**Scripts（2 个）**：
26. `scripts/loop-doctor.sh`
27. `scripts/loop-accuracy.sh`

**Pattern 注册 + 文档（4 个）**：
28. `patterns/registry.yaml`
29. `docs/loop-engineering/autoharness-example.md`（Q1=B）
30. `examples/minimal-rust-loop/README.md`（Q1=B fork 起点）
31. `examples/minimal-rust-loop/SKILL.md`

### 2.2 修改文件（6 个）

1. `AGENTS.md` — 加"Loop 操作"章节（可选：北极星表同步，遵循 AGENTS.md §7.4 指标演进流程）
2. `README.md` — 加 loop-engineering 徽章
3. `DOCS.md` — 加 loop-engineering 索引（已完成，含 audit 文档）
4. `PLANS.md` — 加本次计划条目
5. `docs/architecture/north-star-metrics.md` — 加 4 个新指标 + 变更历史条目
6. `src/main.rs` — 加 `loop` 子命令树（§1 任务表第 17 项）

---

### 2.3 全局 Token 预算（loop-budget.md 依据）

| Pattern | Token/次 | 触发 | 日均摊 |
|---|---|---|---|
| improvement-loop | 50k | 1d | 50k |
| synthesis-quality | 30k | 1d | 30k |
| test-coverage | 30k | 1d | 30k |
| doc-staleness | 20k | 3d | ~7k |
| clippy-fmt-watch | 10k | on-PR | ~2k |
| release-drafter | 15k | on-tag | ~0 |
| dependency-watch | 40k | 1w | ~6k |
| **最坏日合计** | | | **~125k** |

**策略**：全局日预算 **150k**（留 20% 余量）。超预算按优先级降级（P3 停跑 → P2 降频 → P1 保留，优先级见 patterns §6）；单 Pattern 仍按 `loop-budget.md` 的 80% 降级 / 100% 退出。Phase 2 实测后回填真实值。

---

## 3. 风险与边界

### 3.1 风险（要做的事）

| 风险 | 缓解 |
|---|---|
| Loop 改坏 score.sh 分数 | `loop-constraints.md` 写死：run 后 score 不得低于 run 前基线；L2 自动 commit 前先记基线，下降即 `git revert` 该 commit（记录到 run-log） |
| Loop 跑飞（无限循环 / token 爆炸） | `loop-budget.md` + `loop-context --check` 熔断 + kill switch |
| STATE.md 无限膨胀 | 每次 run prune 已解决项；模板里写"30 天清理" |
| 多 loop 互相打架 | `multi-loop.md` 优先级矩阵（仿 loop-engineering） |
| Loop 路径漂移（改 src/core） | `gate.yaml` denylist + `loop-gate check` 在每次 push 前机械验证 |
| 复合性爆炸（loop 越加越多） | `loop-doctor.sh` 评分 ≥ 80 才能启新 loop，否则禁止 |
| L1 永远跑不到 80% | `loop-doctor.sh` 报警 → 调 skill prompt / 加 verifier |
| 人类忘了 review 导致无数据 | workflow 在 STATE.md > 3 天未 review 时发 issue |
| 数据偏差（loop 只发现容易的） | accuracy 用 F-score 替代（漏报权重 2x） |
| 外部工具缺失（tarpaulin / cargo-audit） | Phase 1 检查工具前置；缺失时对应 Pattern 降级 L1 或跳过并在 STATE 标注 |
| 间歇性测试失败被当 bug 修 | flake 规则：同一测试失败 ≥ 2 次才报告；首次失败仅记录，不自动"修复" |

### 3.2 边界（不做的事）

| 边界 | 理由 |
|---|---|
| **不实现 MCP server** | 项目无外部工具集成需求；loop 只读 repo 内文件 |
| **不改 src/engine 任何 Rust 文件** | loop 是元层，引擎是元层的引擎 |
| **不改 src/core/harness.rs** | 核心 trait 是项目灵魂 |
| **不引入新外部依赖** | 复用现有 Cargo 依赖（优先 std + shell）；cargo-tarpaulin / cargo-audit 为可选工具前置，缺失时对应 Pattern 降级 L1 |
| **不替换 GOAL.md** | GOAL.md 是产品目标，loop 是实现目标的机制 |
| **不替换 iterations.jsonl** | code fitness 用它，loop 走 loop-run-log.jsonl |
| **不替换 PLANS.md / DOCS.md** | 已有文档规范保留 |
| **不照搬 npm 包名** | 本项目 Rust 二进制，子命令空间干净 |
| **不仓促上 L3** | 必须 L1 满 2 周、L2 满 1 周、Ready Score ≥ 80 才解锁 L3 |
| **不让 loop 改 GOAL.md / PLANS.md / DOCS.md** | 人类文件，loop 只读 |
| **不让 loop 改任何用户的 commit message** | loop 只 append 到自己的 log |

### 3.3 红线（违反立即停 loop）

- [ ] 单次 run score.sh 下降 > 5 分（硬约束 0 容忍见 loop-constraints.md；此处为立即终止线）
- [ ] 自动 commit 改了 denylist 路径
- [ ] loop-run-log.jsonl 在 24h 内无新条目但 workflow 仍 trigger
- [ ] 任何 PR 出现 "auto-merge" 标签（除非在 allowlist）
- [ ] token 实际开销 > 预算 150%
- [ ] kill switch (pause-all) 已激活但 loop 仍跑
- [ ] 外部工具前置缺失时仍自动跑依赖它的 Pattern

---

## 4. 验收标准（DoD）

### 4.1 Phase 1 完成 DoD

- [ ] 31 个新文件创建
- [ ] `cargo build` 通过
- [ ] `cargo fmt --check` 通过
- [ ] `cargo clippy -D warnings` 通过
- [ ] `cargo test` 通过
- [ ] `bash scripts/loop-doctor.sh` 输出有效 JSON 且分数 ≥ 40（R1 readiness，工具就绪；不触发任何 loop）
- [ ] 所有改动已 git commit（per AGENTS.md "commit as you go"）
- [ ] `loop init .` 在新目录跑通（能 scaffold 出所有文件）
- [ ] `examples/minimal-rust-loop/` 可被另一项目 fork 后用

### 4.2 Phase 2 完成 DoD（14 天 L1）

- [ ] loop-daily-triage.yml 触发次数 ≥ 10
- [ ] STATE.md 至少 10 条 "Recent Runs"
- [ ] 人工 review 记录在 STATE.md "Accuracy Tracking" 段
- [ ] Q3=A 四门全过：L1 ≥ 14 天 + runs ≥ 10 + accuracy ≥ 80% + 人类签字（满足则准备 Phase 3）
- [ ] 任何 score 退化被自动检测并 revert
- [ ] token 用量未超预算 80%

### 4.3 Phase 3 完成 DoD（7 天 L2 起步）

- [ ] L2 unlock approved 签字在 STATE.md
- [ ] 至少 1 个 PR 由 L2 自动 commit + push
- [ ] 该 PR 通过所有 CI 检查（fmt/clippy/test）
- [ ] 该 PR 经人类 review 后合并
- [ ] Verifier sub-agent 至少独立 mark 1 次 done

### 4.4 Phase 4 完成 DoD（每个 Pattern）

- [ ] 在 `patterns/registry.yaml` 注册
- [ ] 至少跑 7 天 L1
- [ ] accuracy ≥ 80% 才升 L2
- [ ] L2 至少 1 个自动 PR 走通

---

## 5. 不做的事的清单（重要）

按 AGENTS.md 的反例章节的镜像：

- ❌ **不**在 Phase 1 之前跑任何 loop（必须先有工具和约束文件）
- ❌ **不**改 `src/core/harness.rs`（核心 trait 不许 loop 碰）
- ❌ **不**改 `src/engine/synthesis.rs`（核心引擎不许 loop 碰）
- ❌ **不**复制 `patterns/*.md` 文件名（loop-engineering 是 npm 包；我们用 `patterns/registry.yaml` 注册即可）
- ❌ **不**把 STATE.md / LOOP.md 写成"漂亮 PPT"——它们是运行态机器可读优先
- ❌ **不**让 loop 改 `GOAL.md` / `PLANS.md` / `DOCS.md`
- ❌ **不**让 loop 改任何用户的 commit message（loop 只 append 到自己的 log）
- ❌ **不**在第一次就跑 L2（数据未积累）
- ❌ **不**让 loop 修改 `Cargo.toml` / `Cargo.lock`（在 denylist）
- ❌ **不**用 npm 工具（我们是 Rust 项目）

---

## 6. 关键时间节点

| 节点 | 日期 | 标志 |
|---|---|---|
| 方案完成 | 2026-08-06 | 本文档定稿 |
| Phase 1 开工 | 待"可以开始" | 用户确认后开始 31 个文件创建 |
| Phase 1 完成 | 开工后 ~3 天 | DoD 满足 + git commit |
| Phase 2 启动 | Phase 1 后立即 | Actions cron 启动 |
| Phase 2 完成 | Phase 2 启动后 14 天 | Q3=A 四门全过（见 §4.2） |
| Phase 3 启动 | Phase 2 完成后立即 | L2 unlock 签字后 |
| Phase 3 完成 | Phase 3 启动后 7 天 | 首个 L2 PR 合并 |
| Phase 4 全展开 | Phase 3 完成后 | 6 个 Pattern 逐个上线 |

---

## 7. 与 loop-engineering 上游的差异

| 维度 | loop-engineering 上游 | AutoHarness 版 |
|---|---|---|
| 语言 | TypeScript / npm | Rust / cargo |
| 调度 | Grok /loop + Claude cron | GitHub Actions cron |
| 工具分发 | npx @cobusgreyling/loop | cargo run -- loop-* |
| 示例项目 | reference repo 自身 | AutoHarness 自身（Q1=B） |
| 多 loop 协调 | docs/multi-loop.md | 同（rules 沿用） |
| 模式目录 | 7 个标准模式 | 7 个 AutoHarness 定制模式 |
| Skill 体系 | 自创 | 复用 AI-Builder-Club 11 个 skill |
| 评分体系 | loop-audit 1.7（10 维） | loop-doctor.sh（10 维，沿用） |
| Kill switch | label + flag | label + STATE.md flag |

---

## 8. 实施 SOP（Phase 1 开工后第一步）

```bash
# 1. 创建 plans/ 计划文件
touch plans/2026-08-06_loop-engineering-integration.md
# (在 PLANS.md 加索引条目)

# 2. 创建目录
mkdir -p src/loop skills/loops/loop-triage skills/loops/loop-budget \
         skills/loops/loop-guard skills/loops/loop-verifier \
         skills/loops/loop-init skills/loops/loop-doctor \
         patterns examples/minimal-rust-loop

# 3. 创建 31 个文件（按清单顺序）

# 4. 修改 5 个现有文件

# 5. 静态验证
cargo build
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test

# 6. loop-doctor 自检
bash scripts/loop-doctor.sh

# 7. 提交
git add .
git commit -m "[loop-engineering] Phase 1: scaffold 31 files + 6 edits, R0 readiness"

# 8. 不要激活任何 Actions workflow（Phase 2 才激活）
```

---

**主方案** → [integration-plan.md](integration-plan.md)
**Patterns 与等级** → [patterns-and-levels.md](patterns-and-levels.md)
**入口** → [README.md](README.md)

**最后更新**：2026-08-06