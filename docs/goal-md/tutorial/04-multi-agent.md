# 多 Agent 协作指南

**让多个 AI Agent 协同工作**

---

## 概述

GOAL.md 模式天然支持多 Agent 协作：
- 不同 Agent 可以负责不同组件
- 共享同一个评分脚本
- 通过迭代日志协调

---

## 架构模式

### 1. 单一 Agent 模式

最简单的模式，一个 Agent 完成所有工作：

```
┌─────────────────────────────┐
│         Agent               │
│                             │
│  1. 读取 GOAL.md            │
│  2. 运行评分脚本            │
│  3. 选择行动                │
│  4. 执行并验证              │
│  5. 记录迭代                │
└─────────────────────────────┘
```

### 2. 协调者模式

一个协调者 Agent 分配任务给多个执行者：

```
┌─────────────────────────────────────────────┐
│              Coordinator Agent               │
│                                             │
│  1. 分析当前分数                            │
│  2. 将任务分配给执行者                      │
│  3. 收集结果                                │
│  4. 验证整体改进                            │
└─────────────────────────────────────────────┘
           │              │
           ▼              ▼
┌─────────────────┐  ┌─────────────────┐
│  Worker Agent   │  │  Worker Agent   │
│  (Quality)      │  │  (Performance)  │
│                 │  │                 │
│  - format       │  │  - bench        │
│  - clippy      │  │  - optimize     │
│  - tests       │  │  - profile      │
└─────────────────┘  └─────────────────┘
```

### 3. 管道模式

多个 Agent 按顺序处理：

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ Format  │───▶│ Clippy  │───▶│ Tests   │───▶│ Docs    │
│ Agent   │    │ Agent   │    │ Agent   │    │ Agent   │
└─────────┘    └─────────┘    └─────────┘    └─────────┘
     │              │              │              │
     └──────────────┴──────────────┴──────────────┘
                          │
                          ▼
                   ┌─────────────┐
                   │   Score     │
                   │   Check     │
                   └─────────────┘
```

---

## 协作规则

### 规则 1: 共享评分脚本

所有 Agent 使用同一个评分脚本：

```bash
# 评分脚本位置
scripts/score.sh

# 总是运行完整评分
./scripts/score.sh --json
```

### 规则 2: 原子提交

每次改进一个提交：

```bash
# ✅ 正确：每个改动一个提交
git commit -m "[S:85→90] clippy: fix unused imports"
git commit -m "[S:90→95] tests: add core module tests"

# ❌ 错误：多个改动混在一起
git commit -m "improve code quality"
```

### 规则 3: 记录迭代

每个 Agent 都必须记录：

```bash
echo '{"iteration":1,"agent":"clippy-worker","component":"clippy","before":85,"after":90}' >> iterations.jsonl
```

### 规则 4: 检查分数上限

避免 Agent 之间竞争：

```bash
# 在执行前检查是否已达到目标
if [[ $CURRENT_SCORE -eq 100 ]]; then
    echo "Goal achieved! Stopping."
    exit 0
fi
```

---

## 任务分配策略

### 按组件分配

每个 Agent 负责一个组件：

| Agent | Components |
|-------|------------|
| quality-agent | format, clippy, tests |
| perf-agent | bench, optimize |
| docs-agent | docs, examples |

### 按阶段分配

不同阶段分配不同 Agent：

| 阶段 | Agent | 任务 |
|------|-------|------|
| 1 | bootstrap-agent | 建立基线，创建必要文件 |
| 2 | quality-agent | 修复格式和 lint |
| 3 | test-agent | 添加测试 |
| 4 | perf-agent | 性能优化 |

### 按优先级分配

根据当前分数动态分配：

```python
def assign_task(components):
    """根据短板分配任务"""
    weakest = min(components, key=lambda c: c.score / c.max)
    
    if weakest.name == "format":
        return Task("format-agent", "cargo fmt")
    elif weakest.name == "clippy":
        return Task("lint-agent", "cargo clippy --fix")
    elif weakest.name == "tests":
        return Task("test-agent", "add tests")
    else:
        return Task("generic-agent", "general improvement")
```

---

## 冲突处理

### 冲突类型

| 类型 | 示例 | 解决方案 |
|------|------|----------|
| 分数竞争 | 两个 Agent 同时改同一文件 | 锁机制或顺序执行 |
| 目标冲突 | A 要改 format，B 要改内容 | 协调者仲裁 |
| 回归 | 改动导致分数下降 | 自动回滚 |

### 解决方案

#### 1. 锁机制

```bash
# 使用文件锁防止并发
LOCK_FILE=".goal-lock"
if [[ -f "$LOCK_FILE" ]]; then
    echo "Another agent is running. Exiting."
    exit 1
fi

trap "rm -f $LOCK_FILE" EXIT
touch "$LOCK_FILE"

# 执行任务...
```

#### 2. 顺序执行

```yaml
# .goal-workflow.yml
agents:
  - name: format-agent
    waits_for: []
  - name: lint-agent
    waits_for: [format-agent]
  - name: test-agent
    waits_for: [lint-agent]
```

#### 3. 乐观锁

```bash
# 在执行前记录分数，执行后验证
BEFORE=$(./scripts/score.sh --json | jq '.total')
git stash
make_change
AFTER=$(./scripts/score.sh --json | jq '.total')

if [[ "$AFTER" -le "$BEFORE" ]]; then
    git stash pop
    echo "No improvement, reverting"
fi
```

---

## 共享状态

### 共享文件

```
.
├── GOAL.md              # 共享目标
├── iterations.jsonl     # 共享日志
└── scripts/
    └── score.sh         # 共享评分脚本
```

### 共享变量

```bash
# shared.env
CURRENT_SCORE=85
TARGET_SCORE=100
ITERATION_COUNT=10
LAST_ITERATION_TIME="2026-08-04T12:00:00Z"
```

---

## 监控与报告

### 进度报告

```bash
#!/bin/bash
# scripts/report.sh - 生成改进报告

echo "=== GOAL.md Improvement Report ==="
echo ""

# 读取迭代日志
TOTAL=$(wc -l < iterations.jsonl)
IMPROVEMENTS=$(grep -c '"result":"kept"' iterations.jsonl)
REGRESSIONS=$(grep -c '"result":"reverted"' iterations.jsonl)

echo "Total iterations: $TOTAL"
echo "Improvements: $IMPROVEMENTS"
echo "Regressions: $REGRESSIONS"
echo ""

# 当前分数
./scripts/score.sh --json | jq '{total, max, components}'
```

### 团队通知

```bash
# 在每次迭代后通知
if [[ -n "${SLACK_WEBHOOK:-}" ]]; then
    curl -X POST "$SLACK_WEBHOOK" \
        -d "{\"text\":\"Iteration $N: $BEFORE → $AFTER (+$DIFF)\"}"
fi
```

---

## 最佳实践

### Do

- ✅ 使用同一个评分脚本
- ✅ 原子提交便于回滚
- ✅ 记录每次迭代
- ✅ 定期检查目标进度
- ✅ 协调者统一分配任务

### Don't

- ❌ 不要让多个 Agent 同时修改同一文件
- ❌ 不要跳过分数验证
- ❌ 不要删除迭代日志
- ❌ 不要修改已锁定的评分脚本

---

## 下一步

- [迭代日志分析](05-iteration-log.md)
- [常见问题解决](06-troubleshooting.md)
