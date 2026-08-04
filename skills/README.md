# AutoHarness Skills

**Claude Code plugin for GOAL.md autonomous improvement + codebase harness**

---

## Overview

This plugin provides AI agents with the capabilities to:
- **Set up GOAL.md** - Initialize autonomous improvement systems
- **Score projects** - Measure code quality with fitness functions
- **Run improvement loops** - Execute iterative improvements
- **Set up codebase harness** - Make any repo agent-ready
- **Verify code** - Validate changes before shipping
- **Create agent loops** - Build compounding workstreams
- **Audit context** - Optimize agent instructions
- **Generate diagrams** - Visualize workflows

## Installation

```bash
# Install via Claude Code
claude plugin add https://github.com/gyc567/AutoHarness

# Or clone locally
git clone https://github.com/gyc567/AutoHarness.git
```

## Available Skills

### GOAL.md System

| Skill | Description | Status |
|-------|-------------|--------|
| `/setup-goal` | Initialize GOAL.md system | ✅ |
| `/score-check` | Check current project score | ✅ |
| `/improvement-loop` | Run one improvement iteration | ✅ |

### Codebase Harness

| Skill | Description | Status |
|-------|-------------|--------|
| `/setup-harness` | Make repo agent-ready (master) | ✅ |
| `/dev-local` | Set up one-command dev environment | ✅ |
| `/verify` | Verify code before shipping | ✅ |

### Agent Loops

| Skill | Description | Status |
|-------|-------------|--------|
| `/new-goal-loop` | Create a new workstream | ✅ |

### Utilities

| Skill | Description | Status |
|-------|-------------|--------|
| `/context-audit` | Audit agent context | ✅ |
| `/flow-diagram` | Generate flow diagrams | ✅ |

## Quick Start

### GOAL.md Workflow

1. Install the plugin
2. Run `/setup-goal` in your project
3. Run `/score-check` to see current score
4. Run `/improvement-loop` to start improving

### Codebase Harness Workflow

1. Run `/setup-harness` to make the repo agent-ready
2. Use `/dev-local` for development
3. Use `/verify` before opening a PR

## Structure

```
skills/
├── .claude-plugin/           # Plugin configuration
│   ├── plugin.json
│   └── marketplace.json
├── goal-md/                  # GOAL.md system
│   ├── setup-goal/
│   ├── score-check/
│   └── improvement-loop/
├── codebase-harness/         # Codebase harness (NEW)
│   ├── setup-harness/
│   ├── dev-local/
│   └── verify/
├── loops/                    # Agent loops
│   └── new-goal-loop/
└── utilities/                # Utilities
    ├── context-audit/
    └── flow-diagram/
```

## Documentation

- [Skills Integration Plan](../docs/goal-md/skills-integration/) - Detailed design
- [GOAL.md Tutorials](../docs/goal-md/tutorial/) - Learning resources
- [Audit Report](../docs/goal-md/skills-integration/00-audit-report.md) - Implementation status

## License

MIT
