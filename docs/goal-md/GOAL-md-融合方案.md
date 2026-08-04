# GOAL.md 模式融合方案

**版本**: 0.1.0
**日期**: 2026-08-04
**状态**: 草稿

---

## 一、现状审计

### 1.1 现有文件分析

| 文件 | 状态 | 问题 |
|------|------|------|
| `GOAL.md` | ✅ 已创建 | 1. 指标定义过于简单<br>2. 缺少北极星指标集成<br>3. Action Catalog 不完整 |
| `scripts/score.sh` | ✅ 已创建 | 1. 测试部分超时问题<br>2. 缺少 JSON 输出格式<br>3. 未集成北极星指标 |
| `iterations.jsonl` | ✅ 已创建 | 1. 格式不规范<br>2. 缺少 timestamp 字段<br>3. 未记录 score 组件详情 |
| `CLAUDE.md` | ❌ 缺失 | **关键缺失** |
| `template/GOAL.md` | ❌ 缺失 | **关键缺失** |
| `examples/` | ❌ 缺失 | **关键缺失** |

---

## 二、问题清单

### 问题 1: GOAL.md 与北极星指标脱节

**现状**: AGENTS.md 定义了北极星指标，但 GOAL.md 没有引用。

```markdown
# AGENTS.md 定义的北极星指标
| 指标名称 | 目标 |
|----------|------|
| 代码合成成功率 | ≥ 85% |
| 平均合成时间 | < 500ms |
| 测试覆盖率 | ≥ 80% |
| CLI 响应时间 | < 100ms |

# GOAL.md 的指标
format + clippy + tests + docs + maintenance
```

**问题**: 两套指标体系独立运行，Agent 可能优化了 GOAL.md 分数但损害了北极星指标。

**建议**: GOAL.md 应作为北极星指标的落地工具，而非独立体系。

### 问题 2: 评分脚本缺少基准测试集成

**现状**: `scripts/score.sh` 没有集成 `cargo bench`。

**问题**: 无法测量性能相关的北极星指标（合成时间、CLI 响应时间）。

### 问题 3: 迭代日志格式不规范

**现状**:

```jsonl
{"iteration":1,"before":15,"after":20,"action":"Fix clippy","result":"kept","note":"..."}
```

**问题**:
- 缺少 `timestamp` 字段
- 缺少 `component` 字段
- 缺少 `branch` 字段

### 问题 4: 缺少 Agent 指导文件

**现状**: 没有 `CLAUDE.md`。

**问题**: Agent 不了解 GOAL.md 模式的具体使用方式。

### 问题 5: 模板和示例缺失

**现状**: 没有可复用的模板和示例。

**问题**: 其他项目无法参考 AutoHarness 来创建自己的 GOAL.md。

---

## 三、优化后的方案

### 3.1 目标定位

**双层目标结构**:


---

## 四、详细设计

### 4.1 优化后的 GOAL.md

```markdown
# Goal: Make AutoHarness production-ready

AutoHarness is an AI-powered test harness synthesizer.

## Fitness Function

```bash
./scripts/score.sh          # 代码质量评分
./scripts/bench.sh           # 性能基准评分
./scripts/coverage.sh        # 覆盖率评分
./scripts/score.sh --all    # 完整评分
```

### Metric Definition

```
total_score = quality_score + performance_score + coverage_score

