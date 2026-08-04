# Skills Overview

**AutoHarness Skills - Agent Capabilities for Autonomous Improvement**

---

## What are Skills?

Skills are reusable AI agent capabilities based on the [AI Builder Club](https://github.com/AI-Builder-Club/skills) pattern. They provide structured, repeatable workflows that agents can execute reliably.

## Available Skills

| Skill | Category | Purpose |
|-------|----------|---------|
| `/setup-goal` | GOAL.md | Initialize GOAL.md system |
| `/score-check` | GOAL.md | Check project score |
| `/improvement-loop` | GOAL.md | Run improvement iteration |
| `/new-goal-loop` | Loops | Create new workstream |
| `/context-audit` | Utilities | Audit agent context |
| `/flow-diagram` | Utilities | Generate diagrams |

## Quick Navigation

- [01-installation.md](01-installation.md) - Installation and setup
- [02-setup-goal.md](02-setup-goal.md) - How to use setup-goal
- [03-score-check.md](03-score-check.md) - How to use score-check
- [04-improvement-loop.md](04-improvement-loop.md) - How to use improvement-loop
- [05-examples.md](05-examples.md) - Real-world examples

## Core Concepts

### GOAL.md System

The GOAL.md system enables **autonomous improvement**:
1. **Fitness Function** - A script that outputs a score (0-100)
2. **Action Catalog** - Actions that can improve the score
3. **Improvement Loop** - Measure → Act → Verify → Log

### Skill Structure

```
skills/
├── goal-md/
│   └── setup-goal/
│       ├── SKILL.md          # Skill definition
│       ├── assets/           # Templates
│       └── references/       # Documentation
```

### Trigger Words

Each skill responds to specific phrases:
- "set up GOAL.md" → `/setup-goal`
- "check score" → `/score-check`
- "improve the project" → `/improvement-loop`

## Next Steps

- [Installation Guide](01-installation.md) - Get started
