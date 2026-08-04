# Score Check Skill

**Quick check of current project score**

---

## Overview

The `/score-check` skill runs the fitness function and reports the current project score with detailed breakdown.

## Trigger Words

- "check score"
- "run score"
- "what's our current score"
- "show me the score"

## Usage

### In Claude Code

```
/score-check
```

### As Standalone

```bash
./scripts/score.sh
```

Or with JSON output:

```bash
./scripts/score.sh --json
```

## Example Output

```
Score: 85 / 100
├── format      : 20 / 20 ✓
├── clippy      : 20 / 20 ✓
├── tests       : 25 / 25 ✓
├── docs        : 10 / 15 ◐
├── maintenance : 10 / 10 ✓
└── safety      :  0 / 10 ✗

Weakest: safety (0/10)
Next action: Add safety documentation (+10)

Run: /improvement-loop to improve
```

## Score Components

| Component | Max | Description |
|-----------|-----|-------------|
| format | 20 | Code formatting compliance |
| clippy | 20 | Lint warnings |
| tests | 25 | Test coverage |
| docs | 15 | Documentation |
| maintenance | 20 | CI/CD, gitignore, etc. |

## Understanding the Score

### Score 90-100 ✓
- Excellent code quality
- Keep maintaining standards

### Score 70-89 ◐
- Good baseline
- Identify and fix weak components

### Score 50-69 ✗
- Needs improvement
- Run improvement loop

### Score <50 ✗
- Critical issues
- Consider running multiple improvement iterations

## JSON Output

```bash
$ ./scripts/score.sh --json
{
  "score": 85,
  "max": 100,
  "components": {
    "format": {"score": 20, "max": 20},
    "clippy": {"score": 20, "max": 20},
    "tests": {"score": 25, "max": 25},
    "docs": {"score": 10, "max": 15},
    "maintenance": {"score": 10, "max": 10}
  }
}
```

## Next Steps

- [Improvement Loop](04-improvement-loop.md) - Improve your score
