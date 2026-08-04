# Setup Goal Skill 使用指南

**为任意项目初始化 GOAL.md 系统**

---

## 概述

`/setup-goal` skill 创建 GOAL.md 基础设施，实现自主改进。它会生成：
- `scripts/score.sh` - 评分脚本
- `GOAL.md` - 目标定义文件
- `iterations.jsonl` - 迭代日志

## 触发词

- "set up GOAL.md"
- "initialize goal tracking"
- "add autonomous improvement"
- "make this project agent-improving"

## 使用方法

### 在 Claude Code 中

```
/setup-goal
```

### 独立使用

```bash
# 读取并执行 skill
cat skills/goal-md/setup-goal/SKILL.md
```

## 它做了什么

### 1. 评估项目

检测以下内容：
- 项目类型 (Rust/Node.js/Python 等)
- 现有测试框架
- 现有 lint 工具
- 文档结构

### 2. 创建评分脚本

生成 `scripts/score.sh`，包含以下评分项：

| 组件 | 分值 | 检查项 |
|------|------|--------|
| Format | 20 | `cargo fmt`, `prettier`, `black` |
| Lint | 20 | `cargo clippy`, `eslint` |
| Tests | 25 | `cargo test`, `npm test` |
| Docs | 15 | README, docs/ 目录 |
| Maintenance | 20 | .gitignore, CI/CD |

### 3. 创建 GOAL.md

```markdown
# Goal: [项目名] - [一句话目标]

## Fitness Function

./scripts/score.sh

## Operating Mode

- [x] **Converge** — 达到目标时停止

## Action Catalog

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 格式化代码 | +20 | `cargo fmt` |
| 修复 clippy | +20 | `cargo clippy --fix` |

## Iteration Log

File: `iterations.jsonl`
```

### 4. 初始化日志

创建空的 `iterations.jsonl`。

## 输出文件

```
project/
├── scripts/
│   └── score.sh        # 自动生成
├── GOAL.md             # 自动生成
└── iterations.jsonl   # 自动生成（空）
```

## 自定义

### 添加自定义评分项

编辑 `scripts/score.sh`:

```bash
# 添加自定义检查
CUSTOM_SCORE=0
if your-custom-check; then
    CUSTOM_SCORE=10
fi
TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE + CUSTOM_SCORE))
```

### 添加自定义行动

编辑 `GOAL.md` 的 Action Catalog:

```markdown
| 我的自定义行动 | +5 | 如何执行 |
```

## 示例

### Rust 项目

```bash
$ /setup-goal
检测到: Rust 项目 (Cargo.toml)
创建中: scripts/score.sh
创建中: GOAL.md
创建中: iterations.jsonl

完成！运行 ./scripts/score.sh 查看分数。
```

### Node.js 项目

```bash
$ /setup-goal
检测到: Node.js 项目 (package.json)
创建中: scripts/score.sh (prettier + eslint)
创建中: GOAL.md
创建中: iterations.jsonl

完成！运行 ./scripts/score.sh 查看分数。
```

## 下一步

- [Score Check](03-score-check.md) - 检查初始分数
- [Run Improvement Loop](04-improvement-loop.md) - 开始改进
