# Skills 整合方案 - GOAL.md Skills

**GOAL.md 自主改进系统的 Skill 设计**

---

## 一、Skills 列表

| Skill | 用途 | user_invocable |
|-------|------|----------------|
| setup-goal | 初始化 GOAL.md 系统 | true |
| score-check | 快速检查当前分数 | true |
| improvement-loop | 执行改进循环 | true |

---

## 二、setup-goal Skill

### 2.1 用途

为任意项目初始化 GOAL.md 系统。

### 2.2 触发词

- "set up GOAL.md"
- "initialize goal tracking"
- "add autonomous improvement"

### 2.3 SKILL.md 模板

```yaml
---
name: setup-goal
description: >
  Initialize GOAL.md system for any project — creates the fitness function script,
  goal definition, and iteration log. Use when the user says "set up GOAL.md",
  "initialize goal tracking", or "add autonomous improvement to this repo".
user_invocable: true
---

# Set up GOAL.md

GOAL.md 是一个让 AI Agent 能够自主改进项目的文件格式。核心思想：
**给 Agent 一个数字（分数），让它自己去让这个数字变大。**

## 什么是 GOAL.md？

1. **Fitness Function** - 一个脚本，输出一个数字来衡量"项目有多好"
2. **Action Catalog** - 列出所有可能的改进行动及其预期影响
3. **Improvement Loop** - 测量→诊断→行动→验证→记录 的循环
4. **Operating Mode** - Converge / Continuous / Supervised
5. **Constraints** - Agent 必须遵守的约束

## 使用场景

- "set up GOAL.md"
- "initialize goal tracking"
- "add autonomous improvement to this repo"

## 步骤

### 1. 评估项目

检查项目当前状态：
- 项目类型 (Rust/JS/Python/其他)
- 现有测试框架
- 现有 lint 工具
- 文档结构

### 2. 创建评分脚本

生成 `scripts/score.sh`:

```bash
#!/bin/bash
# Fitness Function

FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0
DOC_SCORE=0; MAINTENANCE_SCORE=0

# 根据项目类型选择工具
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# ... 其他组件

TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE))
echo "Score: $TOTAL / 100"
```

### 3. 创建 GOAL.md

复制模板并填充项目信息。

### 4. 初始化迭代日志

创建空的 `iterations.jsonl`。

## 输出

- `scripts/score.sh` - 评分脚本
- `GOAL.md` - 目标定义
- `iterations.jsonl` - 迭代日志

## 原则

- **增量式**: 先设置基本评分，逐步完善
- **可验证**: 每个行动都必须能验证效果
- **可追溯**: 所有改动都记录在 iterations.jsonl
```

---

## 三、score-check Skill

### 3.1 用途

快速检查当前项目分数。

### 3.2 触发词

- "check score"
- "run score"
- "what's our current score"

### 3.3 SKILL.md 模板

```yaml
---
name: score-check
description: >
  Quick check of the current project score using the GOAL.md fitness function.
  Use when the user says "check score", "run score", or "what's our current score".
user_invocable: true
---

# Score Check

运行评分脚本，检查当前项目状态。

## 步骤

1. 运行 `./scripts/score.sh`
2. 解析并显示分数
3. 识别最弱的组件
4. 建议下一个改进行动

## 输出示例

```
Score: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
├── maintenance : 20 / 20 ✓
└── safety      :  0 / 10 ✗

Next action: Add safety documentation (+10)
```

## 原则

- 快速响应（< 30 秒）
- 提供可操作的建议
- 识别分数瓶颈
```

---

## 四、improvement-loop Skill

### 4.1 用途

执行一个完整的 GOAL.md 改进循环。

### 4.2 触发词

- "improve the project"
- "run one iteration"
- "make the score better"

### 4.3 SKILL.md 模板

```yaml
---
name: improvement-loop
description: >
  Execute one complete GOAL.md improvement loop: measure, diagnose, plan, execute, verify, and log.
  Use when the user says "improve the project", "run one iteration", or "make the score better".
user_invocable: true
---

# Improvement Loop

执行一个完整的 GOAL.md 改进循环。

## 改进循环

```
repeat:
  1. ./scripts/score.sh --json > /tmp/before.json
  2. 分析分数 - 找到最弱的组件
  3. 从 Action Catalog 选择最高影响的行动
  4. 执行改动
  5. 运行针对性验证
  6. ./scripts/score.sh --json > /tmp/after.json
  7. 比较: 改进了就提交，退步了就回滚
  8. 追加到 iterations.jsonl
  9. 继续
```

## 步骤

### 1. 测量

```bash
./scripts/score.sh --json > /tmp/before.json
```

### 2. 分析

读取分数，识别最弱的组件。

### 3. 规划

从 Action Catalog 选择最高影响的行动。

### 4. 执行

执行选定的行动。

### 5. 验证

```bash
./scripts/score.sh --json > /tmp/after.json
```

### 6. 决策

如果改进了 → 提交
如果退步了 → 回滚

### 7. 记录

追加到 iterations.jsonl:

```json
{"iteration":1,"timestamp":"...","component":"clippy","before":15,"after":20,"action":"fix warnings","result":"kept"}
```

## 原则

- **分数不能下降**: 每次改动后分数不能比之前低
- **一个提交一个改动**: 原子提交便于回滚
- **先格式后 Lint**: 总是先 cargo fmt，再 cargo clippy
```

---

## 五、文件结构

```
skills/goal-md/
├── setup-goal/
│   ├── SKILL.md
│   ├── assets/
│   │   ├── goal.template.md
│   │   └── score.template.sh
│   └── references/
│       └── scoring-guide.md
├── score-check/
│   └── SKILL.md
└── improvement-loop/
    ├── SKILL.md
    └── references/
        └── loop-guide.md
```

---

## 六、下一步

- [04-codebase-harness.md](04-codebase-harness.md) - Codebase Harness 设计
