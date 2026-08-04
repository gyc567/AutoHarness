# Codebase Harness Guide

## Overview

A **codebase harness** is the infrastructure that makes a repository ready for AI agents to work in. Think of it as the "user manual" and "toolkit" that enables reliable agent operations.

## The Four Pillars

### 1. Legible (可读)

The agent must be able to understand the codebase:

- **AGENTS.md / CLAUDE.md**: Primary agent guide
- **README.md**: Human-oriented overview
- **docs/**: Detailed documentation
- **Directory structure**: Logical organization

### 2. Executable (可执行)

The agent must be able to run the code:

- **One-command build**: `cargo build` or equivalent
- **One-command test**: `cargo test` or equivalent
- **Dev environment**: `scripts/dev-local.sh`

### 3. Verifiable (可验证)

The agent must be able to verify its changes:

- **Smoke tests**: Quick sanity checks
- **Unit tests**: Comprehensive coverage
- **E2E tests**: Full workflow validation

### 4. Maintainable (可维护)

The codebase must stay clean:

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Commit conventions**: Structured messages

## Quick Start Template

```bash
# Assessment
ls -la
cat README.md
ls scripts/

# Basic setup
touch AGENTS.md
mkdir -p scripts

# Verify
cargo build && cargo test
```

## Common Patterns

### Rust Projects

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo fmt && cargo clippy

# All
cargo build && cargo test && cargo fmt && cargo clippy
```

### Node.js Projects

```bash
# Build
npm run build

# Test
npm test

# Lint
npm run lint

# Dev
npm run dev
```

## Anti-Patterns

❌ **Don't**: Complex multi-step setup procedures

❌ **Don't**: Hidden dependencies not in version control

❌ **Don't**: Manual steps that should be automated

## Resources

- [GOAL.md Integration](./docs/goal-md/skills-integration/)
- [AI Builder Club Skills](https://github.com/AI-Builder-Club/skills)
