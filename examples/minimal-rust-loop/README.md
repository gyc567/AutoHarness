# Minimal Rust Loop — Fork 起点

> **Q1=B 对外示例**：其他 Rust 项目可 fork 本目录作为 loop-engineering 接入起点。

## 包含

- `README.md`（本文件）
- `SKILL.md` — `/loop-init-minimal-rust` skill 描述

## 不包含（按需 fork 后补）

- `gate.yaml` — 按项目自身 denylist/allowlist 复制
- `loop-constraints.md` — 按项目硬约束复制
- `patterns/registry.yaml` — 按项目 Pattern 列表复制
- GitHub Actions workflows — 按项目 cadence 复制

## 5 分钟接入

```bash
# 1. 复制本目录到目标项目
cp -r examples/minimal-rust-loop/* /path/to/your-project/

# 2. 在目标项目根目录跑 loop init
cd /path/to/your-project
cargo run -- loop init .

# 3. 检查 Readiness
cargo run -- loop doctor .

# 4. 复制本项目的 templates（README + LICENSE）
# 5. 修改项目名、Pattern IDs
# 6. 提交
git add . && git commit -m "Add loop-engineering scaffolding"
```

## 最小可工作配置

仅需 6 个文件：

1. `STATE.md` — 运行态
2. `LOOP.md` — 活动 loop 清单
4. `loop-budget.md` — 配额 + kill switch
5. `loop-run-log.jsonl` — 追加日志（touch 创建即可）
6. `loop-constraints.md` — 硬约束
7. `gate.yaml` — 路径门禁

模板见 [docs/loop-engineering/integration-plan.md §3.4](../../docs/loop-engineering/integration-plan.md#34-机器文件-schema-与模板)。

## 与 AutoHarness 的差异

| 维度 | AutoHarness | Minimal Rust Loop |
|---|---|---|
| `src/loop/` 模块 | ✅ 完整实现 | ❌ 不含（fork 后加） |
| 7 个 Pattern | ✅ | ❌（按需选） |
| GitHub Actions | ✅ 4 个 | ❌（按需复制） |
| Score (code fitness) | ✅ `scripts/score.sh` | ❌（用项目自己的） |
| 北极星指标 | ✅ 4 项 | ❌（按需选） |

## 关键设计继承

- **三层写入规则**：PR 分支内可自动 commit → allowlist 路径可自动 merge → main 永不自动 merge
- **L1 优先**：前 2 周只观察，不动代码
- **maker/checker 分离**：Refiner + Verifier + Critic 双 gate
- **kill switch**：`STATE.md` 的 `pause-all` 标志
- **数据驱动 L2 解锁**：14 天 + ≥10 次 L1 + accuracy ≥ 80% + 人工签字

## 相关文档

- [docs/loop-engineering/README.md](../../docs/loop-engineering/README.md) — 入口
- [docs/loop-engineering/integration-plan.md](../../docs/loop-engineering/integration-plan.md) — 主方案
- [docs/loop-engineering/implementation-roadmap.md](../../docs/loop-engineering/implementation-roadmap.md) — 实施路线

---

*最后更新：2026-08-06*
*关联上游 [loop-engineering](https://github.com/cobusgreyling/loop-engineering)*