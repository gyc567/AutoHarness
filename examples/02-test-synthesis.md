# 测试合成项目 GOAL.md 示例

**适用场景**: AutoHarness 这样的测试合成工具
**模式**: Converge
**基线**: ~65/100

---

## Fitness Function

```bash
./scripts/score.sh --all    # 完整评分
./scripts/score.sh          # 代码质量
./scripts/bench.sh          # 性能基准
./scripts/coverage.sh       # 覆盖率
```

### Metric Definition

```
total = quality_score + performance_score + coverage_score

quality_score = format(20) + clippy(20) + docs(15) + safety(10)
performance_score = synthesis_time(25) + cli_response(25)
coverage_score = line_coverage(50)
```

---

## 北极星指标对齐

| 指标 | 目标 | 对应 GOAL.md 组件 |
|------|------|------------------|
| 代码合成成功率 ≥ 85% | 测试通过率 | 100% |
| 平均合成时间 < 500ms | synthesis_time | < 500ms |
| 测试覆盖率 ≥ 80% | line_coverage | ≥ 80% |
| CLI 响应时间 < 100ms | cli_response | < 100ms |

---

## Bootstrap

```bash
cargo fetch
./scripts/score.sh --all > baseline.txt
# 记录:
#   Quality:   NN/100
#   Perf:      NN/100
#   Coverage:  NN/100
```

---

## Action Catalog

### format (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Run cargo fmt | +20 | `cargo fmt` |

### clippy (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| Fix warnings | +15 | `cargo clippy --fix` |
| Add clippy config | +5 | Add `[lints.clippy]` to Cargo.toml |

### synthesis_time (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| Profile synthesis hot path | +10 | `cargo flamegraph -- synthesis` |
| Cache type information | +10 | Add LRU cache for AST parsing |
| Parallelize independent tasks | +5 | Use Rayon for parallel search |

### line_coverage (target: 50/50)

| Action | Impact | How |
|--------|--------|-----|
| Add unit tests | +10-20 | Test uncovered modules |
| Add integration tests | +10-15 | Test synthesis workflow end-to-end |
| Add benchmarks | +5 | `cargo bench` for regression detection |

---

## Performance Baseline

```bash
# 建立性能基线
./scripts/bench.sh --json > benchmarks/baseline.json

# 对比变化
./scripts/bench.sh --json | jq -s '.[0] as $old | .[1] as $new | 
  "Synthesis time: \($old.synthesis_time.value)ms → \($new.synthesis_time.value)ms"' 
```

---

## Constraints

1. **北极星指标不可妥协** — 即使分数满分，北极星指标不达标也不能停止
2. **性能不退化** — benchmark 结果不能比 baseline 差 10%+
3. **测试完整性** — 新功能必须有对应测试
4. **不要破坏合成能力** — 核心 synthesis 功能必须保持正常
