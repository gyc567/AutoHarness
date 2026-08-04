# Skills 概述

**AutoHarness Skills - 自主改进的 Agent 能力**

---

## 什么是 Skills？

Skills 是基于 [AI Builder Club](https://github.com/AI-Builder-Club/skills) 模式的可复用 AI Agent 能力模块。它们提供结构化、可重复的工作流程，使 Agent 能够可靠地执行任务。

## 可用 Skills

| Skill | 分类 | 用途 |
|-------|------|------|
| `/setup-goal` | GOAL.md | 初始化 GOAL.md 系统 |
| `/score-check` | GOAL.md | 检查项目分数 |
| `/improvement-loop` | GOAL.md | 执行改进循环 |
| `/new-goal-loop` | 循环 | 创建新工作流 |
| `/context-audit` | 工具 | 审计 Agent 上下文 |
| `/flow-diagram` | 工具 | 生成流程图 |

## 快速导航

- [01-installation.md](01-installation.md) - 安装和配置
- [02-setup-goal.md](02-setup-goal.md) - 如何使用 setup-goal
- [03-score-check.md](03-score-check.md) - 如何使用 score-check
- [04-improvement-loop.md](04-improvement-loop.md) - 如何使用 improvement-loop
- [05-examples.md](05-examples.md) - 实际示例

## 核心概念

### GOAL.md 系统

GOAL.md 系统实现**自主改进**：
1. **Fitness Function** - 输出分数 (0-100) 的脚本
2. **Action Catalog** - 可以改进分数的行动
3. **Improvement Loop** - 测量 → 行动 → 验证 → 记录

### Skill 结构

```
skills/
├── goal-md/
│   └── setup-goal/
│       ├── SKILL.md          # Skill 定义
│       ├── assets/           # 模板
│       └── references/       # 文档
```

### 触发词

每个 Skill 对应特定的短语：
- "set up GOAL.md" → `/setup-goal`
- "check score" → `/score-check`
- "improve the project" → `/improvement-loop`

## 下一步

- [安装指南](01-installation.md) - 开始使用
