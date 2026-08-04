# Skills Examples

**Real-world examples of using AutoHarness Skills**

---

## Example 1: Initialize and Improve a Rust Project

### Step 1: Setup GOAL.md

```bash
$ /setup-goal
Detected: Rust project (Cargo.toml)
Creating: scripts/score.sh
Creating: GOAL.md
Creating: iterations.jsonl

✓ GOAL.md system initialized
Run ./scripts/score.sh to see your score.
```

### Step 2: Check Initial Score

```bash
$ /score-check
Score: 70 / 100
├── format      : 20 / 20 ✓
├── clippy      : 15 / 20 ◐
├── tests       : 25 / 25 ✓
├── docs        :  5 / 15 ✗
├── maintenance :  5 / 20 ◐
└── safety      :  0 / 10 ✗

Weakest: docs (5/15)
Next action: Improve documentation
```

### Step 3: Run Improvement Loop

```bash
$ /improvement-loop
=== Improvement Loop Iteration #1 ===

Before: 70 / 100
Action: Add documentation (+10)
Executing: created docs/api.md
After: 80 / 100 ✓

Change: KEPT (+10)
Logged to: iterations.jsonl

$ /improvement-loop
=== Improvement Loop Iteration #2 ===

Before: 80 / 100
Action: Fix clippy warnings (+5)
Executing: cargo clippy --fix
After: 85 / 100 ✓

Change: KEPT (+5)
Logged to: iterations.jsonl
```

---

## Example 2: Node.js Project

### Setup

```bash
$ /setup-goal
Detected: Node.js project (package.json)
Creating: scripts/score.sh (prettier + eslint)
Creating: GOAL.md
Creating: iterations.jsonl
```

### Score

```bash
$ /score-check
Score: 60 / 100
├── format      : 20 / 20 ✓
├── lint        : 10 / 20 ◐
├── tests       : 20 / 25 ◐
├── docs        :  5 / 15 ✗
└── maintenance :  5 / 20 ◐

Weakest: docs (5/15)
```

### Improve

```bash
$ /improvement-loop
=== Improvement Loop Iteration #1 ===

Before: 60 / 100
Action: Add ESLint configuration (+10)
Executing: npx eslint --init
After: 70 / 100 ✓

Change: KEPT (+10)
```

---

## Example 3: Create a New Goal Loop

### Create Loop

```bash
$ /new-goal-loop
Loop name: code-quality
Goal: Maintain 100% code quality score
Cadence: weekly

Creating: domains/code-quality/README.md
Bootstrap: ✓ GOAL.md system exists

=== Test Run ===
Running: weekly quality check
Result: Score 85/100, no critical issues

Timeline: ✓ Updated
LOG.md: ✓ Updated
```

### Loop Structure

```markdown
# domains/code-quality/README.md

---
kind: domain
domain: code-quality
status: active
goal: Maintain 100% code quality score
cadence: weekly
---

## Current Focus

Weekly quality audit

## Backlog

- [ ] Add security scanning
- [ ] Set up coverage reports

## Timeline

| Date | Event | Result |
|------|-------|--------|
| 2024-01-01 | test run | Score 85/100 |
```

---

## Example 4: Context Audit

### Run Audit

```bash
$ /context-audit
Auditing: CLAUDE.md
Auditing: skills/**/*.md
Auditing: README.md

=== Findings Report ===

| File | Size | High | Medium | Low | Verdict |
|------|------|------|--------|-----|---------|
| CLAUDE.md | 150 | 1 | 2 | 3 | keep |
| README.md | 200 | 0 | 1 | 5 | keep |

### Key Findings

| File:Line | Issue | Severity | Fix |
|-----------|-------|----------|-----|
| CLAUDE.md:42 | Hard rule "NEVER use unwrap" | high | Change to judgment |
| CLAUDE.md:50 | Duplicate lint instructions | medium | Remove duplicate |

Apply fixes? [y/n]
```

---

## Example 5: Flow Diagram

### Create Diagram

```bash
$ /flow-diagram
Source: docs/architecture.md

Creating JSON spec from architecture...
Spec created: /tmp/architecture-spec.json

Rendering diagram...
python3 scripts/render_diagram.py --spec /tmp/architecture-spec.json --outdir ./docs --basename architecture

✓ Rendered: docs/architecture.png
✓ Rendered: docs/architecture.gif
```

### Output

```
docs/
├── architecture.md
├── architecture.png    # Static diagram
└── architecture.gif    # Animated diagram
```

---

## Workflow Summary

```
┌──────────────────────────────────────────────────────────┐
│  Project Lifecycle with Skills                           │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  1. /setup-goal ──────────────> Initialize system        │
│           │                                               │
│           v                                               │
│  2. /score-check ────────────> Know your baseline        │
│           │                                               │
│           v                                               │
│  3. /improvement-loop ───────> Run iterations           │
│           │                    (until satisfied)         │
│           │                                               │
│           v                                               │
│  4. /new-goal-loop ─────────> Create monitoring loops  │
│           │                                               │
│           v                                               │
│  5. /context-audit ──────────> Keep context clean       │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Next Steps

- [Skills Overview](../skills-integration/01-overview.md) - Architecture
- [Integration Plan](../skills-integration/) - Detailed design
