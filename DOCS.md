# DOCS.md

**开发文档索引** — 所有文档文件位于 `docs/` 目录，长期积累，不删除。

---

## 索引

### Architecture (架构)

- [项目概览](docs/architecture/overview.md)
- [模块设计](docs/architecture/modules.md)
- [北极星指标](docs/architecture/north-star-metrics.md)

### API

- [CLI 使用指南](docs/api/cli.md)
- [内部 API 文档](docs/api/internal.md)

### Guides (指南)

- [快速开始](docs/guides/getting-started.md)
- [前端优化规范](docs/guides/frontend-optimization.md)
- [回测评估规范](docs/guides/backtesting.md)
- [性能调优规范](docs/guides/performance-tuning.md)
- [代码审查规范](docs/guides/code-review.md)

### Skills (技能系统)

- [Skills 整合方案](docs/goal-md/skills-integration/README.md) - AI-Builder-Club 技能整合
- [Skills 审计报告](docs/goal-md/skills-integration/00-audit-report.md) - 整合状态
- [Skills 概述](docs/goal-md/skills-integration/01-overview.md) - 设计概览
- [Plugin 配置](docs/goal-md/skills-integration/02-plugin-setup.md) - Claude Code 插件配置
- [GOAL.md Skills](docs/goal-md/skills-integration/03-goal-md-skills.md) - 自主改进系统
- [Codebase Harness](docs/goal-md/skills-integration/04-codebase-harness.md) - 代码库基础设施
- [实施计划](docs/goal-md/skills-integration/05-implementation-plan.md) - 实施步骤

#### Skills 教程

- [Skills 教程索引](docs/skills/tutorial/README.md)
- [安装指南](docs/skills/tutorial/01-installation.md)
- [设置 GOAL](docs/skills/tutorial/02-setup-goal.md)
- [评分检查](docs/skills/tutorial/03-score-check.md)
- [改进循环](docs/skills/tutorial/04-improvement-loop.md)
- [示例](docs/skills/tutorial/05-examples.md)

### Internals (内部实现)

- [合成引擎原理](docs/internals/synthesis-engine.md)
- [状态机设计](docs/internals/state-machine.md)

### GOAL.md (自主改进)

- [GOAL.md 融合方案](docs/goal-md/GOAL-md-融合方案.md)
- [快速开始](docs/goal-md/快速开始.md)
- [API 参考](docs/goal-md/API-参考.md)
- [CLAUDE.md](CLAUDE.md) - Agent 指导文件
- [template/GOAL.md](template/GOAL.md) - GOAL.md 可复用模板

#### GOAL.md 教程

- [教程索引](docs/goal-md/tutorial/README.md) - 完整学习路径
- [5 分钟快速开始](docs/goal-md/tutorial/01-quick-start.md)
- [创建第一个 GOAL.md](docs/goal-md/tutorial/03-create-goal.md)
- [多 Agent 协作](docs/goal-md/tutorial/04-multi-agent.md)
- [进阶模式](docs/goal-md/tutorial/05-advanced-patterns-1.md)
- [常见问题](docs/goal-md/tutorial/06-troubleshooting.md)

#### 示例

- [Rust 项目示例](examples/01-rust-code-quality.md)
- [测试合成项目示例](examples/02-test-synthesis.md)
- [CLI 工具示例](examples/03-cli-tool.md)
- [库项目示例](examples/04-library.md)

---

## 使用说明

### 文档层级规范

```
docs/
├── architecture/     # 一级: 架构
├── api/              # 一级: API
├── guides/           # 一级: 指南
├── skills/           # 一级: 技能系统
└── internals/        # 一级: 内部实现
```

**命名规则**:
- 目录名: `snake_case`
- 文件名: `snake_case.md`

### 文档模板

```markdown
# 文档标题

## 概述
简要说明本文档的内容。

## 详细内容

### 子主题 A
...

### 子主题 B
...

## 相关文档
- [相关文档A](docs/path/to/doc-a.md)
- [相关文档B](docs/path/to/doc-b.md)
```

### 防止文件膨胀

- 单个文件建议 ≤ 500 行
- 超过时自动拆分新文件
- 在索引中注册新文件

---

## 多级索引示例

如需三级索引，按如下格式：

```markdown
### Guides (指南)

#### Frontend (前端)
- [前端优化规范](docs/guides/frontend-optimization.md)
- [UI 组件文档](docs/guides/frontend/components.md)

#### Backend (后端)
- [性能调优规范](docs/guides/performance-tuning.md)
```

---

## Skills 系统说明

AutoHarness Skills 基于 [AI-Builder-Club/skills](https://github.com/AI-Builder-Club/skills) 整合而来。

### 当前整合状态

| 类别 | 数量 | 状态 |
|------|------|------|
| GOAL.md Skills | 3 | ✅ 100% |
| Codebase Harness | 4 | ✅ 100% |
| Loops | 1 | ✅ 100% |
| Utilities | 3 | ✅ 100% |
| **总计** | **11** | **100%** |

### 可用 Skills

| Skill | 用途 |
|-------|------|
| `/setup-goal` | 初始化 GOAL.md 系统 |
| `/score-check` | 检查项目分数 |
| `/improvement-loop` | 执行改进循环 |
| `/setup-harness` | 设置代码库 harness |
| `/dev-local` | 本地开发环境 |
| `/e2e-setup` | E2E 测试套件 |
| `/verify` | 验证代码 |
| `/new-goal-loop` | 创建新循环 |
| `/open-agent-teams` | 多 Agent 协作 |
| `/context-audit` | 上下文审计 |
| `/flow-diagram` | 流程图生成 |

---

**Last Updated**: 2026-08-05
