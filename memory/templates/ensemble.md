# Ensemble Template Knowledge

Ensemble 模板：组合多个 harness。通过加权投票或串联方式融合多个 harness 的结果。

## Purpose

当单一 harness 不可靠时，结合多个 harness 的判断提升整体质量。

## When to Use

- 单个 harness 准确率低于目标时
- 需要平衡多个评估维度时
- 有多个互补的 harness 可用时

## Success Patterns

- 子 harness 互补（不同偏见）
- 权重分配有依据（历史准确率或置信度）

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 组合逻辑冲突（权重互相抵消）
- 权重分配不当（过度依赖单一 harness）
- 引入过多噪声（子 harness 质量差异过大）

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z