# Adaptive Template Knowledge

Adaptive 模板：自适应调整行为。根据历史执行结果动态调整策略参数。

## Purpose

当反馈数据可用时，自动调整阈值、权重或参数以优化性能。

## When to Use

- 有历史执行数据可供分析时
- 需要动态调整而非固定策略时
- 收敛速度慢于预期时

## Success Patterns

- 参数更新频率与数据量匹配（太少数据→噪声，太多→惯性）
- 衰减机制防止剧烈波动

## Failure Seeds

失败教训（用于避免重复犯错）。

### Common Errors

- 适应过度（过拟合近期数据）
- 历史数据偏差（冷启动问题）
- 遗忘机制缺失（无法遗忘错误模式）

## Stats

- Successes: 0, Failures: 0
- Last updated: 2026-08-06T00:00:00Z