---
name: rtk-token-killer
description: >
  RTK (Rust Token Killer) — CLI proxy that cuts 60-90% of bash output before it reaches
  the LLM context window. Use when the user wants to optimize token usage, reduce CLI output
  noise, see token savings, or says "rtk", "token savings", "reduce bash output".
---

# RTK — Rust Token Killer

RTK intercepts shell command output and compresses it before it reaches the LLM context.
Single Rust binary, 100+ commands supported, <10ms overhead.

## Quick Reference

| Command | What it does |
|---------|-------------|
| `rtk gain` | Token savings dashboard (total / by command / graph) |
| `rtk gain --history` | Recent command history with savings |
| `rtk gain --daily` | Day-by-day breakdown |
| `rtk discover` | Find missed savings opportunities in recent sessions |
| `rtk proxy <cmd>` | Raw passthrough (for debugging) |
| `rtk err <cmd>` | Filter errors only from any command |
| `rtk test <cmd>` | Generic test wrapper — failures only (-90%) |

## Key Commands (auto-rewrite via hook)

```bash
rtk ls .                      # Compact directory tree
rtk read file.rs              # Signatures + structure
rtk git status                # 1-line status
rtk git log -n 10             # Hash/author/subject only
rtk git diff                  # Condensed diff
rtk git push                  # → "ok main"
rtk cargo test                # Failures only (~20 lines vs 200+)
rtk cargo clippy              # Grouped by rule
rtk pytest                    # Failures only
rtk docker ps                 # Essential fields
rtk kubectl pods              # Compact pod list
```

## Auto-Rewrite Hook

After `rtk init -g`, Claude Code Bash tool calls are automatically rewritten:
`git status` → `rtk git status` — transparent, zero context overhead.

> **Note**: Claude Code built-in tools (Read, Grep, Glob) bypass the hook.
> Use shell commands (`cat`, `rg`, `find`) or call `rtk` directly for those.

## Integration with Loop Engineering

- improvement-loop: `rtk test cargo test` in the verify step reduces 200+ lines to ~20
- loop-doctor: `rtk gain` shows token savings across all loop runs
- `rtk discover` surfaces commands loop patterns are not optimizing

## North Star Metric

| Indicator | Target |
|-----------|--------|
| RTK Token Savings Rate | ≥ 50% (from `rtk gain` global stats) |

## Links

- Website: <https://www.rtk-ai.app>
- Repo: <https://github.com/rtk-ai/rtk>
- Install: `brew install rtk` or `curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh`
- Verify: `rtk --version`
