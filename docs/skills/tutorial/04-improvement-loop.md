# Improvement Loop Skill

**Execute one complete GOAL.md improvement iteration**

---

## Overview

The `/improvement-loop` skill executes a complete improvement cycle:
1. Measure current score
2. Analyze weaknesses
3. Plan improvement action
4. Execute change
5. Verify result
6. Log iteration

## Trigger Words

- "improve the project"
- "run one iteration"
- "make the score better"
- "run the improvement loop"

## Usage

### In Claude Code

```
/improvement-loop
```

### As Standalone

Follow the loop manually:
1. `./scripts/score.sh --json > /tmp/before.json`
2. Analyze and execute improvement
3. `./scripts/score.sh --json > /tmp/after.json`
4. Compare and commit

## The Improvement Loop

```
┌─────────────────────────────────────────────────┐
│  Improvement Loop                               │
├─────────────────────────────────────────────────┤
│                                                 │
│  1. Measure ────> ./scripts/score.sh           │
│       │                                         │
│       v                                         │
│  2. Analyze ───> Find weakest component        │
│       │                                         │
│       v                                         │
│  3. Plan ───────> Select highest-impact action │
│       │                                         │
│       v                                         │
│  4. Execute ────> Make the change              │
│       │                                         │
│       v                                         │
│  5. Verify ────> Run score again               │
│       │                                         │
│       v                                         │
│  6. Decide ─────> Improved? → Commit           │
│       │             Regressed? → Revert        │
│       v                                         │
│  7. Log ────────> Append to iterations.jsonl   │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Example Output

```
=== Improvement Loop Iteration #1 ===

Before: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
└── maintenance : 10 / 10 ✓

Action: Add documentation (+5)
Executing: created docs/api.md
After: 90 / 100 ✓

Change: KEPT (+5)
Logged to: iterations.jsonl
```

## Key Principles

### 1. Score Cannot Decrease

If a change causes regression, **revert it immediately**.

```
Before: 85/100 → After: 80/100 → REVERT!
```

### 2. One Change Per Commit

Atomic commits make rollback easy.

```bash
git add -p  # Stage only related changes
git commit -m "docs: add API documentation (+5)"
```

### 3. Always Verify Before Commit

```bash
./scripts/score.sh  # Must show improvement
git commit
```

### 4. Log Every Iteration

Append to `iterations.jsonl`:

```json
{"iteration":1,"timestamp":"2024-01-01T00:00:00Z","component":"docs","before":10,"after":15,"action":"add api.md","result":"kept"}
```

## Action Catalog

Typical actions in order of impact:

| Priority | Action | Impact | When |
|----------|--------|--------|------|
| 1 | `cargo fmt` | +20 | Always first |
| 2 | `cargo clippy --fix` | +20 | After format |
| 3 | Add tests | +15 | When < 100% |
| 4 | Improve docs | +10 | Weekly |
| 5 | CI/CD setup | +10 | Once |

## Iterations Log Format

```jsonl
{"iteration":N,"timestamp":"ISO8601","component":"name","before":N,"after":N,"action":"description","result":"kept|reverted"}
```

## Tips for Effective Improvement

1. **Start with format** - Always fix formatting first
2. **Focus on weak components** - Target lowest-scoring areas
3. **Small changes** - One thing at a time
4. **Verify frequently** - Run score after each change

## Next Steps

- [Examples](05-examples.md) - Real-world examples
