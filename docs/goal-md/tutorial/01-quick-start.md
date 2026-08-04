# 5 分钟快速开始

**零基础上手 GOAL.md**

---

## 目标

在 5 分钟内为一个现有项目添加 GOAL.md 支持。

---

## Step 1: 准备项目

假设你有一个 Rust 项目：

```bash
cd my-project
ls
# Cargo.toml  src/  README.md
```

---

## Step 2: 创建评分脚本

创建 `scripts/score.sh`：

```bash
mkdir -p scripts
touch scripts/score.sh
chmod +x scripts/score.sh
```

编辑 `scripts/score.sh`：

```bash
#!/bin/bash
# 简单评分：只检查格式和 clippy

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0
CLIPPY_SCORE=0

# 检查格式 (20分)
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# 检查 clippy (20分)
WARN_COUNT=$(cargo clippy 2>&1 | grep -c "warning:" || echo "0")
[[ "$WARN_COUNT" -eq 0 ]] && CLIPPY_SCORE=20

TOTAL=$((FORMAT_SCORE + CLIPPY_SCORE))

echo "Score: $TOTAL / 40"
```

运行测试：

```bash
./scripts/score.sh
# 输出: Score: 20 / 40
```

---

## Step 3: 复制 GOAL.md 模板

```bash
# 从本项目复制模板
cp /path/to/AutoHarness/template/GOAL.md GOAL.md
```

或者手动创建 `GOAL.md`：

```markdown
# Goal: My Project - 提升代码质量

## Fitness Function

```bash
./scripts/score.sh
```

## Operating Mode

- [x] **Converge** — 达到目标时停止

Stop when:
- Score reaches 40/40
- 10 次迭代无改进

## Action Catalog

| Action | Impact | How |
|--------|--------|-----|
| cargo fmt | +20 | `cargo fmt` |
| Fix clippy warnings | +20 | `cargo clippy --fix` |

## Iteration Log

File: `iterations.jsonl`
```

---

## Step 4: 运行第一次评分

```bash
./scripts/score.sh
```

输出示例：
```
Score: 20 / 40
```

这意味着：
- ✓ 格式通过 (20/20)
- ✗ 有 clippy 警告 (0/20)

---

## Step 5: 执行第一个改进

根据 Action Catalog，最高影响的行动是 `cargo fmt`：

```bash
cargo fmt
./scripts/score.sh
```

等等，格式已经通过了。下一个是修复 clippy：

```bash
cargo clippy --fix
./scripts/score.sh
```

---

## Step 6: 记录迭代

创建 `iterations.jsonl`：

```bash
echo '{"iteration":1,"timestamp":"2026-08-04T12:00:00Z","action":"cargo fmt","before":20,"after":20,"result":"kept"}' >> iterations.jsonl
```

---

## 完成！

你现在有：
- ✅ `scripts/score.sh` - 评分脚本
- ✅ `GOAL.md` - 目标定义
- ✅ `iterations.jsonl` - 迭代日志

---

## 进阶下一步

想要更完整的评分？查看：
- [创建你的第一个 GOAL.md](03-create-goal.md)
- [评分脚本编写指南](04-scoring-guide.md)

---

## 常见问题

### Q: 脚本报错 "Permission denied"
```bash
chmod +x scripts/score.sh
```

### Q: 分数不增不减
检查 iterations.jsonl 是否正确记录，或者尝试更高 impact 的行动。

### Q: 如何让 Agent 自动执行？
将 `GOAL.md` 和 `CLAUDE.md` 放到项目根目录，Agent 会自动识别。
