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

## High Priority (loop is acting or waiting on human)

<!-- Loop appends above -->

## Watch List

<!-- Loop appends above -->

- [ts-1785980641] **safety**: 7/10 — Remove or justify `unsafe` blocks (target 10/10)

## Recent Noise (ignored this run)

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
**Last updated**: <由 loop 自动更新>