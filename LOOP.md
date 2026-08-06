# Loop 清单

> **活动 loop 清单 + 心跳表**。loop 自动更新；人类可读可编辑。
>
> **职责边界**：
> - `LOOP.md` — **清单与心跳**：哪些 loop 在跑、cadence、活跃状态、心跳记录
> - `STATE.md` — **运行态数据**：Human Inbox、Watch List、Recent Noise、Kill Switch、Metrics
> 这两个文件互补、不重叠。LOOP.md 是"谁在跑"，STATE.md 是"当前状态"

## Active Loops

| Pattern ID | Level | Cadence | 最近 run | 心跳 OK | Status |
|---|---|---|---|---|---|
| improvement-loop | L1 | 1d | ts-1785980641 | ✅ | active (run=run-1785980641-1) |
| daily-triage | L1 | 1d | — | ⏳ | scaffolded 2026-08-06 |

## Pattern 注册

完整注册见 [`patterns/registry.yaml`](patterns/registry.yaml)。

## Human Gates

- No auto-fix until L2 checklist complete
- All high-risk paths: human review required

## Budget

- Max sub-agent spawns per run: 0 (L1) / 2 (L2)
- Max tokens/day: 100k
- Kill switch: `pause-all: true` in STATE.md — pause all loops and notify human

## 心跳表（loop-doctor 用）

> 每 24h 至少 1 条 `loop-run-log.jsonl` 记录，否则 R1 readiness 扣分。

| 日期 | 写入条目数 | 心跳 OK |
|---|---|---|
| improvement-loop | L1 | 1d | ts-1785988400 | ✅ | active (run=run-1785988400-1) |

---

**Last updated**: 2026-08-06 (daily-triage scaffolded)

_last_run: ts-1785988416

_last_run: 2026-08-06T14:47:35Z
