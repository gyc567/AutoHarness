# Delegation Rules Template

Use this template for your project's delegation rules.

---

## Coordinator Role

**Who**: This agent (Claude Code or primary agent)

**Responsibilities**:
- Break down complex tasks into executable subtasks
- Delegate subtasks to executors
- Review executor output before shipping
- Handle errors and retries
- Log decisions and outcomes

**Guidelines**:
1. Prompts must be self-contained (executor has no context)
2. Include success criteria in each prompt
3. Set clear timeout expectations
4. Review ALL output before accepting

---

## Executor Role

**Who**: Delegated CLI agent (claude, codex, grok, etc.)

**Responsibilities**:
- Execute assigned task faithfully
- Write results to result file
- Touch done file on completion
- Report errors clearly

**Guidelines**:
1. Work in isolation (no access to coordinator context)
2. Follow instructions exactly
3. Ask for clarification only if truly stuck
4. Produce verifiable output

---

## Communication Protocol

```
Coordinator → Executor: Task via prompt
Executor → Coordinator: Result via file
```

### Prompt Format

```markdown
ROLE: EXECUTOR

## Task
<clear description of what to do>

## Context
<relevant files, existing code, constraints>

## Success Criteria
<what constitutes done>

## Output
<where to write result>
```

### Result Format

```markdown
# Task Result

## Completed
- <list of what was done>

## Files Changed
- file1.md
- file2.rs

## Verification
<how to verify the work>

## Notes
<any issues or next steps>
```

---

## Example Workflow

```bash
# 1. Coordinator breaks down task
# Task: "Refactor auth module"
# Subtasks:
#   - "Extract login logic to auth/login.rs"
#   - "Add session management to auth/session.rs"
#   - "Update tests for new structure"

# 2. Delegate first subtask
./scripts/tdel start auth-refactor --agent claude --prompt "Extract login logic..."

# 3. Wait for completion
./scripts/tdel wait auth-refactor 600

# 4. Review result
cat /tmp/agent-delegate/auth-refactor/result.md

# 5. If good, continue. If not, iterate.
./scripts/tdel send auth-refactor "Please add error handling..."
```

---

## Integration with CLAUDE.md

Reference this file in your project's CLAUDE.md:

```markdown
## Delegation

This project uses the open-agent-teams skill for multi-agent collaboration.

See: skills/utilities/open-agent-teams/references/delegation-template.md
```
