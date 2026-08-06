# Policy Template Knowledge

Policy 模板：纯代码策略，无 LLM。基于确定性规则选择动作。

## Purpose

用硬编码逻辑替代 LLM 调用，实现可预测、可复现的动作选择。

## When to Use

- 动作空间小且规则清晰时
- 需要确定性行为而非随机采样时
- 作为基线或 fallback 时

## Success Patterns

- 规则覆盖所有已知情况
- 有明确的优先级和冲突解决策略

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 状态机不完整（遗漏状态）
- 硬编码值过多（不灵活）
- 新情况无对应规则

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z