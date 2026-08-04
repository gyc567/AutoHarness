# CLI 工具项目 GOAL.md 示例

**适用场景**: 命令行工具（如 ripgrep, bat, fd）
**模式**: Converge
**基线**: ~60/100

---

## Fitness Function

```bash
./scripts/score.sh          # 人类可读
./scripts/score.sh --json   # JSON 格式
```

### Metric Definition

```
score = correctness + performance + usability + documentation
```

| Component | Max | What it measures |
|-----------|-----|------------------|
| correctness | 25 | 所有测试通过，无 regression |
| performance | 25 | benchmark 达标 |
| usability | 25 | CLI 帮助信息完整，错误提示清晰 |
| documentation | 25 | README, man page, 示例 |

---

## Bootstrap

```bash
cargo build --release
./scripts/score.sh
```

---

## Action Catalog

### correctness (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 运行完整测试套件 | +25 | `cargo test --all-features` |

### performance (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 运行 benchmark | +10 | `hyperfine 'target/release/app --help'` |
| 优化启动时间 | +10 | 检查 main.rs 初始化代码 |
| 优化热路径 | +5 | `cargo flamegraph` |

### usability (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| 检查 --help 输出 | +10 | `app --help` 必须清晰 |
| 检查错误信息 | +10 | `app invalid-input` 必须友好 |
| 检查 shell completion | +5 | 生成 bash/zsh/fish completion |

### documentation (target: 25/25)

| Action | Impact | How |
|--------|--------|-----|
| README 完整性 | +10 | 包含安装、使用、示例 |
| man page | +10 | `app --help` 能生成 man |
| 示例文件 | +5 | 在 examples/ 目录 |

---

## Constraints

1. **不要改变 CLI 接口** — 除非重大版本升级
2. **性能不能退化** — 新版本必须至少和旧版本一样快
3. **向后兼容** — 现有 flag 和参数必须保持