quality_score = format(20) + clippy(20) + docs(15) + safety(10)
performance_score = synthesis_time(25) + cli_response(25)
coverage_score = line_coverage(50)
```

**与北极星指标对齐**:

| 北极星指标 | GOAL.md 组件 | 目标 |
|-----------|-------------|------|
| 代码合成成功率 ≥ 85% | 测试通过率 | 100% |
| 平均合成时间 < 500ms | synthesis_time | < 500ms |
| 测试覆盖率 ≥ 80% | line_coverage | ≥ 80% |
| CLI 响应时间 < 100ms | cli_response | < 100ms |

### Metric Mutability

- [x] **Open** — 指标定义可以调整，但需要记录变更原因
- **变更流程**: 在 `docs/goal-md/指标变更记录.md` 中记录

## Operating Mode

- [x] **Converge** — 当所有北极星指标达标时停止

### Stopping Conditions

Stop when ALL of:
- quality_score = 100/100
- performance_score = 100/100
- coverage_score ≥ 80/100

Or ANY of:
- 30 iterations completed
- 10 consecutive iterations with no improvement

## Bootstrap

```bash
cargo fetch
./scripts/score.sh --all > baseline.txt
# Record: 质量分数、性能分数、覆盖率
```

## Improvement Loop

```
repeat:
  0. Read iterations.jsonl — note what's been tried
  1. ./scripts/score.sh --all --json > /tmp/before.json
  2. Parse scores — find weakest component
  3. If quality < 100: work on quality components
  4. If performance < 100: run benchmarks, profile
  5. If coverage < 80: add tests
  6. Make the change
  7. Verify: targeted test
  8. ./scripts/score.sh --all --json > /tmp/after.json
  9. Compare: if improved, commit; if regressed, revert
  10. Append to iterations.jsonl
  11. Continue
```

## Action Catalog

### format (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Run cargo fmt | +20 | `cargo fmt -- --edition 2021` |

### clippy (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Fix warnings | +5-15 | `cargo clippy --fix` |
| Add clippy config | +5 | Add `[lints.clippy]` to Cargo.toml |

### docs (target: 15/15)

Maintain it.

### safety (target: 10/10)

| Action | Impact | How |
|--------|--------|-----|
| Audit unsafe blocks | +5 | Review all unsafe code |
| Document safety invariants | +5 | Add safety comments |

### synthesis_time (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| Profile synthesis hot path | +5-10 | `cargo flamegraph` |
| Cache type information | +5-10 | Add LRU cache |
| Parallelize independent tasks | +5 | Use Rayon |

### cli_response (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|


## 五、新增文件清单

### 5.1 核心文件

| 文件路径 | 说明 | 优先级 | 依赖 |
|----------|------|--------|------|
| `CLAUDE.md` | Agent 指导文件 | P0 | 无 |
| `template/GOAL.md` | 标准 GOAL.md 模板 | P0 | 无 |
| `scripts/bench.sh` | 性能基准评分脚本 | P1 | bench exists |
| `scripts/coverage.sh` | 覆盖率评分脚本 | P1 | tarpaulin |
| `examples/01-rust-code-quality.md` | Rust 代码质量示例 | P1 | template |

### 5.2 扩展文件

| 文件路径 | 说明 | 优先级 |
|----------|------|--------|
| `examples/02-test-synthesis.md` | 测试合成示例 | P2 |
| `examples/03-cli-tool.md` | CLI 工具示例 | P2 |
| `examples/04-library.md` | 库项目示例 | P2 |
| `scripts/goal-init.sh` | 交互式初始化脚本 | P2 |
| `docs/goal-md/快速开始.md` | 使用指南 | P2 |
| `docs/goal-md/API-参考.md` | API 文档 | P3 |

---

## 六、评分脚本设计

### 6.1 score.sh (优化版)

```bash
#!/bin/bash
# AutoHarness Code Quality Fitness Function

JSON_OUTPUT=false
ALL_OUTPUT=false
[[ "$1" == "--json" ]] && JSON_OUTPUT=true
[[ "$1" == "--all" ]] && ALL_OUTPUT=true

# ... 评分逻辑 ...

# JSON 输出格式 (规范化)
{
  "timestamp": "2026-08-04T12:00:00Z",
  "version": "0.1.0",
  "total": 85,
  "max": 100,
  "components": {
    "format": {"score": 20, "max": 20, "status": "pass"},
    "clippy": {"score": 20, "max": 20, "status": "pass"},
    "docs": {"score": 15, "max": 15, "status": "pass"},
    "safety": {"score": 10, "max": 10, "status": "pass"}
  }
}
```

### 6.2 bench.sh (新增)

```bash
#!/bin/bash
# AutoHarness Performance Fitness Function

