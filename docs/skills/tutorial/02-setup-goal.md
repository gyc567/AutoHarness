# Setup Goal Skill

**Initialize GOAL.md system for any project**

---

## Overview

The `/setup-goal` skill creates the GOAL.md infrastructure for autonomous improvement. It generates:
- `scripts/score.sh` - Fitness function script
- `GOAL.md` - Goal definition file
- `iterations.jsonl` - Iteration log

## Trigger Words

- "set up GOAL.md"
- "initialize goal tracking"
- "add autonomous improvement"
- "make this project agent-improving"

## Usage

### In Claude Code

```
/setup-goal
```

### As Standalone

```bash
# Read and execute the skill
cat skills/goal-md/setup-goal/SKILL.md
```

## What It Does

### 1. Assesses the Project

Detects:
- Project type (Rust/Node.js/Python/etc.)
- Existing test framework
- Existing lint tools
- Documentation structure

### 2. Creates Scoring Script

Generates `scripts/score.sh` with components for:

| Component | Points | Checks |
|-----------|--------|--------|
| Format | 20 | `cargo fmt`, `prettier`, `black` |
| Lint | 20 | `cargo clippy`, `eslint` |
| Tests | 25 | `cargo test`, `npm test` |
| Docs | 15 | README, docs/ directory |
| Maintenance | 20 | .gitignore, CI/CD |

### 3. Creates GOAL.md

```markdown
# Goal: [Project Name] - [One-line goal]

## Fitness Function

./scripts/score.sh

## Operating Mode

- [x] **Converge** — Stop when goal is reached

## Action Catalog

| Action | Impact | How |
|--------|--------|-----|
| Format code | +20 | `cargo fmt` |
| Fix clippy | +20 | `cargo clippy --fix` |

## Iteration Log

File: `iterations.jsonl`
```

### 4. Initializes Log

Creates empty `iterations.jsonl`.

## Output Files

```
project/
├── scripts/
│   └── score.sh        # Generated
├── GOAL.md             # Generated
└── iterations.jsonl    # Generated (empty)
```

## Customization

### Add Custom Score Components

Edit `scripts/score.sh`:

```bash
# Add custom check
CUSTOM_SCORE=0
if your-custom-check; then
    CUSTOM_SCORE=10
fi
TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE + CUSTOM_SCORE))
```

### Add Custom Actions

Edit `GOAL.md` Action Catalog:

```markdown
| My custom action | +5 | How to do it |
```

## Examples

### Rust Project

```bash
$ /setup-goal
Detected: Rust project (Cargo.toml)
Creating: scripts/score.sh
Creating: GOAL.md
Creating: iterations.jsonl

Done! Run ./scripts/score.sh to see your score.
```

### Node.js Project

```bash
$ /setup-goal
Detected: Node.js project (package.json)
Creating: scripts/score.sh (prettier + eslint)
Creating: GOAL.md
Creating: iterations.jsonl

Done! Run ./scripts/score.sh to see your score.
```

## Next Steps

- [Score Check](03-score-check.md) - Check your initial score
- [Run Improvement Loop](04-improvement-loop.md) - Start improving
