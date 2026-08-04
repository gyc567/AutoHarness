# Skills 整合方案 - 插件配置

**Claude Code 插件配置详解**

---

## 一、插件结构

```
.claude-plugin/
├── marketplace.json      # 市场配置
└── plugin.json          # 插件元数据
```

---

## 二、plugin.json

```json
{
  "name": "autoharness",
  "description": "AutoHarness: GOAL.md autonomous improvement + codebase harness for AI agents. Set up GOAL.md scoring, run improvement loops, verify code before shipping, and create compounding agent loops.",
  "version": "0.1.0",
  "author": {
    "name": "AutoHarness Team",
    "url": "https://github.com/gyc567/AutoHarness"
  },
  "homepage": "https://github.com/gyc567/AutoHarness",
  "license": "MIT",
  "keywords": [
    "autoharness",
    "goal-md",
    "autonomous-improvement",
    "codebase-harness",
    "loop-engineer",
    "ai-agents",
    "code-quality",
    "verification",
    "test-synthesis"
  ]
}
```

---

## 三、marketplace.json

```json
{
  "name": "autoharness",
  "owner": {
    "name": "AutoHarness",
    "url": "https://github.com/gyc567/AutoHarness"
  },
  "metadata": {
    "description": "GOAL.md autonomous improvement + codebase harness for AI agents",
    "version": "0.1.0"
  },
  "plugins": [
    {
      "name": "goal-md",
      "source": "./skills/goal-md",
      "description": "GOAL.md autonomous improvement system",
      "category": "engineering"
    },
    {
      "name": "codebase-harness",
      "source": "./skills/codebase-harness",
      "description": "Codebase harness for agent-ready development",
      "category": "engineering"
    },
    {
      "name": "loops",
      "source": "./skills/loops",
      "description": "Compounding agent loops",
      "category": "automation"
    },
    {
      "name": "utilities",
      "source": "./skills/utilities",
      "description": "Utility skills",
      "category": "utilities"
    }
  ]
}
```

---

## 四、目录结构

```bash
.claude-plugin/
├── marketplace.json
└── plugin.json

skills/
├── goal-md/
│   ├── setup-goal/
│   ├── score-check/
│   └── improvement-loop/
├── codebase-harness/
│   ├── setup-harness/
│   ├── dev-local/
│   └── verify/
├── loops/
│   └── new-goal-loop/
└── utilities/
    ├── context-audit/
    └── flow-diagram/
```

---

## 五、Category 说明

| Category | 说明 | Skills |
|----------|------|--------|
| engineering | 工程类技能 | goal-md, codebase-harness |
| automation | 自动化技能 | loops |
| utilities | 工具类技能 | utilities |

---

## 六、下一步

- [03-goal-md-skills.md](03-goal-md-skills.md) - GOAL.md Skills 设计
