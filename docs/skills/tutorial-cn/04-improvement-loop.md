# Improvement Loop Skill 使用指南

**执行一个完整的 GOAL.md 改进循环**

---

## 概述

`/improvement-loop` skill 执行完整的改进循环：
1. 测量当前分数
2. 分析弱点
3. 规划改进行动
4. 执行改动
5. 验证结果
6. 记录迭代

## 触发词

- "improve the project"
- "run one iteration"
- "make the score better"
- "run the improvement loop"

## 使用方法

### 在 Claude Code 中

```
/improvement-loop
```

### 独立使用

手动执行循环：
1. `./scripts/score.sh --json > /tmp/before.json`
2. 分析并执行改进
3. `./scripts/score.sh --json > /tmp/after.json`
4. 比较并提交

## 改进循环

```
┌─────────────────────────────────────────────────┐
│  改进循环 (Improvement Loop)                     │
├─────────────────────────────────────────────────┤
│                                                  │
│  1. 测量 ────> ./scripts/score.sh              │
│       │                                          │
│       v                                          │
│  2. 分析 ────> 找到最弱的组件                   │
│       │                                          │
│       v                                          │
│  3. 规划 ───────> 选择影响最大的行动            │
│       │                                          │
│       v                                          │
│  4. 执行 ────> 进行改动                          │
│       │                                          │
│       v                                          │
│  5. 验证 ────> 再次运行评分                     │
│       │                                          │
│       v                                          │
│  6. 决策 ─────> 改进了？→ 提交                  │
│       │           退步了？→ 回滚                 │
│       v                                          │
│  7. 记录 ──────> 追加到 iterations.jsonl         │
│                                                  │
└─────────────────────────────────────────────────┘
```

## 示例输出

```
=== 改进循环 第 1 次迭代 ===

改进前: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
└── maintenance : 10 / 10 ✓

行动: 添加文档 (+5)
执行中: 创建 docs/api.md
改进后: 90 / 100 ✓

改动: 保留 (+5)
记录到: iterations.jsonl
```

## 核心原则

### 1. 分数不能下降

如果改动导致分数下降，**立即回滚**。

```
改进前: 85/100 → 改进后: 80/100 → 回滚！
```

### 2. 每次提交一个改动

原子提交便于回滚。

```bash
git add -p  # 只暂存相关改动
git commit -m "docs: 添加 API 文档 (+5)"
```

### 3. 提交前务必验证

```bash
./scripts/score.sh  # 必须显示改进
git commit
```

### 4. 记录每次迭代

追加到 `iterations.jsonl`：

```json
{"iteration":1,"timestamp":"2024-01-01T00:00:00Z","component":"docs","before":10,"after":15,"action":"add api.md","result":"kept"}
```

## 行动目录

按影响排序的典型行动：

| 优先级 | 行动 | 影响 | 时机 |
|--------|------|------|------|
| 1 | `cargo fmt` | +20 | 总是首先执行 |
| 2 | `cargo clippy --fix` | +20 | 格式化后 |
| 3 | 添加测试 | +15 | 覆盖率不足时 |
| 4 | 改进文档 | +10 | 每周 |
| 5 | CI/CD 设置 | +10 | 一次性 |

## 迭代日志格式

```jsonl
{"iteration":N,"timestamp":"ISO8601","component":"name","before":N,"after":N,"action":"description","result":"kept|reverted"}
```

## 有效改进的技巧

1. **从格式化开始** - 总是先修复格式
2. **关注薄弱项** - 瞄准分数最低的区域
3. **小改动** - 一次只做一件事
4. **频繁验证** - 每次改动后运行评分

## 下一步

- [示例](05-examples.md) - 实际示例
