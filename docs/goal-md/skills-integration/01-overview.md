# Skills 整合方案 - 概述

**基于 AI Builder Club Skills 的 AutoHarness Agent 能力扩展**

---

## 一、项目目标

将 AI Builder Club 的 skills 模式整合到 AutoHarness 项目中，使 AutoHarness 成为一个完整的 **Loop Engineer 平台**，能够：

1. **GOAL.md 自主改进** - 基于评分脚本的持续优化
2. **代码库 Harness** - 让任何仓库对 AI Agent 可用
3. **验证-部署工作流** - 验证后再部署
4. **多 Agent 协作** - 团队 Agent 委托与协调

---

## 二、核心概念

### 2.1 什么是 Skill？

**Skill** 是一个可复用的 Agent 能力单元，包含：
- `SKILL.md` - 定义技能名称、用途、使用方法
- `assets/` - 模板文件
- `references/` - 参考文档
- `scripts/` - 辅助脚本

### 2.2 Skill 结构示例

```yaml
---
name: skill-name
description: >
  简洁描述技能用途和使用场景
user_invocable: true  # 是否可被用户直接调用
---

# Skill Title

## 概述
简要说明技能做什么。

## 使用场景
什么情况下使用这个技能。

## 步骤
1. 步骤一
2. 步骤二

## 原则
- 原则一
```

---

## 三、整合架构

```
AutoHarness/
├── .claude-plugin/           # Claude Code 插件配置
│   ├── marketplace.json      # 市场配置
│   └── plugin.json          # 插件元数据
├── skills/                   # Agent Skills 目录
│   ├── goal-md/            # GOAL.md 相关 skills
│   ├── codebase-harness/   # 代码库 Harness
│   ├── loops/             # 循环技能
│   └── utilities/          # 工具技能
└── docs/goal-md/           # GOAL.md 文档
```

---

## 四、Skills 分类

### 4.1 GOAL.md 类别

| Skill | 用途 |
|-------|------|
| setup-goal | 初始化 GOAL.md 系统 |
| score-check | 快速检查当前分数 |
| improvement-loop | 执行改进循环 |

### 4.2 Codebase Harness 类别

| Skill | 用途 |
|-------|------|
| setup-harness | 主协调器 |
| dev-local | 本地开发环境 |
| verify | 验证技能 |

### 4.3 Loops 类别

| Skill | 用途 |
|-------|------|
| new-goal-loop | 创建新目标循环 |

### 4.4 Utilities 类别

| Skill | 用途 |
|-------|------|
| context-audit | 上下文审计 |
| flow-diagram | 流程图生成 |

---

## 五、实施阶段

| 阶段 | 内容 | 时间 |
|------|------|------|
| 1 | 基础架构、插件配置 | Week 1 |
| 2 | GOAL.md Skills | Week 2 |
| 3 | Codebase Harness | Week 3 |
| 4 | 高级功能 | Week 4 |
| 5 | 文档与测试 | Week 5 |

---

## 六、下一步

- [02-plugin-setup.md](02-plugin-setup.md) - 插件配置
- [03-goal-md-skills.md](03-goal-md-skills.md) - GOAL.md Skills 设计
- [04-codebase-harness.md](04-codebase-harness.md) - Codebase Harness 设计
- [05-implementation-plan.md](05-implementation-plan.md) - 实施计划
