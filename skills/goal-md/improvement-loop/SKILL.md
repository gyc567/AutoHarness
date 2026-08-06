---
name: improvement-loop
description: >
  Execute one complete GOAL.md improvement loop: measure, diagnose, plan, execute, verify, and log.
  Use when the user says "improve the project", "run one iteration", or "make the score better".
user_invocable: true
---

# Improvement Loop

Execute one complete GOAL.md improvement loop.

## When to use

- "improve the project"
- "run one iteration"
- "make the score better"
- "run the improvement loop"

## The Improvement Loop

```
repeat:
  1. ./scripts/score.sh --json > /tmp/before.json
  2. Analyze score - find weakest component
  3. Select highest-impact action from catalog
  4. Execute the change
  5. Run targeted verification
  6. ./scripts/score.sh --json > /tmp/after.json
  7. Compare: improved → commit, regressed → revert
  8. Append to iterations.jsonl
  9. Continue
```

## Steps

### 1. Measure

```bash
./scripts/score.sh --json > /tmp/before.json
```

### 2. Analyze

Read the score, identify the weakest component.

### 3. Plan

**Ponytail ladder self-check** — before picking an action, ask:
1. Does this need to exist at all? (YAGNI → noop is valid)
2. Already in this codebase? → reuse it
3. Stdlib does it? → use it
4. Native platform feature? → use it
5. Already-installed dependency? → use it
6. Can it be one line? → one line
7. Only then: the minimum code that works

Select the highest-impact action from the Action Catalog below.

### Type: Ponytail Refactor

Triggered when the ladder finds over-engineering in the codebase:

| Tag | Meaning | Action |
|-----|---------|--------|
| `stdlib` | hand-rolled, stdlib ships it | replace with stdlib |
| `native` | dependency does platform-native thing | use native instead |
| `yagni` | one-implementation abstraction | inline it |
| `shrink` | same logic, fewer lines | shorten |
| `delete` | dead code or zero callers | delete |

When logging a Ponytail Refactor, append to the iteration record:
```json
{"iteration":N,...,"action":"<what changed>","ponytail_tag":"stdlib|native|yagni|shrink|delete"}
```

### 4. Execute

### 4. Execute

Execute the selected action.

### 5. Verify

```bash
./scripts/score.sh --json > /tmp/after.json
```

### 6. Decide

- **Improved** → commit the changes
- **Regressed** → revert and log

### 7. Log

Append to `iterations.jsonl`:

```json
{"iteration":1,"timestamp":"2024-01-01T00:00:00Z","component":"clippy","before":15,"after":20,"action":"fix warnings","result":"kept"}

{"iteration":2,"timestamp":"2024-01-01T01:00:00Z","component":"maintenance","before":8,"after":12,"action":"inline one-implementation trait","result":"kept","ponytail_tag":"yagni"}
```

> For Ponytail Refactor actions, always include the `ponytail_tag` field (stdlib | native | yagni | shrink | delete).

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

## Principles

- **Score cannot decrease**: Each change must improve or maintain the score
- **One commit per change**: Atomic commits for easy rollback
- **Always format first**: Run `cargo fmt` before `cargo clippy`
- **Verify before commit**: Always run the score script after changes
- **Ponytail first** (see Code Norms below): before picking an action, run the 7-level ladder self-check
