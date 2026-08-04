---
name: verify
description: >
  Verify that code works before shipping. Run smoke tests, build verification,
  and regression checks. Use when the user says "verify this", "check if it works",
  or before opening a PR.
user_invocable: true
---

# Verify Before Ship

Verify that code changes work correctly before shipping.

## When to use

- "verify this"
- "check if it works"
- "run verification"
- "verify before PR"
- "test my changes"

## Prerequisites

- Changes have been made and committed
- On a feature branch (not main)

## Verification Steps

### 1. Build Verification

```bash
cargo build
```

### 2. Lint Check

```bash
cargo fmt -- --check
cargo clippy
```

### 3. Test Suite

```bash
cargo test
```

### 4. Integration Test (if available)

```bash
./scripts/test-integration.sh
```

### 5. Manual Smoke Test

For CLI tools:
```bash
cargo run -- --help
```

For libraries:
```bash
cargo test --doc
```

## Success Criteria

All of the following must pass:
- [ ] `cargo build` succeeds
- [ ] `cargo fmt -- --check` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo test` all pass

## Failure Handling

If verification fails:
1. Review the error messages
2. Fix the issues
3. Re-run verification
4. Do not open PR until all checks pass

## Principles

- **Fail fast**: Stop on first failure
- **Be thorough**: Don't skip lint or tests
- **Document results**: Log verification outcomes

## Integration with AutoHarness

AutoHarness can automatically generate verification tests. Use:

```bash
# Generate tests for a function
autoharness synthesize --code "fn my_function() { ... }"
```
