# Skills 整合方案 - Codebase Harness

**代码库 Harness 的 Skill 设计**

---

## 一、概念

### 1.1 什么是 Codebase Harness？

Codebase Harness 是让代码库对 AI Agent 可用的基础设施，包括：
- **可读 (Legible)** - 清晰的文档和结构
- **可执行 (Executable)** - 一键启动的开发环境
- **可验证 (Verifiable)** - 验证代码工作的方法

### 1.2 核心思想

> **"模型是固定的 - 我们设计的是它周围的基础设施"**

Agent 的能力取决于代码库提供的基础设施，而非 Agent 本身。

---

## 二、Skills 列表

| Skill | 用途 | user_invocable |
|-------|------|----------------|
| setup-harness | 主协调器 | true |
| dev-local | 本地开发环境 | true |
| verify | 验证技能 | true |

---

## 三、setup-harness Skill

### 3.1 用途

主协调器，设置完整代码库 harness。

### 3.2 触发词

- "set up the harness"
- "make this repo agent-ready"
- "harness this codebase"

### 3.3 核心步骤

```
1. 评估 → 了解现有基础设施
2. 可读 → 添加文档和 lint
3. 可执行 → 设置 dev-local
4. 可验证 → 设置 verify
5. 维护 → 添加提交规范
```

### 3.4 SKILL.md 核心内容

```yaml
---
name: setup-harness
description: >
  Master skill — set up the full agent harness for any repo so an agent can work
  it reliably: legible (map + docs + custom lints), executable (one-command dev
  stack), verifiable (verify-before-ship loop). Use when onboarding a new codebase
  to agent-driven development.
user_invocable: true
---

# Set up the Codebase Harness

## 四支柱

1. **Legible (可读)** - Agent 能理解代码库
2. **Executable (可执行)** - Agent 能运行代码
3. **Verifiable (可验证)** - Agent 能验证代码
4. **Maintainable (可维护)** - 代码库保持整洁

## 步骤

### 1. 评估

调查代码库：
- 堆栈和包管理器
- 服务和端口
- 基础设施依赖
- 现有文档/测试/CI

### 2. Legible

- 创建精简的 AGENTS.md/CLAUDE.md
- 添加文档系统
- 添加自定义 lint

### 3. Executable

- 运行 dev-local-setup
- 创建 scripts/dev-local.sh

### 4. Verifiable

- 运行 e2e-setup
- 运行 verifier-setup
- 创建 /verify skill

### 5. Maintainable

- 添加提交规范
- 设置 lint-staged
```

---

## 四、dev-local Skill

### 4.1 用途

一键启动本地开发环境。

### 4.2 触发词

- "set up dev-local"
- "one-command dev"
- "start the dev stack"

### 4.3 输出

`scripts/dev-local.sh`

### 4.4 功能

```bash
# 启动所有服务
./scripts/dev-local.sh up

# 停止所有服务
./scripts/dev-local.sh down

# 查看状态
./scripts/dev-local.sh status

# 查看日志
./scripts/dev-local.sh logs <service>
```

### 4.5 模板

```bash
#!/bin/bash
# Dev Local Setup Script

set -e

# 发现服务...
# 启动 tmux session
# 每个服务一个窗口
# 等待服务就绪
# 打印 URL
```

---

## 五、verify Skill

### 5.1 用途

验证代码工作后再部署。

### 5.2 触发词

- "verify this"
- "check if it works"
- "run verification"

### 5.3 核心流程

```
1. 启动开发栈
2. 启动验证 agent
3. 驱动应用
4. 截图/视频证明
5. 运行回归测试
6. 打开 PR
```

### 5.4 模板

```yaml
---
name: verify
description: >
  Verify that code works before shipping. Use when the user says "verify this",
  or before opening a PR.
user_invocable: true
---

# Verify Before Ship

## 前置条件

- 分支有更改已提交
- dev-local 栈已启动

## 步骤

1. 运行 e2e 测试
2. 截图关键流程
3. 运行回归测试
4. 报告结果
```

---

## 六、文件结构

```
skills/codebase-harness/
├── setup-harness/
│   ├── SKILL.md
│   └── references/
│       └── harness-guide.md
├── dev-local/
│   ├── SKILL.md
│   └── assets/
│       └── dev-local.template.sh
└── verify/
    ├── SKILL.md
    └── assets/
        └── verify.template.md
```

---

## 七、与其他 Skills 的关系

```
setup-harness (orchestrator)
    ├── dev-local (creates dev-local.sh)
    └── verify (creates /verify skill)

setup-goal (creates GOAL.md)
    └── score.sh (can use dev-local)
```

---

## 八、下一步

- [05-implementation-plan.md](05-implementation-plan.md) - 实施计划
