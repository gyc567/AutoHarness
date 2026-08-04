---
name: setup-harness
description: >
  Master skill — set up the full agent harness for any repo so an agent can work
  it reliably: legible (map + docs + custom lints), executable (one-command dev
  stack), verifiable (verify-before-ship loop). Use when onboarding a new codebase
  to agent-driven development.
user_invocable: true
---

# Set up the Codebase Harness

Make any codebase agent-ready. This skill sets up the four pillars:
1. **Legible** — Agent can understand the codebase
2. **Executable** — Agent can run the code
3. **Verifiable** — Agent can verify the code works
4. **Maintainable** — Codebase stays clean

## When to use

- "set up the harness"
- "make this repo agent-ready"
- "harness this codebase"
- "setup the codebase for AI agents"

## Steps

### 1. Assess the codebase

Survey the project:
- Language and package manager (cargo, npm, pip, etc.)
- Build system and commands
- Existing tests and how to run them
- Documentation structure
- CI/CD setup

### 2. Make it Legible

Create essential documentation:

```bash
# Create AGENTS.md / CLAUDE.md
touch AGENTS.md
```

Add to AGENTS.md:
- Project overview
- Key commands (build, test, lint)
- Directory structure
- Important files

### 3. Make it Executable

Create a dev-local script:

```bash
mkdir -p scripts
touch scripts/dev-local.sh
chmod +x scripts/dev-local.sh
```

### 4. Make it Verifiable

Create a verify workflow:
- Define what "works" means for this project
- Add smoke tests
- Document verification commands

### 5. Make it Maintainable

Add commit conventions:
- Commit message format
- Branch naming convention
- PR requirements

## Output

- `AGENTS.md` or `CLAUDE.md` — Agent guide
- `scripts/dev-local.sh` — Dev environment script
- `CONTRIBUTING.md` — Contribution guidelines (optional)

## Principles

- **Start simple**: Basic harness first, enhance later
- **Document conventions**: Not just what, but why
- **Automate the basics**: build, test, lint should be one-command
