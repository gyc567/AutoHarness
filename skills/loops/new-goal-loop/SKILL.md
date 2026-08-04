---
name: new-goal-loop
description: >
  Create a new goal loop (domain) for recurring improvement work — bootstrap the GOAL.md
  system if missing, scaffold a domain README, then do ONE real test run and record it.
  Use when the user says "set up a new loop", "create a new goal domain", or "start a new improvement workstream".
user_invocable: true
---

# new-goal-loop — Create a new goal loop

A **loop** is a recurring thread of work the agent owns: a charter, a cadence, and
the artifacts it produces. This skill creates one, proves it works with a single real run, and
leaves behind a `domains/<name>/README.md` that is the loop's live state.

## When to use

The user wants to stand up a new workstream/beat/job (e.g. "a weekly SEO loop", "a code
quality loop", "a documentation loop"). Don't use this for a one-off task.

## Inputs to gather (ask only what's missing)

Infer from the request; ask a short clarifying round only for what you can't:

1. **name** — kebab-case, the loop's home folder (`domains/<name>/`). Keep it short.
2. **goal** — one line: the outcome this loop drives.
3. **cadence** — `manual` / `daily` / `weekly` / a cron expr. Default `manual`.
4. **what it does** — what it consumes (signals? data? an inbox? a URL?) and produces
   (GOAL.md improvements? docs? code changes?).
5. **tools/data** — sources or credentials it needs.

## Procedure

### 1. Bootstrap GOAL.md if missing

Check for:
- `scripts/score.sh`
- `GOAL.md`
- `iterations.jsonl`

**All present** → skip to Step 2.
**Anything missing** → run `/setup-goal` to initialize the GOAL.md system.

### 2. Scaffold the loop README

Create `domains/<name>/README.md` with:

```markdown
---
kind: domain
domain: <name>
status: active
goal: <one-line goal>
cadence: <manual|daily|weekly|cron>
---

# <Name> Loop

## Goal

<one-line goal>

## Cadence

<manual|daily|weekly|cron>

## What it does

<description>

## Current Focus

TBD

## Backlog

- [ ] Task 1
- [ ] Task 2

## Timeline

| Date | Event | Result |
|------|-------|--------|
| YYYY-MM-DD | test run | <what happened> |
```

### 3. Do ONE real test run

Actually run the loop once, at small scale.

### 4. Record the test run

Append to the loop's Timeline and to `LOG.md`.

## Output

- `domains/<name>/README.md` - Domain README
- `LOG.md` entry - Test run record

## Principles

- **Don't gold-plate**: Start lean; let the README grow via its Timeline
- **One loop = one separable workstream**: If it's part of an existing loop, add it there
