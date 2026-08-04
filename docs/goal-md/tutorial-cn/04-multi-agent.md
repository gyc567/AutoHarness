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
│  1. 读取 GOAL.md           │
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
│  1. 分析当前分数                           │
│  2. 将任务分配给执行者                     │
│  3. 收集结果                               │
│  4. 验证整体改进                           │
└─────────────────────────────────────────────┘
           │              │
           ▼              ▼
┌─────────────────┐  ┌─────────────────┐
│  Worker Agent   │  │  Worker Agent   │
│  (Quality)      │  │  (Performance)  │
│                 │  │                 │
│  - format       │  │  - bench        │
│  - clippy      │  │  - optimize     │
│  - tests       │  │                 │
└─────────────────┘  └─────────────────┘
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

# ❌ 错误：多个改动混在一起
git commit -m "improve code quality"
```

### 规则 3: 记录迭代

每个 Agent 都必须记录：

```bash
echo '{"iteration":1,"agent":"clippy-worker","component":"clippy","before":85,"after":90}' >> iterations.jsonl
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
```

#### 2. 顺序执行

```yaml
# .goal-workflow.yml
agents:
  - name: format-agent
    waits_for: []
  - name: lint-agent
    waits_for: [format-agent]
```

---

## 最佳实践

### Do

- ✅ 使用同一个评分脚本
- ✅ 原子提交便于回滚
- ✅ 记录每次迭代
- ✅ 协调者统一分配任务

### Don't

- ❌ 不要让多个 Agent 同时修改同一文件
- ❌ 不要跳过分数验证
- ❌ 不要删除迭代日志

---

## 下一步

- [进阶模式](05-advanced-patterns.md)
- [常见问题解决](06-troubleshooting.md)
