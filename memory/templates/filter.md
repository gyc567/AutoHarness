# Filter Template Knowledge

Filter 模板：过滤/筛选合法动作。只通过符合安全或业务规则的候选动作。

## Purpose

在生成候选动作后、执行前，根据规则过滤掉不合规的选项。

## When to Use

- 有明确的安全或业务规则时
- 需要防止特定类型的动作被执行时
- 作为 Verifier 的前置层时

## Success Patterns

- 规则覆盖所有拒绝条件
- 快速短路（先检查最常见失败原因）

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 缺少边界检查
- 未处理 None/空状态
- 规则过于宽松（漏过危险动作）

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z