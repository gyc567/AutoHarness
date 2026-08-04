# 性能调优规范

**版本**: 0.1.0
**更新日期**: 2026-08-04

---

## 目标

定义性能调优的标准流程，确保优化有据可依、有量可测。

---

## 反馈机制

### 1. Profiling 工具

```bash
# Rust: 使用 flamegraph
cargo flamegraph --bin autoharness -- synthesize

# 查看生成的性能火焰图
open flamegraph.svg
```

### 2. 基准测试对比

```bash
# 保存基准
cargo bench -- --save-baseline before

# 优化后运行
cargo bench -- --baseline before
```

### 3. 内存分析

```bash
# 使用 valgrind (Linux)
valgrind --tool=massif cargo run -- synthesize

# 或使用 dhat
RUSTFLAGS="-F debug" cargo build
```

---

## 调优流程

### Step 1: 测量基线

```bash
# 运行基准测试获取当前性能
cargo bench > baseline.txt

# 记录关键指标
cat baseline.txt | grep -E "synthesis|parse|compile"
```

### Step 2: 识别瓶颈

- 使用 profiling 工具找到热点函数
- 分析内存分配模式
- 检查 I/O 操作频率

### Step 3: 实施优化

常见优化策略:
1. **缓存**: 避免重复计算
2. **预分配**: 减少动态内存分配
3. **并行化**: 使用 Rayon 并行处理
4. **惰性求值**: 延迟非必要计算

### Step 4: 验证改善

```bash
# 对比基准
cargo bench -- --baseline before

# 确保无功能退化
cargo test
```

---

## 性能目标 (北极星指标)

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 平均合成时间 | < 500ms | `cargo bench` |
| CLI 响应时间 | < 100ms | 手动测量 |
| 内存峰值 | < 100MB | 内存分析工具 |

---

## 验收检查清单

```
性能调优验收清单
================

基线测量:
[ ] 有优化前的基准数据
[ ] 关键函数有 profiling 数据

优化实施:
[ ] 优化点明确 (非猜测)
[ ] 代码改动最小化
[ ] 无引入新依赖

结果验证:
[ ] 性能有明确改善 (量化)
[ ] 功能无退化
[ ] 测试全部通过
[ ] 性能提升 ≥ 10% (或达到目标)
```

---

## 相关文档

- [北极星指标](../architecture/north-star-metrics.md)
- [回测评估规范](./backtesting.md)
- [前端优化规范](./frontend-optimization.md)
