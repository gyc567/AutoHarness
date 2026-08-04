# Installation Guide

**How to install and configure AutoHarness Skills**

---

## Prerequisites

- [Claude Code](https://claude.com/claude-code) (recommended) or any CLI agent
- Git

## Installation Methods

### Method 1: Clone Repository (Recommended)

```bash
# Clone AutoHarness
git clone https://github.com/gyc567/AutoHarness.git
cd AutoHarness

# Verify skills directory
ls skills/
# goal-md/  loops/  utilities/
```

### Method 2: Copy Skills Directory

```bash
# Copy skills to your project
cp -r /path/to/AutoHarness/skills ./skills

# Or use as submodule
git submodule add https://github.com/gyc567/AutoHarness.git skills
```

### Method 3: Claude Code Plugin (Future)

```bash
# When Claude Code plugin support is available
claude plugin add https://github.com/gyc567/AutoHarness
```

## Configuration

### Set Skills Directory

For Claude Code, add to your `CLAUDE.md`:

```markdown
## Skills

AutoHarness Skills are located at: `./skills/`

Available skills:
- `/setup-goal` - Initialize GOAL.md
- `/score-check` - Check score
- `/improvement-loop` - Run improvement
```

### Verify Installation

```bash
# Check skills are present
ls skills/goal-md/setup-goal/SKILL.md
# Should output: skills/goal-md/setup-goal/SKILL.md

# Run a simple test
bash skills/goal-md/setup-goal/assets/score.template.sh
```

## Directory Structure

```
AutoHarness/
├── skills/
│   ├── goal-md/              # GOAL.md Skills
│   │   ├── setup-goal/
│   │   ├── score-check/
│   │   └── improvement-loop/
│   ├── loops/                # Loop Skills
│   │   └── new-goal-loop/
│   └── utilities/            # Utility Skills
│       ├── context-audit/
│       └── flow-diagram/
├── .claude-plugin/           # Plugin config
└── docs/                     # Documentation
```

## Next Steps

- [Setup GOAL.md](02-setup-goal.md) - Initialize for your project
