# AutoHarness AGENTS.md

**Guidelines for AI agents operating in this repository**

---

## 1. Build, Test & Development Commands

### Core Commands

```bash
# Build the project
cargo build

# Build with all features
cargo build --all-features

# Release build
cargo build --release

# Run the CLI
cargo run -- --help

# Run a specific CLI command
cargo run -- synthesize --code "fn test() {}"
```

### Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_state_serialization
cargo test test_action_parse_error
cargo test test_synthesis_convergence_early

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Run with coverage (requires tarpaulin)
cargo tarpaulin --output --tests
```

### Linting & Formatting

```bash
# Format code
cargo fmt

# Check formatting (without making changes)
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all warnings as errors
cargo clippy -- -D warnings

# Run clippy for workspace
cargo clippy --workspace -- -D warnings
```

### Documentation

```bash
# Build documentation
cargo doc

# Build documentation (open in browser)
cargo doc --open

# Build documentation without dependencies
cargo doc --no-deps
```

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench -- synthesis
```

---

## 2. Code Style Guidelines

### General Principles

- **Rust Edition**: 2021
- **Maximum line length**: 100 characters
- **Indentation**: 4 spaces (not tabs)
- **Trailing commas**: Always use trailing commas in multi-line contexts

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Variables | snake_case | `let max_iterations = 50;` |
| Functions | snake_case | `fn synthesize_code()` |
| Structs | PascalCase | `struct CodeSynthesisEngine` |
| Enums | PascalCase | `enum HarnessType` |
| Enum Variants | PascalCase | `Filter, Verifier, Policy` |
| Constants | SCREAMING_SNAKE_CASE | `const MAX_DEPTH: u32 = 10;` |
| Modules | snake_case | `mod engine;` |
| Traits | PascalCase | `trait State` |
| Types | PascalCase | `type Result<T> = ...` |

### Import Organization

Organize imports in this order with blank lines between groups:

```rust
// 1. Standard library
use std::collections::HashMap;
use std::path::PathBuf;

// 2. External crates
use serde::{Deserialize, Serialize};
use tokio::fs;

// 3. Internal crate modules
use autoharness::core::{Action, State};
use autoharness::engine::SynthesisConfig;

// 4. Module-local
use super::error::Result;
```

### Error Handling

- Use `thiserror` for defining error enums
- Use the `HarnessError` type from `crate::core::error`
- Always return `Result<T>` for fallible operations
- Never use `unwrap()` in production code
- Use `?` operator for error propagation
- Provide meaningful error messages

```rust
// Good
pub fn evaluate(&self, code: &str) -> Result<f64> {
    if code.is_empty() {
        return Err(HarnessError::evaluation("Code cannot be empty"));
    }
    // ...
}

// Bad - don't do this
pub fn evaluate(&self, code: &str) -> f64 {
    code.len() as f64 // No error handling!
}
```

### Type Annotations

- Always annotate function return types
- Use explicit types for public APIs
- Prefer type inference for local variables when obvious

```rust
// Good
pub fn new() -> Self {
    let config = SynthesisConfig::default();
    Self { config }
}

// Good - obvious inference
let items = vec![1, 2, 3]; // Vec<i32> is obvious
```

### Documentation

- Document all public APIs with doc comments (`///`)
- Include examples in doc comments
- Use meaningful descriptions, not just "Creates a new instance"

```rust
/// Creates a new synthesis engine with the given configuration.
///
/// # Arguments
///
/// * `config` - The synthesis configuration
///
/// # Returns
///
/// A new `CodeSynthesisEngine` instance
///
/// # Example
///
/// ```
/// let config = SynthesisConfig::new();
/// let engine = CodeSynthesisEngine::new(config);
/// ```
pub fn new(config: SynthesisConfig) -> Self {
    // ...
}
```

### Testing

- All public functions should have tests
- Follow naming convention: `#[test] fn test_<functionality>_<expected_behavior>()`
- Use descriptive test names
- Test both success and failure cases

```rust
#[test]
fn test_synthesis_config_builder() {
    let config = SynthesisConfig::new()
        .with_max_iterations(100)
        .with_convergence_threshold(0.99);
    
    assert_eq!(config.max_iterations, 100);
    assert_eq!(config.convergence_threshold, 0.99);
}
```

### Module Structure

- One public trait/type per module minimum
- Use `pub mod` for module visibility
- Group related functionality together
- Keep modules focused and cohesive

---

## 3. Development Principles (Mandatory)

**All code fixes and new features MUST follow these principles:**

### 1. Design Principles
- **KISS Principle**: Keep code simple and clean
- **High Cohesion, Low Coupling**: Use lean design patterns
- **Single Responsibility**: Each module does one thing

### 2. Testing Requirements
- **Coverage**: All new feature code must be tested with **100% test coverage**
- **TDD Method**: Write tests first, then implementation
- **Regression Testing**: Verify fixes don't break other features

### 3. Change Principles
- **Isolation**: Changes must not affect unrelated features
- **Minimization**: Only change necessary code, no large refactors

### 4. Proposal Process
- **New Features**: Use **openspec** to generate feature proposals
- **Bug Fixes**: Use **openspec** to generate fix proposals
- **Any Code Change**: Must include these requirements

---

## 4. Code Quality Checklist

Before completing a task, verify:

- [ ] Code is readable with clear naming
- [ ] Functions are concise (< 50 lines)
- [ ] Files are focused (< 800 lines)
- [ ] No deep nesting (> 4 levels)
- [ ] Error handling is complete
- [ ] No println!/print! statements in production
- [ ] No hardcoded values
- [ ] Uses immutable patterns where possible

---

## 5. Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Test all | `cargo test` |
| Test single | `cargo test test_name` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |
| Docs | `cargo doc --open` |
| Run CLI | `cargo run -- <args>` |

---

**Last Updated**: 2026-03-29
**Version**: 0.1.0
