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

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0; DOC_SCORE=0

# Format Check (20分)
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# Clippy Check (20分)
WARN_COUNT=$(cargo clippy 2>&1 | grep -c "warning:" || true)
case "$WARN_COUNT" in
    0) CLIPPY_SCORE=20 ;;
    [1-5]) CLIPPY_SCORE=15 ;;
    *) CLIPPY_SCORE=10 ;;
esac

# Test Coverage (20分)
cargo test 2>&1 | grep -q "test result: ok" && TEST_SCORE=20

# Documentation (10分)
[[ -f "README.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -f "AGENTS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))

TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE))

echo "Score: $TOTAL / 70"
```

运行测试：

```bash
chmod +x scripts/score.sh
./scripts/score.sh
# Score: 5 / 70
```

---

## Step 3: 创建 GOAL.md

创建 `GOAL.md`：

```markdown
# Goal: my-cli - 提升代码质量到 70/70

## Fitness Function

./scripts/score.sh

## Operating Mode

- [x] **Converge** — 达到目标时停止

Stop when:
- Score reaches 70/70
- 10 次迭代无改进

## Action Catalog

### format (目标: 20/20)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 运行 cargo fmt | +20 | `cargo fmt` |

### clippy (目标: 20/20)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 运行 cargo clippy --fix | +10-20 | `cargo clippy --fix --allow-dirty` |

### tests (目标: 20/20)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 添加基本单元测试 | +10 | 为每个公共函数添加测试 |

### docs (目标: 10/10)

| 行动 | 影响 | 如何执行 |
|------|------|----------|
| 创建 AGENTS.md | +5 | 添加 Agent 使用指南 |

## Constraints

1. **不要破坏现有功能** — 测试必须通过
2. **先格式后 lint** — 总是 `cargo fmt` 在 `cargo clippy` 之前
3. **一个提交一个改动** — 原子提交便于回滚
```

---

## Step 4: 运行改进循环

```bash
# 记录基线
echo '{"iteration":0,"score":5}' > iterations.jsonl

# 执行第一个改进：格式
cargo fmt
./scripts/score.sh
# Score: 25 / 70

echo '{"iteration":1,"component":"format","before":5,"after":25}' >> iterations.jsonl

# 执行 clippy 修复
cargo clippy --fix --allow-dirty
cargo fmt
./scripts/score.sh
# Score: 45 / 70
```

---

## 最终状态

```
my-cli/
├── GOAL.md           # 目标定义
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

- [多 Agent 协作](04-multi-agent.md)
- [进阶模式](05-advanced-patterns.md)
