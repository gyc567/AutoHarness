# Goal: [Project Name] - [One-line goal statement]

## Fitness Function

```bash
./scripts/score.sh
```

## Operating Mode

- [ ] **Converge** — Stop when goal is reached
- [ ] **Continuous** — Keep improving forever
- [x] **Supervised** — Stop and ask for confirmation

### Stop Conditions

Stop when:
- Score reaches **100/100**, OR
- **10 iterations** with no improvement, OR
- Human interrupts

## Action Catalog

| Action | Impact | How | When |
|--------|--------|-----|------|
| Format code | +20 | `cargo fmt` | Always first |
| Fix clippy warnings | +20 | `cargo clippy --fix` | After format |
| Add tests | +15 | `cargo test` | When < 100% |
| Improve docs | +10 | Add to `docs/` | Weekly |
| CI/CD setup | +10 | Add `.github/workflows/` | Once |

## Current State

**Score**: _/100 (run `./scripts/score.sh` to measure)_

| Component | Score | Target |
|-----------|-------|--------|
| format | _/20 | 20 |
| lint | _/20 | 20 |
| tests | _/25 | 25 |
| docs | _/15 | 15 |
| maintenance | _/20 | 20 |

## Constraints

1. **Never reduce the score** — If a change causes regression, revert it
2. **One change per commit** — Atomic commits for easy rollback
3. **Verify before commit** — Run score script after every change
4. **Log all iterations** — Append to `iterations.jsonl`

## Iteration Log

File: `iterations.jsonl`

Format:
```json
{"iteration":N,"timestamp":"ISO8601","component":"name","before":N,"after":N,"action":"description","result":"kept|reverted"}
```