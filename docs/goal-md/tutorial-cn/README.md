# GOAL.md 教程索引

**让 AI Agent 自主改进代码，只需给它一个数字**

---

## 📚 教程列表

### 新手入门

| 文档 | 内容 | 时间 |
|------|------|------|
| [00-overview.md](00-overview.md) | 概念介绍与快速导航 | 5 分钟 |
| [01-quick-start.md](01-quick-start.md) | 5 分钟上手 | 5 分钟 |

### 核心概念

| 文档 | 内容 |
|------|------|
| [02a-fitness-function.md](02a-fitness-function.md) | Fitness Function 详解 |
| [02b-action-catalog.md](02b-action-catalog.md) | Action Catalog 详解 |

### 实践指南

| 文档 | 内容 |
|------|------|
| [03-create-goal.md](03-create-goal.md) | 创建你的第一个 GOAL.md |
| [04-multi-agent.md](04-multi-agent.md) | 多 Agent 协作 |

### 进阶主题

| 文档 | 内容 |
|------|------|
| [05-advanced-patterns.md](05-advanced-patterns.md) | 进阶模式 |
| [06-troubleshooting.md](06-troubleshooting.md) | 常见问题解决 |

---

## 🎯 学习路径

### 路径 1: 快速上手（30 分钟）

```
00-overview.md → 01-quick-start.md → 03-create-goal.md
```

### 路径 2: 深入理解（1 小时）

```
00-overview.md → 01-quick-start.md → 02a-fitness-function.md → 
02b-action-catalog.md → 03-create-goal.md → 04-multi-agent.md
```

---

## 📋 快速参考

### 基本命令

```bash
# 运行评分
./scripts/score.sh

# JSON 输出
./scripts/score.sh --json

# 记录迭代
echo '{"iteration":1,"before":50,"after":60}' >> iterations.jsonl
```

### GOAL.md 结构

```markdown
# Goal: [项目名]

## Fitness Function
./scripts/score.sh

## Operating Mode
- [x] Converge

## Action Catalog
| Action | Impact | How |

## Iteration Log
iterations.jsonl
```

---

## 🔗 相关资源

- [English Tutorial](../tutorial/) - 英文教程
- [template/GOAL.md](../../template/GOAL.md) - 完整模板
- [examples/](../../examples/) - 示例集合
- [CLAUDE.md](../../CLAUDE.md) - Agent 指南
