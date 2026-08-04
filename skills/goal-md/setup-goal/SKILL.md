---
name: setup-goal
description: >
  Initialize GOAL.md system for any project — creates the fitness function script,
  goal definition, and iteration log. Use when the user says "set up GOAL.md",
  "initialize goal tracking", or "add autonomous improvement to this repo".
user_invocable: true
---

# Set up GOAL.md

GOAL.md is a file format that enables AI agents to autonomously improve a project. 
The core idea: **Give the agent a number (score), and let it figure out how to make that number bigger.**

## What is GOAL.md?

GOAL.md consists of five core elements:

| Element | Description |
|---------|-------------|
| **Fitness Function** | A runnable scoring script that outputs a number |
| **Improvement Loop** | Measure → Diagnose → Act → Verify → Log |
| **Action Catalog** | Specific improvement actions with expected score impact |
| **Operating Mode** | Converge / Continuous / Supervised |
| **Constraints** | Rules the agent must follow |

## When to use

- "set up GOAL.md"
- "initialize goal tracking"
- "add autonomous improvement to this repo"
- "make this project agent-improving"

## Steps

### 1. Assess the project

Check the current project state:
- Project type (Rust/JS/Python/etc.)
- Existing test framework
- Existing lint tools
- Documentation structure

### 2. Create the scoring script

Create `scripts/score.sh`:

```bash
#!/bin/bash
# Fitness Function - Project Quality Score

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0
DOC_SCORE=0; MAINTENANCE_SCORE=0

# Format Check (20 points)
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# Clippy Check (20 points)
WARN_COUNT=$(cargo clippy 2>&1 | grep -c "warning:" || true)
[[ "$WARN_COUNT" -eq 0 ]] && CLIPPY_SCORE=20

# Test Coverage (25 points)
cargo test --no-run 2>/dev/null && TEST_SCORE=25

# Documentation (15 points)
[[ -f "README.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -f "docs" ]] && DOC_SCORE=$((DOC_SCORE + 5))

# Maintenance (20 points)
[[ -f ".gitignore" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ -f "Cargo.lock" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))

TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE))

echo "Score: $TOTAL / 100"
```

### 3. Create GOAL.md

Copy from template and fill in project info:

```markdown
# Goal: [Project Name] - [One-line goal]

## Fitness Function

./scripts/score.sh

## Operating Mode

- [x] **Converge** — Stop when goal is reached

Stop when:
- Score reaches 100/100
- 10 iterations with no improvement

## Action Catalog

| Action | Impact | How |
|--------|--------|-----|
| cargo fmt | +20 | `cargo fmt` |
| Fix clippy | +20 | `cargo clippy --fix` |

## Iteration Log

File: `iterations.jsonl`
```

### 4. Initialize iteration log

Create empty `iterations.jsonl`:

```bash
touch iterations.jsonl
```

## Output

- `scripts/score.sh` - Scoring script
- `GOAL.md` - Goal definition
- `iterations.jsonl` - Iteration log (empty)

## Principles

- **Incremental**: Start with basic scoring, improve over time
- **Verifiable**: Every action must be verifiable
- **Traceable**: All changes logged in iterations.jsonl
