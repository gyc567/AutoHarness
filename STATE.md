# Loop State — AutoHarness

> **运行态文件**（Operational）。loop 自动写入；人类只读 / review / override。
> 区别于 `GOAL.md`（目标态 Target）：GOAL 是产品目标，STATE 是 loop 当下状态。

## Phase 2 Kickoff（2026-08-06）

- ✅ Phase 1 完成（loop Readiness R3, code quality 100/100）
- 🚀 Phase 2 开始：improvement-loop Pattern L1 report-only 启动
- 📋 14 天观察期；按 Q3=A 数据驱动解锁 L2
- ⏸ **不**激活 `.github/workflows/loop-daily-triage.yml`（手动触发）

## Human Inbox（loop 需要人介入的项）

> loop 在 escalation 时追加；人类 review 后移除。

<!-- Loop appends above -->

### 🚨 [audit-2026-08-06] CRITICAL findings (6 项)

> **报告**：`docs/audit/code-audit-2026-08-06.md`
> **执行者**：loop-engineering audit (Maker + Checker + Critic)
> **Score gate**：100/100 维持（C1+C2+H2+H3 修复后无退化）

P0 立刻修：
- [x] **[C1]** `src/sandbox/executor.rs:135` ~~shell 注入~~ — 已修复：validate_code 在写盘前执行（`4591bae`）
- [x] **[C2]** ~~`shell_escape()` 字符集不全~~ — 已修复：19 字符覆盖（`4591bae`）
- [x] **[H2]** ~~`chrono_like_now()` 输出 `ts-{secs}` 而非 ISO 8601~~ — 已修复：`Utc::now()` 替换（`4591bae`）
- [x] **[H3]** ~~`generate_run_id()` 破 schema~~ — 已修复：`YYYYMMDDTHHMMSSZ-NNN`（`4591bae`）

P1 本周：
- [x] **[C3]** ~~`use_seccomp/use_cgroups/use_namespaces: true` 默认但**未实际应用**~~ — 已修复：默认值改为 `false` + validate() 警告 + struct 文档说明（`4591bae` 后追加）
- [x] **[C4]** ~~`working_directory` 不验证~~ — 已修复：`validate()` 中增加绝对路径、存在性、`..` 检查（`4591bae` 后追加）
- [x] **[H4]** ~~`gate.yaml` 缺 8 个 src/ 路径 denylist~~ — 已修复：补充 `executor.rs config.rs limits.rs search.rs error.rs state.rs loop/mod.rs loop/gate.rs`（`4591bae` 后追加）

P2 Phase 4：
- [x] **[C5]** ~~`validate_code` 黑名单过弱~~ — 已修复：元字符全部拦截 + backtick/unquoted-$ + eval/exec 模式；由 C1/C2 一并解决
- [x] **[H1]** ~~7 个 harness template 是 stub~~ — 已改进：每个模板增加 Purpose、When to Use、Success Patterns、Common Errors 节（`6f39f48`）
- [x] **[M1]** ~~3 个未使用依赖~~ — 已移除：`duct`, `notify`, `metrics`（`6f39f48`）

P3 未来：
- [x] **[M2]** ~~README badge 数据陈旧~~ — 已更新：88/100 → 100/100（`6f39f48`）
- [x] **[M3]** ~~无 CHANGELOG.md~~ — 已创建：CHANGELOG.md（`6f39f48`）
- [x] **[LE2]** ~~LOOP.md vs STATE.md 责任重叠~~ — 已明确：LOOP.md 是"谁在跑"，STATE.md 是"当前状态"（`6f39f48`）
- [x] **[LE4]** ~~kill switch 用 grep 不稳健~~ — 已重构：`contains()` → section-aware KV 解析（`6f39f48`）

## High Priority (loop is acting or waiting on human)

<!-- Loop appends above -->

## Watch List

<!-- Loop appends above -->

- [ts-1785980641] **safety**: 7/10 — Remove or justify `unsafe` blocks (target 10/10)

- [ts-1785988416] **safety**: 7/10 — Remove or justify `unsafe` blocks (target 10/10)

- [2026-08-06T14:47:35Z] **safety**: 7/10 — Remove or justify `unsafe` blocks (target 10/10)

## Recent Noise (ignored this run)

<!-- Loop appends above -->

- High-noise: dependabot PRs surfaced again — add to ignore list
- False positives: 1 CI flake (known flaky test)
- Deprioritize: lint warnings moved to Watch List
- Friction: triage missed nightly deploy failure (was infra, not code)
- Adjustment: include infra check status in scan

## Post-Run Critique (from last run)

<!-- Loop appends above -->

## Kill Switch

- pause-all: false                  # 置 true 后所有 loop 立即停止
- pause-improvement-loop: false
- pause-clippy-fmt-watch: false
- pause-synthesis-quality: false
- pause-test-coverage: false
- pause-doc-staleness: false
- pause-release-drafter: false
- pause-dependency-watch: false

## Recent Runs

<!-- Loop appends above -->

## Accuracy Tracking（Phase 2 起启用，loop-accuracy.sh 解析）

<!-- Human appends review records above -->

- L2-unlock-approved: <date> <signer>   # Q3=A 门 4，人类签字后出现

## Metrics（可选，loop 写入）

<!-- Loop appends above -->

---

**Read by**: `.github/workflows/loop-daily-triage.yml`（Phase 2+ 激活）
**Last updated**: 2026-08-06 (audit run, Human Inbox restored)
