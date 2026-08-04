# Verify Before Ship Template

Use this template to create a `/verify` skill for your project.

## Project-Specific Verification

Copy this to your project root as `skills/verify/SKILL.md` and customize.

---
name: verify
description: >
  Verify [PROJECT_NAME] code changes before shipping.
user_invocable: true
---

# Verify Before Ship - [PROJECT_NAME]

## Project-Specific Commands

### Build

```bash
# Add your build command
cargo build
```

### Test

```bash
# Add your test command
cargo test
```

### Lint

```bash
# Add your lint commands
cargo fmt -- --check
cargo clippy
```

## Verification Checklist

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code is formatted correctly
- [ ] No clippy warnings
- [ ] Documentation updated (if needed)

## Customization Guide

1. Replace `[PROJECT_NAME]` with your project name
2. Update build/test/lint commands
3. Add project-specific checks
4. Document any special requirements