JSON_OUTPUT=false
[[ "$1" == "--json" ]] && JSON_OUTPUT=true

# 运行基准测试
# 测量: synthesis_time, cli_response

# 输出:
{
  "timestamp": "2026-08-04T12:00:00Z",
  "synthesis_time": {"value": 450, "unit": "ms", "target": 500},
  "cli_response": {"value": 80, "unit": "ms", "target": 100},
  "score": 100,
  "max": 100
}
```

### 6.3 coverage.sh (新增)

```bash
#!/bin/bash
# AutoHarness Coverage Fitness Function

# 使用 cargo tarpaulin
# 测量: line_coverage, branch_coverage

# 输出:
{
  "timestamp": "2026-08-04T12:00:00Z",
  "line_coverage": {"value": 75, "target": 80},
  "branch_coverage": {"value": 60, "target": 60},
  "score": 75,
  "max": 100
}
```

---

## 七、迭代日志格式 (规范化)

### 7.1 JSONL 格式规范

```jsonl
{"iteration":1,"timestamp":"2026-08-04T12:00:00Z","component":"clippy","before":15,"after":20,"action":"Fix sort_by_key warnings","result":"kept","note":"Replaced sort_by with sort_by_key","commit":"abc1234"}
{"iteration":2,"timestamp":"2026-08-04T12:05:00Z","component":"format","before":0,"after":20,"action":"Run cargo fmt","result":"kept","note":"Fixed formatting on 5 files","commit":"def5678"}
```

### 7.2 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| iteration | int | 是 | 迭代编号 |
| timestamp | ISO8601 | 是 | 时间戳 |
| component | string | 是 | 组件名称 |
| before | int | 是 | 改前分数 |
| after | int | 是 | 改后分数 |
| action | string | 是 | 采取的行动 |
| result | enum | 是 | kept/reverted/failed |
| note | string | 否 | 备注 |
| commit | string | 否 | 提交 hash |
| branch | string | 否 | 分支名称 |

---

## 八、约束与边界条件

### 8.1 约束条件

1. **北极星指标不可妥协**: 
   - 测试覆盖率必须 ≥ 80%
   - 合成时间必须 < 500ms
   - 即使 GOAL.md 分数满分，北极星指标不达标也不能停止

2. **性能不退化**:
   - benchmark 结果不能比 baseline 差 10% 以上
   - 每次性能相关改动后必须运行 bench.sh

3. **测试完整性**:
   - 所有 cargo test 必须通过
   - 新功能必须有对应测试

### 8.2 边界条件

1. **评分脚本超时**: 
   - 设置 120s 超时
   - 超时计为 0 分但不影响其他组件

2. **竞态条件**:
   - 使用 flock 防止并发执行
   - 评分期间锁定 iterations.jsonl

---

## 九、风险评估

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| 评分脚本执行时间过长 | 中 | 中 | 添加超时控制，分批执行 |
| Agent 优化分数而非实际质量 | 中 | 高 | 添加北极星指标约束 |
| 多分支场景混乱 | 低 | 中 | 迭代日志记录 branch |
| 脚本依赖缺失 | 低 | 高 | Bootstrap 检查依赖 |

---

## 十、实施计划

### Phase 1: 基础建设 (Week 1)

| 任务 | 负责人 | 交付物 |
|------|--------|--------|
| 创建 CLAUDE.md | AI Agent | CLAUDE.md |
| 创建 template/GOAL.md | AI Agent | template/GOAL.md |
| 优化 scripts/score.sh | AI Agent | score.sh (优化版) |
| 建立基线 | AI Agent | baseline.json |

### Phase 2: 性能集成 (Week 2)

| 任务 | 负责人 | 交付物 |
|------|--------|--------|
| 创建 scripts/bench.sh | AI Agent | bench.sh |
| 创建 scripts/coverage.sh | AI Agent | coverage.sh |
| 运行性能基线测试 | AI Agent | benchmark-baseline.json |

### Phase 3: 示例和文档 (Week 3)

| 任务 | 负责人 | 交付物 |
|------|--------|--------|
| 创建 examples/ | AI Agent | 4 个示例文件 |
| 创建 docs/goal-md/ | AI Agent | 2 个指南文件 |
| 更新 DOCS.md 索引 | AI Agent | DOCS.md 更新 |

---

## 十一、验收标准

### 11.1 交付物验收

- [ ] CLAUDE.md 存在且完整
- [ ] template/GOAL.md 可直接使用
- [ ] scripts/*.sh 均可执行
- [ ] examples/ 包含 ≥ 3 个示例

### 11.2 功能验收

- [ ] `./scripts/score.sh --json` 输出规范 JSON
- [ ] `./scripts/bench.sh --json` 输出规范 JSON
- [ ] `./scripts/coverage.sh --json` 输出规范 JSON
- [ ] `iterations.jsonl` 格式正确

### 11.3 集成验收

- [ ] GOAL.md 与北极星指标对齐
- [ ] AGENTS.md 引用 GOAL.md
- [ ] DOCS.md 索引包含 GOAL.md 相关文档

---

**文档状态**: 草稿
**下次审查**: 2026-08-11
**变更历史**: 见 git log

| Lazy load modules | +10 | Defer non-essential imports |
| Optimize startup | +10 | Remove blocking init |
| Stream first output | +5 | Bufferless stdout |

### line_coverage (target: ≥80)

| Action | Impact | How |
|--------|--------|-----|
| Add unit tests | +5-20 | Test uncovered modules |
| Add integration tests | +5-10 | Test CLI workflows |

## Constraints

1. **北极星指标优先** — GOAL.md 分数不能以牺牲北极星指标为代价
2. **测试必须通过** — 所有 cargo test 必须通过
3. **性能不退化** — benchmark 结果不能比 baseline 差 10% 以上
4. **一个提交一个改动** — 原子提交，便于 bisect

## File Map

| File | Role | Editable? |
|------|------|-----------|
| `scripts/*.sh` | 评分脚本 | Yes |
| `iterations.jsonl` | 迭代日志 | Append only |
| `AGENTS.md` | Agent 指南 | Yes |
| `src/**/*.rs` | 源代码 | Yes |


```
┌─────────────────────────────────────────────────────────┐
│                    北极星指标 (North Star)               │
│  代码合成成功率 ≥ 85% | 合成时间 < 500ms | 覆盖率 ≥ 80% │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│                    GOAL.md (Fitness Function)            │
│         format + clippy + tests + docs + safety          │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│                 具体改进 (Action Catalog)                │
│              格式化 + Lint + 测试 + 文档                 │
└─────────────────────────────────────────────────────────┘
```

### 3.2 目录结构

```
AutoHarness/
├── GOAL.md                        # 项目自身 GOAL.md (已优化)
├── CLAUDE.md                      # Agent 指导文件 🆕
├── scripts/
│   ├── score.sh                   # 代码质量评分 (已优化)
│   ├── bench.sh                   # 性能基准评分 🆕
│   ├── coverage.sh                # 覆盖率评分 🆕
│   └── goal-init.sh               # GOAL.md 初始化脚本 🆕
├── template/                      # 🆕 可复用模板
│   ├── GOAL.md                    # 标准 GOAL.md 模板
│   └── rust-quality.md            # Rust 项目专用模板
├── examples/                       # 🆕 示例集合
│   ├── 01-rust-code-quality.md   # Rust 代码质量
│   ├── 02-test-synthesis.md      # 测试合成优化
│   ├── 03-cli-tool.md            # CLI 工具项目
│   └── 04-library.md             # 库项目
└── docs/
    └── goal-md/                   # 🆕 GOAL.md 文档
        ├── GOAL-md-融合方案.md    # 本文档
        ├── 快速开始.md           # 使用指南
        └── API-参考.md           # API 文档
```
