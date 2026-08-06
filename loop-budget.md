# Loop Budget

> **Token 配额 + Kill Switch**。人类写；loop 只读。
> 详细 schema 见 [integration-plan.md §3.4](../../docs/loop-engineering/integration-plan.md#34-机器文件-schema-与模板)

## 全局配额

| 范围 | 预算 | 检查时机 |
|---|---|---|
| 全局日 token | 150000 | 每次 run 前 |
| 单 Pattern 日 token | 见 `patterns/registry.yaml` `suggested_daily_cap` | 每次 run 前 |
| 80% 阈值 | 降级 L1 report-only | 达 80% 时 |
| 100% 阈值 | 立即退出 + log | 达 100% 时 |
| 月 token | 3000000 | 每月 1 日 |

## 单 Pattern 配额

详见 [patterns/registry.yaml](../../patterns/registry.yaml)。每个 Pattern 必须设 `suggested_daily_cap`。

## 超预算行为

1. **80%-99%**：自动降级 L1 report-only（仅 STATE.md 写入，不动代码）
2. **100%-149%**：立即退出 + 写 `loop-run-log.jsonl`（status=failed, reason=budget）
3. **≥ 150%**：触发红线（见 [implementation-roadmap.md §3.3](../../docs/loop-engineering/implementation-roadmap.md#33-红线违反立即停-loop)）

## 降级优先级

冲突时按 [patterns-and-levels.md §6 优先级矩阵](../../docs/loop-engineering/patterns-and-levels.md#6-多-loop-协调矩阵来自-loop-engineering-multi-loopmd)：

1. 任何 Pattern 发现 CI 红 → 立即阻塞，进 STATE.md Human Inbox
2. clippy-fmt-watch（PR 阻塞，时效性最高）
3. improvement-loop（分数是北极星）
4. dependency-watch（CVE 有时效性）
5. synthesis-quality / test-coverage / doc-staleness / release-drafter（非阻塞巡检）

预算不足时：**P3 → P2 → P1** 逐级停跑。

## Kill Switch

```yaml
pause-all: false                  # 置 true 后所有 loop 立即停止
pause-improvement-loop: false
pause-clippy-fmt-watch: false
pause-synthesis-quality: false
pause-test-coverage: false
pause-doc-staleness: false
pause-release-drafter: false
pause-dependency-watch: false
```

激活方式（任一即可）：

1. 在本文件顶部将 `pause-all` 置为 `true` 后 commit
2. 给 PR/issue 加 `loop-pause-all` 标签
3. GitHub Actions 手动 dispatch 并传 `pause=true`

---

**Last updated**: 2026-08-06