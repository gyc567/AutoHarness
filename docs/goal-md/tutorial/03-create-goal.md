# 创建你的第一个 GOAL.md

**完整示例：从零开始为一个项目添加 GOAL.md**

---

## 场景

假设你有一个小型 Rust CLI 工具：

```
my-cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
└── README.md
```

当前状态：
- 代码格式混乱
- 有一些 clippy 警告
- 缺少测试
- README 很简单

---

## Step 1: 分析当前状态

```bash
cd my-cli

# 检查格式
cargo fmt -- --check
# error: Binary file src/main.rs matches

# 检查 clippy
cargo clippy 2>&1 | grep "warning:"
# warning: unused import
# warning: unused variable

# 检查测试
cargo test
# running 0 tests
```

---

## Step 2: 创建评分脚本

创建 `scripts/score.sh`：

```bash
#!/bin/bash
set -uo pipefail

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0; DOC_SCORE=0
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Format Check (20分)
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# Clippy Check (20分)
WARN_COUNT=$(cargo clippy 2>&1 | grep -c "warning:" || true)
case "$WARN_COUNT" in
    0) CLIPPY_SCORE=20 ;;
    [1-5]) CLIPPY_SCORE=15 ;;
    [6-10]) CLIPPY_SCORE=10 ;;
    *) CLIPPY_SCORE=5 ;;
esac

# Test Coverage (20分)
TEST_OUTPUT=$(cargo test 2>&1 || true)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep -oP '\d+ passed' | grep -oP '\d+' | tail -1 || echo "0")
    [[ "$PASSED" -gt 0 ]] && TEST_SCORE=20
fi

# Documentation (10分)
[[ -f "README.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -f "AGENTS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))

TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE))

if [[ "$JSON_OUTPUT" == "true" ]]; then
    echo "{\"timestamp\":\"$TIMESTAMP\",\"total\":$TOTAL,\"max\":70}"
else
    echo "Score: $TOTAL / 70"
    echo "  format: $FORMAT_SCORE/20, clippy: $CLIPPY_SCORE/20, tests: $TEST_SCORE/20, docs: $DOC_SCORE/10"
fi
```

运行测试：

```bash
chmod +x scripts/score.sh
./scripts/score.sh
# Score: 5 / 70
#   format: 0/20, clippy: 0/20, tests: 0/20, docs: 5/10
```

---

## Step 3: 创建 GOAL.md

创建 `GOAL.md`：

```markdown
# Goal: my-cli - 提升代码质量到 70/70

## Fitness Function

```bash
./scripts/score.sh          # 人类可读
./scripts/score.sh --json   # JSON 格式
```

### Metric Definition

```
score = format(20) + clippy(20) + tests(20) + docs(10)
```

| Component | Max | What it measures | How to verify |
|-----------|-----|------------------|----------------|
| format | 20 | 代码格式 | `cargo fmt -- --check` |
| clippy | 20 | Lint 警告 | `cargo clippy` |
| tests | 20 | 测试通过 | `cargo test` |
| docs | 10 | 文档完整 | README, AGENTS.md |

### Metric Mutability

- [x] **Locked** — Agent 不能修改评分脚本

## Operating Mode

- [x] **Converge** — 达到目标时停止

Stop when:
- Score reaches 70/70
- 10 consecutive iterations with no improvement

## Bootstrap

```bash
# 基线分数
./scripts/score.sh
# Baseline: 5/70
```

## Improvement Loop

```
repeat:
  1. ./scripts/score.sh --json > /tmp/before.json
  2. Read scores — find weakest component
  3. Pick highest-impact action from Action Catalog
  4. Make the change
  5. ./scripts/score.sh --json > /tmp/after.json
  6. Compare: if improved, commit; if regressed, revert
  7. Append to iterations.jsonl
  8. Continue
```

## Iteration Log

File: `iterations.jsonl`

## Action Catalog

### format (target: 20/20) -- 当前: 0/20

| Action | Impact | How |
|--------|--------|-----|
| Run cargo fmt | +20 | `cargo fmt` |

### clippy (target: 20/20) -- 当前: 0/20

| Action | Impact | How |
|--------|--------|-----|
| Run cargo clippy --fix | +10-20 | `cargo clippy --fix --allow-dirty` |
| Remove unused imports | +5 | 手动审查并修复 |

### tests (target: 20/20) -- 当前: 0/20

| Action | Impact | How |
|--------|--------|-----|
| Add basic unit tests | +10 | 为每个公共函数添加测试 |
| Add integration test | +10 | 测试 CLI 端到端 |

### docs (target: 10/10) -- 当前: 5/10

| Action | Impact | How |
|--------|--------|-----|
| Create AGENTS.md | +5 | 添加 Agent 使用指南 |

## Constraints

1. **不要破坏现有功能** — 测试必须通过
2. **先格式后 lint** — 总是 `cargo fmt` 在 `cargo clippy` 之前
3. **一个提交一个改动** — 原子提交便于回滚

## File Map

| File | Role | Editable? |
|------|------|-----------|
| scripts/score.sh | Fitness Function | No (Locked) |
| GOAL.md | Goal Definition | Yes |
| iterations.jsonl | Iteration Log | No (append only) |
| src/*.rs | Source Code | Yes |
| Cargo.toml | Dependencies | Yes |

## When to Stop

```
Starting score: 5/70
Ending score:   70/70
Iterations:     6
Changes made:   fmt, clippy--fix, add 5 tests, create AGENTS.md
Remaining gaps: None
Next actions:   (None - goal achieved!)
```
```

---

## Step 4: 运行第一次改进

```bash
# 记录基线
./scripts/score.sh > baseline.txt
echo '{"iteration":0,"score":5,"timestamp":"2026-08-04T12:00:00Z"}' > iterations.jsonl

# 执行第一个改进：格式
cargo fmt
./scripts/score.sh
# Score: 25 / 70
#   format: 20/20 ✓

# 记录
echo '{"iteration":1,"component":"format","before":5,"after":25,"action":"cargo fmt","result":"kept"}' >> iterations.jsonl
```

---

## Step 5: 继续改进

```bash
# 执行 clippy 修复
cargo clippy --fix --allow-dirty
cargo fmt  # 修复后重新格式化

./scripts/score.sh
# Score: 45 / 70
#   format: 20/20 ✓
#   clippy: 20/20 ✓

echo '{"iteration":2,"component":"clippy","before":25,"after":45,"action":"cargo clippy --fix","result":"kept"}' >> iterations.jsonl

# 添加测试
# ... 添加测试代码 ...
cargo test
./scripts/score.sh
# Score: 65 / 70
```

---

## Step 6: 完成

```bash
# 添加 AGENTS.md
cat > AGENTS.md << 'EOF'
# AGENTS.md

本项目的 AI Agent 指南。

## Quick Start

```bash
cargo build
cargo test
./scripts/score.sh
```

## Goals

使用 GOAL.md 持续改进代码质量。
EOF

./scripts/score.sh
# Score: 70 / 70 ✓
```

---

## 最终状态

```
my-cli/
├── GOAL.md           # 目标定义
├── CLAUDE.md         # Agent 指南（可选）
├── AGENTS.md         # Agent 指南
├── iterations.jsonl  # 迭代日志
├── scripts/
│   └── score.sh      # 评分脚本
├── src/
│   └── ...
└── Cargo.toml
```

---

## 下一步

- [评分脚本编写指南](04-scoring-guide.md)
- [常见问题解决](06-troubleshooting.md)
