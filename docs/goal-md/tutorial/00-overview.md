# GOAL.md 自主改进教程

**让 AI Agent 自主改进代码，只需给它一个数字**

---

## 什么是 GOAL.md？

GOAL.md 是一种让 AI Agent 能够自主改进项目的文件格式。它解决了一个核心问题：

> **"我想要这个项目变得更好，但我不确定该怎么做"**

传统的做法：
1. 人工分析代码
2. 列出待办事项
3. 逐个执行
4. 手动验证

GOAL.md 的做法：
1. 写一个评分脚本（输出一个数字）
2. 写一个 GOAL.md 文件
3. 让 Agent 自己想办法让分数变高
4. Agent 会记录每次改动和分数变化

---

## 核心概念

### 1. Fitness Function（适应度函数）

一个脚本，输出一个数字来衡量"项目有多好"：

```bash
./scripts/score.sh
# 输出: 85 / 100
```

### 2. Action Catalog（行动目录）

列出所有可能的改进行动及其预期影响：

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 运行 cargo fmt | +20 | `cargo fmt` |
| 修复 clippy 警告 | +10 | 逐一修复 |

### 3. Improvement Loop（改进循环）

```
1. 测量当前分数
2. 选择最高影响的行动
3. 执行改动
4. 验证分数提高
5. 记录到日志
6. 重复
```

### 4. Operating Mode（运行模式）

- **Converge**: 达到目标分数后停止
- **Continuous**: 持续运行直到中断
- **Supervised**: 在关键点暂停等待确认

---

## 快速导航

### 新手入门

1. [5 分钟快速开始](01-quick-start.md) - 最简单的上手方式
2. [基础概念详解](02-concepts.md) - 理解 Fitness Function、Action Catalog 等

### 实际应用

3. [创建你的第一个 GOAL.md](03-create-goal.md) - 完整示例
4. [评分脚本编写指南](04-scoring-guide.md) - 编写好的评分脚本

### 进阶主题

5. [迭代日志分析](05-iteration-log.md) - 分析改进历史
6. [常见问题解决](06-troubleshooting.md) - 故障排除

### 参考资料

- [template/GOAL.md](../../template/GOAL.md) - GOAL.md 模板
- [examples/](../../examples/) - 各种场景示例
- [CLAUDE.md](../../CLAUDE.md) - Agent 使用指南

---

## 适用场景

| 场景 | 推荐模式 | 示例 |
|------|----------|------|
| 代码质量改进 | Converge | Clippy 警告清理 |
| 性能优化 | Continuous | Benchmark 持续优化 |
| 安全审计 | Supervised | 敏感代码审查 |
| 文档完善 | Converge | README 编写 |

---

## 下一步

想要立即开始？前往 [5 分钟快速开始](01-quick-start.md)

---

**相关文档**
- [融合方案](../GOAL-md-融合方案.md) - 本项目的集成实现
- [API 参考](../API-参考.md) - 脚本 API 文档
