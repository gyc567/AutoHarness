---
name: score-check
description: >
  Quick check of the current project score using the GOAL.md fitness function.
  Use when the user says "check score", "run score", or "what's our current score".
user_invocable: true
---

# Score Check

Run the scoring script and report the current project state.

## When to use

- "check score"
- "run score"
- "what's our current score"
- "show me the score"

## Steps

1. Run `./scripts/score.sh`
2. Parse and display the score
3. Identify the weakest components
4. Suggest the next improvement action

## Example Output

```
Score: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
├── maintenance : 20 / 20 ✓
└── safety      :  0 / 10 ✗

Weakest: safety (0/10)
Next action: Add safety documentation (+10)

Run: /improvement-loop to improve
```

## When GOAL.md doesn't exist

If `scripts/score.sh` doesn't exist:

1. Run `/setup-goal` first to initialize the GOAL.md system
2. Then run this skill again

## Principles

- **Fast response**: Complete in under 30 seconds
- **Actionable**: Always suggest next step
- **Identify bottlenecks**: Focus on lowest-scoring components
