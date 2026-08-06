# Loop Constraints — AutoHarness

> **强制约束**。人类写；loop 每次启动**必须**读取并强制执行。
> 详细来源：[integration-plan.md §6.2](../../docs/loop-engineering/integration-plan.md#62-loop-constraintsmd人类读--skill-解析)

## Push & Merge

- **不要 push 前不说**（**例外**：loop-bot 的 L1 报告 push 仅限 `STATE.md` / `LOOP.md` / `loop-run-log.jsonl`，是唯一自动 push 例外）
- main 上**绝不**自动 merge；永远先开 draft PR
- **三层写入规则**：
  ① PR 分支内可自动 commit（如 fmt 修复）
  ② allowlist 路径可自动 merge（draft PR 人审后）
  ③ main **永不**自动 merge

## Paths

- **永不改**（denylist）：`.env` / `.env.*` / `Cargo.lock` / `src/core/*` / `src/engine/*` / `src/main.rs`
- 改 `docs/**` 可自动
- 改 `skills/**/SKILL.md` 可自动
- 改 `tests/**` 可自动
- 改 `scripts/**` 可自动（不含 score.sh 行为变更）
- `gate.yaml` 与 `loop-constraints.md` 本身**只能**人类改

## Code

- 改前**必跑**：`cargo fmt` → `cargo clippy` → `cargo test`
- 每项 fix 最多 **3 次**尝试，超过 escalate 到 STATE.md Human Inbox
- 单次 run 只修一处
- **score 退化硬约束 0 容忍**：run 后 score.sh `total` 不得低于 run 前基线；L2 自动 commit 前先记基线，下降即 `git revert` 该 commit（记录到 run-log）
- **flake 规则**：同一测试失败 ≥ 2 次才报告；首次失败仅记录在 STATE.md Recent Noise，不自动"修复"

## Budget

- token 用到 80% 切 L1 report-only
- token 用到 100% 立即退出 + 写 log
- kill switch (`pause-all`) 激活**立即**退出

## Score

- **不许降低** `score.sh` 分数（硬约束 0 容忍）
- **不许回退** 已达成的 milestone
- 红线：单次 run score 下降 > 5 分 → 立即终止 loop（见 [implementation-roadmap.md §3.3](../../docs/loop-engineering/implementation-roadmap.md#33-红线违反立即停-loop)）

## Escalation

- escalation 自动写入 `STATE.md` 的 **Human Inbox** 段
- GitHub Actions workflow 需 `permissions: issues: write` 开 issue 通知
- 3 次 fix 失败 → 强制 escalate，不再尝试

---

<!-- Repo-specific rules above. Add your own below. -->

**Last updated**: 2026-08-06