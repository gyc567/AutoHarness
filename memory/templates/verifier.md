# Verifier Template Knowledge

Verifier 模板：验证/检查动作合法性。在执行前确认动作满足所有约束。

## Purpose

作为安全网，在动作执行前验证其正确性和安全性。

## When to Use

- 有明确的不变量需要维护时
- 动作可能破坏系统约束时
- 作为最后一道防线时

## Success Patterns

- 不变量定义完整
- 检查快速（不阻塞主流程）

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 正则表达式不完整（误判或漏判）
- 遗漏边界情况
- 检查过于严格（阻止合法动作）

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z