---
name: open-agent-teams
description: >
  Delegate tasks to ANY CLI agent (claude, codex, aider, grok, ...) running in a detached
  tmux session, with a race-safe done-signal protocol and multi-turn iteration. Use when
  delegating work to another CLI agent, or when executor work should run in an observable
  background terminal.
user_invocable: true
---

# Open Agent Teams

Delegate tasks to other CLI agents running in detached tmux sessions.

## When to use

- "delegate to codex"
- "run agent in tmux"
- "open-agent-teams"
- "run this in background"
- "parallel agent execution"

## Prerequisites

- `tmux` installed
- CLI agents available on PATH (claude, codex, grok, aider, pi, opencode, etc.)
- `../setup-harness` completed for delegation rules

## Key Concepts

### Coordinator vs Executor

- **Coordinator**: This agent (Claude Code) that delegates work
- **Executor**: The delegated agent running in tmux

### Communication Protocol

```
Coordinator → Executor: prompt via CLI arg / send-keys
Executor → Coordinator: completion via file sentinel
```

### Why File Sentinels?

File sentinels are used instead of tmux wait-for because:
- `wait-for -S` with no waiter is silently lost
- Files never race and allow timeouts

## Initial Setup

### 1. Install tdel helper

The `tdel` script manages agent sessions:

```bash
# Copy tdel script
cp scripts/tdel-template.sh scripts/tdel
chmod +x scripts/tdel
```

### 2. Create delegation rules

Create `DELEGATION_RULES.md`:

```markdown
# Delegation Rules

## Coordinator
- This agent (Claude Code) coordinates and delegates
- Reviews executor output before shipping

## Executor
- Receives clear, self-contained prompts
- Writes results to result file
- Touches done file on completion
```

### 3. Verify tmux

```bash
tmux -V  # Should show version
```

## Usage

### Start an Executor Agent

```bash
# Start claude as executor
./scripts/tdel start claude-session --agent claude --prompt "Fix the bug in src/main.rs"

# Start codex as executor
./scripts/tdel start codex-session --agent codex --prompt "Add tests for the API"
```

### Wait for Completion

```bash
# Wait with timeout (in background task)
./scripts/tdel wait claude-session 300  # 5 min timeout

# Exit 0 = done (check result file)
# Exit 124 = timeout
# Exit non-zero = session died
```

### Send Feedback / Iterate

```bash
# Send next instruction
./scripts/tdel send claude-session "Now add error handling"

# Wait for next completion
./scripts/tdel wait claude-session 300
```

### Debug / Monitor

```bash
# View live pane
./scripts/tdel peek claude-session 50

# List all sessions
./scripts/tdel status
```

### Stop / Cleanup

```bash
# Stop session
./scripts/tdel stop claude-session
```

## Agent Reference

| Agent | Command | Interrupt | Notes |
|-------|---------|-----------|-------|
| claude | `claude --dangerously-skip-permissions` | `esc` | Model via `--model` |
| codex | `codex --dangerously-bypass-approvals` | `/quit` | Resume via `codex resume` |
| grok | `grok --always-approve` | `Ctrl+C` | Model via `--model` |
| pi | `pi` | `/quit` | Always autonomous |
| aider | `aider` | `Ctrl+D` | Chat-based |

## Best Practices

### ✅ Do: Self-contained prompts

```bash
# BAD: Assumes executor has context
"Fix the bug"

# GOOD: Full context
"""
You are working on the auth module in src/auth.rs.

Task: Fix the race condition in the login function.

Requirements:
- Use mutex for thread safety
- Add tests
- Follow existing code style

Expected output: A working fix with tests.
"""
```

### ✅ Do: One task per session

```bash
# GOOD: Parallel sessions for parallel tasks
./scripts/tdel start task-1 --agent claude --prompt "Task 1"
./scripts/tdel start task-2 --agent claude --prompt "Task 2"
```

### ❌ Don't: Trust pane output as deliverable

```bash
# BAD: Trust capture-pane
tmux capture-pane -t session

# GOOD: Use result file + changed files
cat /tmp/agent-delegate/claude-session/result.md
```

### ❌ Don't: Assume executor has context

Prompts are flattened to one line before sending. Put large specs in a file:

```bash
# GOOD: Reference file path
"""
Task: Review the security implementation.

See full spec: ./docs/security-review.md

Output: A security audit report.
"""
```

## Integration with AutoHarness

```bash
# Delegate test generation to another agent
./scripts/tdel start test-agent --agent claude \
  --prompt "Generate tests for src/synthesis.rs using AutoHarness"

# Wait for result
./scripts/tdel wait test-agent 600

# Review and integrate
cat /tmp/agent-delegate/test-agent/result.md
```

## Timeout Handling

On timeout:
1. `peek` first — agent may have finished but forgot to touch done file
2. If visibly done, treat pane output + changed files as result
3. Send reminder to executor

## Output

- `scripts/tdel` - Session management helper
- `DELEGATION_RULES.md` - Coordinator/executor rules
- Session state in `/tmp/agent-delegate/<session-name>/`
