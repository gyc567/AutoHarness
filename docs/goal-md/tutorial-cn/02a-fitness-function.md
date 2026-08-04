# Fitness Function（适应度函数）

**评分脚本的核心设计指南**

---

## 什么是 Fitness Function？

Fitness Function 是一个程序，输出一个数字来衡量项目质量：

```bash
./scripts/score.sh
# 输出: 85 / 100
```

分数越高，项目越好。

---

## 基本结构

```bash
#!/bin/bash
set -uo pipefail

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0
CLIPPY_SCORE=0
TEST_SCORE=0

# 组件 1: 格式检查 (20分)
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# 组件 2: Lint 检查 (20分)
CLIPPY_OUTPUT=$(cargo clippy 2>&1 || true)
WARN_COUNT=$(echo "$CLIPPY_OUTPUT" | grep -c "warning:" || true)
[[ "$WARN_COUNT" -eq 0 ]] && CLIPPY_SCORE=20

# 组件 3: 测试覆盖 (25分)
cargo test --no-run 2>/dev/null && TEST_SCORE=25

# 计算总分
TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE))

# 输出
echo "Score: $TOTAL / 65"
```

---

## 设计原则

### 1. 确定性

相同输入必须产生相同输出：

```bash
# ✅ 确定性：每次运行结果一致
WARN_COUNT=$(cargo clippy 2>&1 | grep -c "warning:" || true)

# ❌ 非确定性：每次可能不同
RANDOM_VALUE=$((RANDOM % 100))
```

### 2. 快速

最好在 60 秒内完成：

| 时间 | 建议 |
|------|------|
| < 30s | 理想，适合频繁运行 |
| 30-60s | 可接受 |
| > 60s | 考虑拆分成子脚本 |

### 3. 独立

不依赖外部状态：

```bash
# ✅ 独立：每次都完整检查
cargo fmt -- --check

# ❌ 依赖：依赖上一次的状态
[[ -f ".last_score" ]] && cat .last_score
```

### 4. 可组合

分数 = 各组件分数之和：

```bash
# 评分结构
score = format(20) + clippy(20) + tests(25) + docs(15) + ...

TOTAL=$((FORMAT + CLIPPY + TEST + DOC))
```

---

## 组件设计

### 常见组件

| 组件 | 分值 | 检查内容 | 工具 |
|------|------|----------|------|
| format | 20 | 代码格式 | `cargo fmt` |
| clippy | 20 | Lint 警告 | `cargo clippy` |
| tests | 25 | 测试通过 | `cargo test` |
| docs | 15 | 文档完整 | 文件检查 |
| maintenance | 10 | 项目维护 | 文件检查 |
| safety | 10 | 安全代码 | `unsafe` 检查 |

### 组件评分逻辑

```bash
# 格式检查
if cargo fmt -- --check 2>/dev/null; then
    FORMAT_SCORE=20
else
    FORMAT_SCORE=0
fi

# 警告计数分级
case "$WARN_COUNT" in
    0) CLIPPY_SCORE=20 ;;
    1|2|3) CLIPPY_SCORE=15 ;;
    4|5|6|7|8|9) CLIPPY_SCORE=10 ;;
    *) CLIPPY_SCORE=5 ;;
esac
```

---

## JSON 输出格式

支持 `--json` 参数是标准做法：

```bash
if [[ "$JSON_OUTPUT" == "true" ]]; then
    echo "{\"total\":$TOTAL,\"max\":100}"
else
    echo "Score: $TOTAL / 100"
fi
```

### 标准 JSON 结构

```json
{
  "timestamp": "2026-08-04T12:00:00Z",
  "total": 85,
  "max": 100,
  "components": {
    "format": {"score": 20, "max": 20},
    "clippy": {"score": 20, "max": 20}
  }
}
```

---

## 常见模式

### 1. 工具存在性检查

```bash
# 检查工具是否存在
if command -v cargo-tarpaulin &>/dev/null; then
    COVERAGE=$(cargo tarpaulin --out json | jq '.line_percent')
else
    COVERAGE=0
fi
```

### 2. 超时处理

```bash
# 使用 timeout 防止脚本卡住
TEST_OUTPUT=$(timeout 120 cargo test 2>&1 || true)
```

---

## 下一步

- [创建你的第一个 GOAL.md](03-create-goal.md)
- [Action Catalog 详解](02b-action-catalog.md)
