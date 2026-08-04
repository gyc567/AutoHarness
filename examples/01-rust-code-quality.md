# Rust 项目代码质量 GOAL.md 示例

**适用场景**: Rust CLI 工具、库、wasm 项目
**模式**: Converge
**基线**: ~70/100

---

## Fitness Function

```bash
./scripts/score.sh          # 人类可读
./scripts/score.sh --json   # JSON 格式
```

### Metric Definition

```
score = format + clippy + tests + docs + safety
```

| Component | Max | What it measures | How to verify |
|-----------|-----|------------------|----------------|
| format | 20 | `cargo fmt` 通过 | `cargo fmt -- --check` |
| clippy | 20 | 无 clippy 警告 | `cargo clippy` |
| tests | 25 | 所有测试通过 | `cargo test` |
| docs | 15 | 文档完整 | 检查 README、src doc |
| safety | 20 | unsafe 代码安全 | 审查 unsafe 块 |

---

## Bootstrap

```bash
cargo fetch
./scripts/score.sh
# 记录基线分数
```

---

## Action Catalog

### format (target: 20/20) -- 通常 0/20

| Action | Impact | How |
|--------|--------|-----|
| 运行 cargo fmt | +20 | `cargo fmt` |

### clippy (target: 20/20) -- 通常 15/20

| Action | Impact | How |
|--------|--------|-----|
| 修复 sort_by_key 警告 | +2-5 | `sort_by(\|a,b\| b.key.cmp(&a.key))` → `sort_by_key(\|x\| x.key)` |
| 修复 map_or_else 警告 | +2-3 | 相应替换 |
| 添加 clippy 配置 | +5 | 在 Cargo.toml 添加 `[lints.clippy]` |
| 抑制误报 | +2 | `#[allow(clippy::xxx)]` |

### tests (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 运行完整测试套件 | +25 | `cargo test` 必须 100% 通过 |
| 添加缺失测试 | +2-5 | 覆盖未测试的模块 |

### docs (target: 15/15)

| Action | Impact | How |
|--------|--------|-----|
| 检查 README 完整性 | +5 | 包含安装、使用、示例 |
| 添加 src doc | +5 | 为公共 API 添加 `///` 注释 |
| 检查文档链接 | +5 | 确认链接有效 |

### safety (target: 20/20)

| Action | Impact | How |
|--------|--------|-----|
| 审查 unsafe 代码 | +10 | 列出所有 unsafe 块 |
| 添加安全注释 | +10 | 为每个 unsafe 添加 `// SAFETY: ...` |

---

## Constraints

1. **不要破坏现有功能** — 所有测试必须通过
2. **不要添加生产依赖** — 只允许 dev dependencies
3. **先格式后 lint** — 总是 `cargo fmt` 在 `cargo clippy` 之前
4. **一个提交一个改动** — 原子提交便于 bisect

---

## 迭代日志示例

```jsonl
{"iteration":1,"timestamp":"2026-08-04T10:00:00Z","component":"format","before":0,"after":20,"action":"Run cargo fmt","result":"kept","note":"Applied rustfmt to all files","commit":"abc1234"}
{"iteration":2,"timestamp":"2026-08-04T10:15:00Z","component":"clippy","before":15,"after":20,"action":"Fix sort_by_key warnings","result":"kept","note":"3 warnings fixed","commit":"def5678"}
```

---

## 完整文件结构

```
my-rust-project/
├── GOAL.md
├── scripts/
│   └── score.sh          # 评分脚本
├── iterations.jsonl      # 迭代日志
├── src/
│   └── lib.rs
├── Cargo.toml
└── README.md
```
