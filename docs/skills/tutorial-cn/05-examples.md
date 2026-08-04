# Skills 使用示例

**使用 AutoHarness Skills 的实际示例**

---

## 示例 1: 初始化并改进 Rust 项目

### 步骤 1: 设置 GOAL.md

```bash
$ /setup-goal
检测到: Rust 项目 (Cargo.toml)
创建中: scripts/score.sh
创建中: GOAL.md
创建中: iterations.jsonl

✓ GOAL.md 系统已初始化
运行 ./scripts/score.sh 查看分数。
```

### 步骤 2: 检查初始分数

```bash
$ /score-check
分数: 70 / 100
├── format      : 20 / 20 ✓
├── clippy      : 15 / 20 ◐
├── tests       : 25 / 25 ✓
├── docs        :  5 / 15 ✗
├── maintenance :  5 / 20 ◐
└── safety      :  0 / 10 ✗

最弱项: docs (5/15)
建议行动: 改进文档
```

### 步骤 3: 运行改进循环

```bash
$ /improvement-loop
=== 改进循环 第 1 次迭代 ===

改进前: 70 / 100
行动: 添加文档 (+10)
执行中: 创建 docs/api.md
改进后: 80 / 100 ✓

改动: 保留 (+10)
记录到: iterations.jsonl

$ /improvement-loop
=== 改进循环 第 2 次迭代 ===

改进前: 80 / 100
行动: 修复 clippy 警告 (+5)
执行中: cargo clippy --fix
改进后: 85 / 100 ✓

改动: 保留 (+5)
记录到: iterations.jsonl
```

---

## 示例 2: Node.js 项目

### 设置

```bash
$ /setup-goal
检测到: Node.js 项目 (package.json)
创建中: scripts/score.sh (prettier + eslint)
创建中: GOAL.md
创建中: iterations.jsonl
```

### 检查分数

```bash
$ /score-check
分数: 60 / 100
├── format      : 20 / 20 ✓
├── lint        : 10 / 20 ◐
├── tests       : 20 / 25 ◐
├── docs        :  5 / 15 ✗
└── maintenance :  5 / 20 ◐

最弱项: docs (5/15)
```

### 改进

```bash
$ /improvement-loop
=== 改进循环 第 1 次迭代 ===

改进前: 60 / 100
行动: 添加 ESLint 配置 (+10)
执行中: npx eslint --init
改进后: 70 / 100 ✓

改动: 保留 (+10)
```

---

## 示例 3: 创建新的目标循环

### 创建循环

```bash
$ /new-goal-loop
循环名称: code-quality
目标: 保持 100% 代码质量分数
周期: weekly

创建中: domains/code-quality/README.md
初始化: ✓ GOAL.md 系统已存在

=== 测试运行 ===
运行中: 每周质量检查
结果: 分数 85/100，无严重问题

时间线: ✓ 已更新
LOG.md: ✓ 已更新
```

### 循环结构

```markdown
# domains/code-quality/README.md

---
kind: domain
domain: code-quality
status: active
goal: 保持 100% 代码质量分数
cadence: weekly
---

## 当前重点

每周质量审计

## 待办事项

- [ ] 添加安全扫描
- [ ] 设置覆盖率报告

## 时间线

| 日期 | 事件 | 结果 |
|------|------|------|
| 2024-01-01 | 测试运行 | 分数 85/100 |
```

---

## 示例 4: 上下文审计

### 运行审计

```bash
$ /context-audit
审计中: CLAUDE.md
审计中: skills/**/*.md
审计中: README.md

=== 审计报告 ===

| 文件 | 大小 | 高 | 中 | 低 | 结论 |
|------|------|------|------|------|---------|
| CLAUDE.md | 150 | 1 | 2 | 3 | 保留 |
| README.md | 200 | 0 | 1 | 5 | 保留 |

### 主要发现

| 文件:行 | 问题 | 严重性 | 修复建议 |
|---------|------|--------|----------|
| CLAUDE.md:42 | 硬性规则 "NEVER use unwrap" | 高 | 改为判断性表述 |
| CLAUDE.md:50 | 重复的 lint 指令 | 中 | 删除重复 |

应用修复？[y/n]
```

---

## 示例 5: 流程图

### 创建图表

```bash
$ /flow-diagram
来源: docs/architecture.md

从架构创建 JSON 规范...
规范已创建: /tmp/architecture-spec.json

渲染图表...
python3 scripts/render_diagram.py --spec /tmp/architecture-spec.json --outdir ./docs --basename architecture

✓ 已渲染: docs/architecture.png
✓ 已渲染: docs/architecture.gif
```

### 输出

```
docs/
├── architecture.md
├── architecture.png    # 静态图表
└── architecture.gif    # 动态图表
```

---

## 工作流程总结

```
┌──────────────────────────────────────────────────────────┐
│  使用 Skills 的项目生命周期                               │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  1. /setup-goal ──────────────> 初始化系统              │
│           │                                               │
│           v                                               │
│  2. /score-check ────────────> 了解基线                  │
│           │                                               │
│           v                                               │
│  3. /improvement-loop ───────> 运行迭代                 │
│           │                    (直到满意)                  │
│           │                                               │
│           v                                               │
│  4. /new-goal-loop ─────────> 创建监控循环              │
│           │                                               │
│           v                                               │
│  5. /context-audit ──────────> 保持上下文清晰           │
│                                                           │
└──────────────────────────────────────────────────────────┘
```

---

## 下一步

- [Skills 概述](../skills-integration/01-overview.md) - 架构
- [整合方案](../skills-integration/) - 详细设计
