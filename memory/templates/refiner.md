# Refiner Template Knowledge

Refiner 模板：自我改进/迭代优化。对现有代码进行渐进式改进直到满足条件。

## Purpose

在保持语义等价的前提下，通过反复修改提升代码质量或性能。

## When to Use

- 有明确优化目标（分数、速度、长度）时
- 当前代码接近但未达到目标时
- 需要探索多个变体时

## Success Patterns

- 每次修改有明确的改进方向
- 有收敛判断条件（最大迭代次数、最小改进量）

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 迭代不收敛（方向错误或步长过大）
- 修复引入新 bug（回归）
- 陷入局部最优

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z