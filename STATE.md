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
> **Score gate**：维持 100/100（未修改任何代码）

P0 立刻修：
- [ ] **[C1]** `src/sandbox/executor.rs:135` shell 注入 — 用户 code 直接拼入 shell 脚本后执行
- [ ] **[C2]** `shell_escape()` 字符集不全 — 漏 `& ; | > < ( )` 等 metacharacter
- [ ] **[H2]** `chrono_like_now()` 输出 `ts-{secs}` 而非 ISO 8601 — `loop-run-log.jsonl` schema 破坏
- [ ] **[H3]** `generate_run_id()` 输出 `run-{secs}-{n}` 而非 `YYYYMMDDTHHMMSSZ-NNN`

P1 本周：
- [ ] **[C3]** `use_seccomp/use_cgroups/use_namespaces: true` 默认但**未实际应用** — 误导性配置
- [ ] **[C4]** `working_directory` 不验证 — sandbox 失效风险
- [ ] **[H4]** `gate.yaml` 缺 8 个 src/ 路径 denylist（含 sandbox/executor.rs）

P2 Phase 4：
- [ ] **[C5]** `validate_code` 黑名单过弱
- [ ] **[H1]** 7 个 harness template 是 stub（含 `TODO`）
- [ ] **[M1]** 3 个未使用依赖（`duct`, `notify`, `metrics`）

P3 未来：
- [ ] **[M2]** README badge 数据陈旧 (88/100 → 实际 93/100)
- [ ] **[M3]** 无 CHANGELOG.md
- [ ] **[LE2]** LOOP.md vs STATE.md 责任重叠
- [ ] **[LE4]** kill switch 用 grep 不稳健

## High Priority (loop is acting or waiting on human)

<!-- Loop appends above -->

## Watch List

<!-- Loop appends above -->

- [ts-1785980641] **safety**: 7/10 — Remove or justify `unsafe` blocks (target 10/10)

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
