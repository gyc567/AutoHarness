# Goal: Make AutoHarness production-ready with high code quality

AutoHarness is an AI-powered test harness synthesizer. The goal is to achieve production-grade code quality: zero formatting issues, zero clippy warnings, all tests passing, comprehensive documentation, and clean maintenance hygiene. Run the fitness function, find the gap, fix it, repeat.

## Fitness Function

```bash
./scripts/score.sh          # human-readable
./scripts/score.sh --json   # machine-readable
```

### Metric Definition

```
quality_score = format + clippy + tests + docs + maintenance
```

| Component | Max | What it measures |
|-----------|-----|------------------|
| **format** | 20 | `cargo fmt` passes cleanly |
| **clippy** | 20 | Zero clippy warnings |
| **tests** | 25 | All tests passing |
| **docs** | 15 | AGENTS.md, DOCS.md, docs/ populated |
| **maintenance** | 20 | Valid Cargo.toml, .gitignore, Cargo.lock, src structure |

### Metric Mutability

- [x] **Open** — The scoring criteria are part of the work. The agent can propose metric changes but should justify them.

## Operating Mode

- [x] **Converge** — Stop when score reaches 100/100

### Stopping Conditions

Stop and report when ANY of:
- Score reaches 100/100
- 10 consecutive iterations with no improvement
- 20 iterations completed

## Bootstrap

```bash
# 1. Install dependencies
cargo fetch

# 2. Run baseline score
./scripts/score.sh

# 3. Expected baseline: ~65-75 (format issues, clippy warnings present)
```

## Improvement Loop

```
repeat:
  0. Read iterations.jsonl if exists — note what's been tried
  1. ./scripts/score.sh --json > /tmp/before.json
  2. Read score breakdown — find weakest component
  3. Pick highest-impact action from Action Catalog
  4. Make the change
  5. Run targeted verification (cargo fmt, cargo clippy, or cargo test)
  6. ./scripts/score.sh --json > /tmp/after.json
  7. Compare: if improved, commit; if regressed, revert
  8. Append to iterations.jsonl
  9. Continue
```

Commit messages: `[S:NN→NN] component: what changed`

## Iteration Log

File: `iterations.jsonl` (append-only, one JSON object per line)

```jsonl
{"iteration":1,"before":65,"after":70,"action":"Fix format issues","result":"kept","note":"Ran cargo fmt on 12 files"}
{"iteration":2,"before":70,"after":70,"action":"Refactor error handling","result":"reverted","note":"No score change, reverted"}
```

## Action Catalog

### format (target: 20/20) -- 0/20

| Action | Impact | How |
|--------|--------|-----|
| Run cargo fmt on all source files | +20 pts | `cargo fmt -- --edition` then verify with `cargo fmt -- --check` |

### clippy (target: 20/20) -- ~15/20

| Action | Impact | How |
|--------|--------|-----|
| Fix sort_by_key warnings | +2-5 pts | Change `.sort_by(\|a, b\| b.key.cmp(&a.key))` to `.sort_by_key(\|x\| x.key)` |
| Add clippy config to Cargo.toml | +5 pts | Add `[lints.clippy]` with recommended settings from AGENTS.md |
| Suppress false positives with allow attributes | +2 pts | Add `#[allow(clippy::xxx)]` with reason |

### tests (target: 25/25) -- 25/25

Maintain it.

| Action | Impact | How |
|--------|--------|-----|
| Add integration tests for CLI | +2 pts | Test synthesis, evaluate, run commands |
| Add benchmarks for synthesis | +2 pts | `cargo bench` for synthesis workflow |

### docs (target: 15/15) -- 15/15

Maintain it.

### maintenance (target: 20/20) -- 20/20

Maintain it.

## Constraints

1. **Do not break existing functionality** — All tests must pass before any commit
2. **No new production dependencies** — Only dev dependencies allowed
3. **Preserve AGENTS.md** — Agent guidelines must remain intact
4. **Format first, lint second** — Always run `cargo fmt` before `cargo clippy`
5. **One logical change per commit** — Atomic commits for bisectability

## File Map

| File | Role | Editable? |
|------|------|-----------|
| `scripts/score.sh` | Fitness function | Yes |
| `AGENTS.md` | Agent guidelines | Yes |
| `DOCS.md` | Documentation index | Yes |
| `docs/` | Development docs | Yes |
| `src/**/*.rs` | Source code | Yes |
| `Cargo.toml` | Project config | Yes |
| `iterations.jsonl` | Iteration log | Append only |

## When to Stop

```
Starting score: NN
Ending score:   NN
Iterations:     N
Changes made:   (list)
Remaining gaps: (list)
Next actions:   (what to do next)
```
