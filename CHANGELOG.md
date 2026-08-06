# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security
- **[P0]** Shell injection defense-in-depth: `validate_code()` now blocks all shell metacharacters (`& ; | > < ( )`), backtick command substitution, unquoted `$`, and `eval`/`exec` patterns. Gate added at `execute_with_input()` entry.
- **[P0]** `shell_escape()` expanded from 5 to 19 characters (complete POSIX coverage).
- **[P1]** `working_directory` validation: rejects relative paths, nonexistent dirs, and path traversal (`..`).
- **[P1]** `use_seccomp`/`use_cgroups`/`use_namespaces` defaults changed from `true` to `false` — no longer misleading when features are unimplemented.

### Bug Fixes
- **[P0]** `chrono_like_now()` now outputs real ISO 8601 UTC (`2026-08-06T...`) instead of `ts-{unix_secs}` — fixes `loop-run-log.jsonl` schema.
- **[P0]** `generate_run_id()` now outputs `YYYYMMDDTHHMMSSZ-NNN` format instead of `run-{secs}-{n}`.

### Maintenance
- **[P2]** Removed 3 unused dependencies: `duct`, `notify`, `metrics`.
- **[P2]** README badge updated to 100/100 (was 88/100 stale).
- **[P1]** `gate.yaml` denylist extended with 8 missing critical paths.

## [0.1.0] - 2026-08-06

### Added
- AutoHarness: automatically synthesize code harnesses for LLM agents
- 7 harness templates: adaptive, critic, ensemble, filter, policy, refiner, verifier
- Sandbox executor with resource limits (time, memory, output size)
- Loop engineering methodology layer (Phase 1 + Phase 2)
  - Patterns: improvement-loop, synthesis-quality, test-coverage, doc-staleness, clippy-fmt-watch, release-drafter, dependency-watch, daily-triage
  - Loop Ready: 100/100 (R3)
  - Gate system, budget tracking, constraints management
- Comprehensive test suite (29 sandbox tests + integration tests)
- CLI with `synthesize`, `run`, `loop`, `gate`, `audit` commands

[unreleased]: https://github.com/cobusgreyling/AutoHarness
[0.1.0]: https://github.com/cobusgreyling/AutoHarness/releases/tag/v0.1.0
