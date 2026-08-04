# AutoHarness Skills

**Claude Code plugin for GOAL.md autonomous improvement + codebase harness**

---

## Overview

This plugin provides AI agents with the capabilities to:
- **Set up GOAL.md** - Initialize autonomous improvement systems
- **Score projects** - Measure code quality with fitness functions
- **Run improvement loops** - Execute iterative improvements
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

| Skill | Description |
|-------|-------------|
| `/setup-goal` | Initialize GOAL.md system |
| `/score-check` | Check current project score |
| `/improvement-loop` | Run one improvement iteration |

### Agent Loops

| Skill | Description |
|-------|-------------|
| `/new-goal-loop` | Create a new workstream |

### Utilities

| Skill | Description |
|-------|-------------|
| `/context-audit` | Audit agent context |
| `/flow-diagram` | Generate flow diagrams |

## Quick Start

1. Install the plugin
2. Run `/setup-goal` in your project
3. Run `/score-check` to see current score
4. Run `/improvement-loop` to start improving

## Structure

```
skills/
├── goal-md/                    # GOAL.md system
│   ├── setup-goal/            # Initialize GOAL.md
│   ├── score-check/           # Check score
│   └── improvement-loop/     # Run improvement
├── loops/                     # Agent loops
│   └── new-goal-loop/        # Create loops
└── utilities/                 # Utilities
    ├── context-audit/        # Context audit
    └── flow-diagram/         # Diagrams
```

## Documentation

- [Skills Integration Plan](../docs/goal-md/skills-integration/) - Detailed design
- [GOAL.md Tutorials](../docs/goal-md/tutorial/) - Learning resources

## License

MIT
