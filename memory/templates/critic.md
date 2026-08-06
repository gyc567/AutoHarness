# Critic Template Knowledge

Critic 模板：评估/打分动作质量。为每个候选动作提供质量分数。

## Purpose

独立评估候选动作的好坏，为 Selector 提供排序依据。

## When to Use

- 有多个候选动作需要排序时
- 需要量化动作质量时
- 作为 Selector 或 Ensemble 的子组件时

## Success Patterns

- 评分标准与目标一致
- 分数有合理的归一化范围

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 评分逻辑不一致（相同输入不同分数）
- 缺少归一化（分数范围不固定）
- 过度依赖单一维度

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z