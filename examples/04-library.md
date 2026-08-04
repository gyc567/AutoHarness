# Rust 库项目 GOAL.md 示例

**适用场景**: 发布到 crates.io 的 Rust 库
**模式**: Converge
**基线**: ~55/100

---

## Fitness Function

```bash
./scripts/score.sh
./scripts/check-crate.sh   # crates.io 发布检查
```

### Metric Definition

```
score = api_design + documentation + compatibility + quality
```

| Component | Max | What it measures |
|-----------|-----|------------------|
| api_design | 25 | API 符合 Rust API Guidelines |
| documentation | 25 | crates.io 描述、README、doc 示例 |
| compatibility | 25 | MSRV、旧版 Rust 支持 |
| quality | 25 | 零警告、零 unsafe、未发布特性 |

---

## Bootstrap

```bash
cargo publish --dry-run
./scripts/score.sh
```

---

## Action Catalog

### api_design (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 检查 API Guidelines | +10 | 对照 rust-lang/api-guidelines |
| 命名规范 | +10 | `snake_case`, `PascalCase` 正确 |
| 错误处理 | +5 | 使用 `thiserror` 或 `anyhow` |

### documentation (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| crates.io 描述 | +10 | 清晰描述功能和用途 |
| README 完整性 | +10 | 包含示例、依赖方式 |
| doc 示例 | +5 | `cargo doc --document-private-items` |

### compatibility (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 设置 MSRV | +10 | 在 Cargo.toml 指定 `rust-version` |
| 测试旧版 Rust | +10 | CI 中测试 MSRV 版本 |
| 避免未稳定特性 | +5 | 检查 `Cargo.lock` |

### quality (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 零 clippy 警告 | +10 | `cargo clippy --all-targets` |
| 无 unsafe | +10 | 除非必要，否则避免 |
| 无未发布特性 | +5 | 检查 Cargo.toml |

---

## Crates.io 检查清单

```bash
# 发布前检查
cargo publish --dry-run
cargo doc --no-deps
cargo fmt -- --check
cargo clippy --all-targets
cargo audit
```

---

## Constraints

1. **不要破坏 semver** — 公共 API 变更必须 major 版本升级
2. **保持 MSRV 稳定** — 一旦发布，MSRV 只能增加不能减少
3. **文档必须自包含** — 用户只需要 README 就能使用
