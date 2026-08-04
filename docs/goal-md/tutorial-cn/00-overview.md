# GOAL.md 概述

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

## 适用场景

| 场景 | 推荐模式 | 示例 |
|------|----------|------|
| 代码质量改进 | Converge | Clippy 警告清理 |
| 性能优化 | Continuous | Benchmark 持续优化 |
| 安全审计 | Supervised | 敏感代码审查 |
| 文档完善 | Converge | README 编写 |

---

## 快速导航

### 新手入门

1. [5 分钟快速开始](01-quick-start.md) - 最简单的上手方式
2. [核心概念详解](02a-fitness-function.md) - 理解 Fitness Function

### 实际应用

3. [创建你的第一个 GOAL.md](03-create-goal.md) - 完整示例
4. [多 Agent 协作](04-multi-agent.md) - 团队协作指南

### 进阶主题

5. [进阶模式](05-advanced-patterns.md) - 高级技巧
6. [常见问题解决](06-troubleshooting.md) - 故障排除

---

## 下一步

想要立即开始？前往 [5 分钟快速开始](01-quick-start.md)
