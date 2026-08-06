# Loop State — AutoHarness

> **运行态文件**（Operational）。loop 自动写入；人类只读 / review / override。
> 区别于 `GOAL.md`（目标态 Target）：GOAL 是产品目标，STATE 是 loop 当下状态。

## Human Inbox（loop 需要人介入的项）

> loop 在 escalation 时追加；人类 review 后移除。

<!-- Loop appends above -->

## High Priority (loop is acting or waiting on human)

<!-- Loop appends above -->

## Watch List

<!-- Loop appends above -->

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

**Read by**: `.github/workflows/loop-daily-triage.yml`
**Last updated**: <由 loop 自动更新>