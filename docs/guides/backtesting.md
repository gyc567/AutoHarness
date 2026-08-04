# 回测评估规范

**版本**: 0.1.0
**更新日期**: 2026-08-04

---

## 目标

定义回测评估的标准流程，确保代码改动的质量可量化、可验证。

---

## 反馈机制

### 1. 单元测试回测

```bash
# 运行完整测试套件
cargo test

# 运行带覆盖率的测试
cargo tarpaulin --output --tests

# 检查测试通过率
cargo test -- --nocapture | grep "test result"
```

### 2. 集成测试回测

```bash
# 运行集成测试
cargo test --test integration

# 运行特定场景测试
cargo test --test integration -- synthesis_workflow
```

### 3. 性能回测

```bash
# 运行基准测试
cargo bench

# 对比性能变化
git diff HEAD~10 -- benches/
```

---

## 回测场景分类

| 场景 | 测试文件 | 运行频率 |
|------|----------|----------|
| 核心功能 | `tests/unit/*.rs` | 每次 PR |
| 合成算法 | `tests/synthesis/*.rs` | 每次 PR |
| 性能基准 | `benches/*.rs` | 每日 |
| 集成测试 | `tests/integration/*.rs` | 每次合并 |

---

## 验收标准

### 功能回测

- [ ] 所有单元测试通过
- [ ] 所有集成测试通过
- [ ] 新功能有对应的测试用例
- [ ] 测试覆盖率不降低

### 性能回测

- [ ] 基准测试无明显退化 (≤ 5%)
- [ ] 内存使用无泄漏
- [ ] CLI 响应时间符合目标

---

## 回测报告模板

```markdown
## 回测报告

**日期**: YYYY-MM-DD
**提交**: abc1234
**审查者**: AI Agent

### 测试结果

| 测试类型 | 通过率 | 覆盖率 | 状态 |
|----------|--------|--------|------|
| 单元测试 | 100% | 85% | ✅ |
| 集成测试 | 100% | - | ✅ |
| 性能测试 | - | - | ✅ |

### 性能对比

| 指标 | 之前 | 之后 | 变化 |
|------|------|------|------|
| 合成时间 | 450ms | 460ms | +2.2% |

### 结论

[通过/不通过] - 原因说明
```

---

## 相关文档

- [北极星指标](../architecture/north-star-metrics.md)
- [性能调优规范](./performance-tuning.md)
- [代码审查规范](./code-review.md)
