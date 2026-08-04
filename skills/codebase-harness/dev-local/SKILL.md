---
name: dev-local
description: >
  Set up a one-command local development environment. Creates scripts/dev-local.sh
  that can start, stop, and manage the dev stack. Use when setting up the harness
  or when the user says "set up dev-local", "one-command dev", or "start the dev stack".
user_invocable: true
---

# Dev Local Setup

Create a one-command development environment script.

## When to use

- "set up dev-local"
- "one-command dev"
- "start the dev stack"
- "setup development environment"

## Steps

### 1. Assess the project

Identify what services/processes are needed:
- Database services
- API servers
- Background workers
- External dependencies

### 2. Create the script

Create `scripts/dev-local.sh`:

```bash
#!/bin/bash
# Dev Local Setup Script

set -e

COMMAND="${1:-up}"

case "$COMMAND" in
    up)
        echo "Starting dev environment..."
        # Add your startup commands here
        ;;
    down)
        echo "Stopping dev environment..."
        # Add your shutdown commands here
        ;;
    status)
        echo "Dev environment status"
        # Add your status commands here
        ;;
    *)
        echo "Usage: $0 {up|down|status}"
        exit 1
        ;;
esac
```

### 3. Make it executable

```bash
chmod +x scripts/dev-local.sh
```

### 4. Document usage

Add usage instructions to the script comments.

## Usage

```bash
# Start the dev environment
./scripts/dev-local.sh up

# Stop the dev environment
./scripts/dev-local.sh down

# Check status
./scripts/dev-local.sh status

# View logs
./scripts/dev-local.sh logs <service>
```

## For Rust Projects

```bash
#!/bin/bash
# Rust Dev Local Script

set -e

COMMAND="${1:-up}"

case "$COMMAND" in
    up)
        echo "Starting Rust dev environment..."
        cargo build
        echo "Ready! Run 'cargo run' to start."
        ;;
    test)
        echo "Running tests..."
        cargo test
        ;;
    lint)
        echo "Running lints..."
        cargo fmt && cargo clippy
        ;;
    *)
        echo "Usage: $0 {up|test|lint}"
        exit 1
        ;;
esac
```

## Principles

- **Idempotent**: Running up twice should be safe
- **Clear output**: Tell the user what's happening
- **Fail fast**: Exit early on errors
- **Document dependencies**: What needs to be installed first
