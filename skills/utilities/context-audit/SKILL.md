---
name: context-audit
description: >
  Audit a repo's agent context — CLAUDE.md, codebase docs, skills, and tool/MCP designs —
  against best practices. Finds overconstraint, conflicting instructions, redundancy, stale
  facts, and missing "unknown knowns"; produces a scored findings report with concrete rewrites.
  Use when someone says "audit my CLAUDE.md", "context audit", or "unhobble this repo".
user_invocable: true
---

# agent-context-audit — Audit agent context

Goal: find where this repo's context (CLAUDE.md, docs, skills, tool designs)
**hobbles** an AI agent — overconstrains it, contradicts itself, repeats itself, 
or hides context the agent actually needs — and leave behind a findings report plus approved fixes.

## The Six Shifts (audit rubric)

Every finding maps to one of these:

1. **Rules → Judgment** — Hard rules ("NEVER...", "ALWAYS...") should become judgment framing
2. **Examples → Interface** — Long examples should become expressive interfaces
3. **Upfront → Progressive** — Long docs should move to skills, loaded on demand
4. **Repetition → Single-home** — Each instruction has exactly one home
5. **Manual → Automatic** — Hand-maintained notes are obsolete with auto-memory
6. **Simple → Rich** — Point at source files, not prose paraphrases

## Steps

### 1. Inventory artifacts

Collect all agent-facing docs:
- CLAUDE.md / AGENTS.md
- skills/**/*.md
- README, CONTRIBUTING
- Tool definitions

### 2. Audit each artifact

For each file, check for:
- Overconstraint (too many rules)
- Conflicts (contradicting instructions)
- Redundancy (same info in multiple places)
- Staleness (outdated info)
- Missing gaps (needed info not present)

### 3. Report findings

```markdown
## Findings Report

### Scorecard

| File | Size | High | Medium | Low | Verdict |
|------|------|------|--------|-----|---------|
| CLAUDE.md | 150 lines | 1 | 2 | 3 | keep |

### Findings

| File:Line | Quote | Shift | Severity | Fix |
|-----------|-------|-------|----------|-----|
| CLAUDE.md:42 | "NEVER use unwrap" | 1 | high | Change to judgment framing |

### Gaps

| Missing | Proposed text | Home |
|---------|--------------|------|
| How to run tests | `./scripts/score.sh` | CLAUDE.md |
```

### 4. Apply fixes (with approval)

Ask which findings to apply, then make the edits.

## Output

- `docs/context-audit-YYYY-MM-DD.md` - Findings report

## Principles

- **Audit first, fix second**: Don't edit anything until Step 4
- **Concrete rewrites**: Every finding must include the proposed fix
- **Preserve voice**: Keep the team's phrasing where content is sound
