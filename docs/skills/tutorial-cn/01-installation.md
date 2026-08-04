# 安装指南

**如何安装和配置 AutoHarness Skills**

---

## 前置要求

- [Claude Code](https://claude.com/claude-code)（推荐）或其他 CLI Agent
- Git

## 安装方式

### 方式一：克隆仓库（推荐）

```bash
# 克隆 AutoHarness
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness

# 验证 skills 目录
ls skills/
# goal-md/  loops/  utilities/
```

### 方式二：复制 Skills 目录

```bash
# 复制 skills 到你的项目
cp -r /path/to/AutoHarness/skills ./skills

# 或使用子模块
git submodule add https://github.com/gyc567/AutoHarness.git skills
```

### 方式三：Claude Code 插件（未来）

```bash
# 未来支持插件安装时
claude plugin add https://github.com/gyc567/AutoHarness
```

## 配置

### 设置 Skills 目录

在 Claude Code 的 `CLAUDE.md` 中添加：

```markdown
## Skills

AutoHarness Skills 位于: `./skills/`

可用 skills:
- `/setup-goal` - 初始化 GOAL.md
- `/score-check` - 检查分数
- `/improvement-loop` - 执行改进
```

### 验证安装

```bash
# 检查 skills 是否存在
ls skills/goal-md/setup-goal/SKILL.md
# 应输出: skills/goal-md/setup-goal/SKILL.md

# 运行简单测试
bash skills/goal-md/setup-goal/assets/score.template.sh
```

## 目录结构

```
AutoHarness/
├── skills/
│   ├── goal-md/              # GOAL.md Skills
│   │   ├── setup-goal/
│   │   ├── score-check/
│   │   └── improvement-loop/
│   ├── loops/                # 循环 Skills
│   │   └── new-goal-loop/
│   └── utilities/            # 工具 Skills
│       ├── context-audit/
│       └── flow-diagram/
├── .claude-plugin/           # 插件配置
└── docs/                     # 文档
```

## 下一步

- [Setup GOAL.md](02-setup-goal.md) - 为项目初始化
