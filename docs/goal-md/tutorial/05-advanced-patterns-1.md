# 进阶模式（上）

**高级 GOAL.md 使用技巧**

---

## 1. 双层评分

当测量工具本身需要改进时使用。

### 场景

你想改进"文档质量"，但首先需要修复"文档 linter"。

### 解决方案

```markdown
## Fitness Function

Two scores, tracked independently:

docs_quality     = (accuracy + completeness + usability) / 75
instrument_quality = (linter_precision + prop_check_recall) / 25

total = docs_quality + instrument_quality
```

### 实现

```bash
#!/bin/bash
# 双层评分

# Layer 1: 工具质量
LINTER_SCORE=$(check_linter_precision)
BENCH_SCORE=$(check_benchmark_recall)

# Layer 2: 实际指标
DOC_SCORE=$(eval_docs)

# 只有工具准备好后才计算实际指标
if [[ $LINTER_SCORE -ge 80 ]]; then
    TOTAL=$((DOC_SCORE + LINTER_SCORE))
else
    TOTAL=$LINTER_SCORE
fi
```

---

## 2. 渐进式目标

目标随进度调整。

### 场景

你想持续改进，但不想设定固定目标。

### 解决方案

```markdown
## Operating Mode

- [x] **Continuous** — 持续运行

Progressive targets:
- Week 1: Reach 60/100
- Week 2: Reach 75/100
- Week 3: Reach 90/100
- Week 4: Reach 100/100
```

### 实现

```bash
#!/bin/bash
# 渐进式目标

WEEK=$(date +%U)
BASE_WEEK=31
PROGRESS_WEEK=$((WEEK - BASE_WEEK + 1))

case $PROGRESS_WEEK in
    1) TARGET=60 ;;
    2) TARGET=75 ;;
    3) TARGET=90 ;;
    *) TARGET=100 ;;
esac

CURRENT=$(./scripts/score.sh --json | jq '.total')
echo "Progress Week $PROGRESS_WEEK: $CURRENT/$TARGET"
```

---

## 3. 加权评分

不同组件有不同权重。

### 场景

你想优先改进性能而不是格式。

### 解决方案

```markdown
## Fitness Function

```
score = performance*2 + reliability*2 + quality*1
```

| Component | Weight | Max |
|-----------|--------|-----|
| performance | 2x | 50 |
| reliability | 2x | 50 |
| quality | 1x | 50 |

Final = score / 5 (normalized to 100)
```

### 实现

```bash
PERF=$(check_performance)
REL=$(check_reliability)
QUAL=$(check_quality)

RAW=$((PERF * 2 + REL * 2 + QUAL * 1))
TOTAL=$((RAW / 5))
```

---

## 4. 时间限制评分

在时间限制内尽可能提高分数。

### 场景

你只有 1 小时时间改进项目。

### 解决方案

```bash
#!/bin/bash
TIME_LIMIT=60
START_TIME=$(date +%s)

check_time() {
    ELAPSED=$(($(date +%s) - START_TIME))
    [[ $ELAPSED -gt $((TIME_LIMIT * 60)) ]] && exit 0
}

check_time; quick_fix_1
check_time; medium_fix_1
```

---

## 5. 多目标追踪

同时追踪多个独立目标。

### 解决方案

```bash
#!/bin/bash
QUALITY=$(./scripts/quality.sh --json | jq '.total')
PERF=$(./scripts/bench.sh --json | jq '.score')
COMBINED=$(((QUALITY + PERF) / 2))
echo "{\"quality\":$QUALITY,\"performance\":$PERF,\"combined\":$COMBINED}"
```

---

## 6. 分支评分

在多个分支上同时评分。

### 解决方案

```bash
#!/bin/bash
for branch in $(git branch | grep -v '\*' | tr -d ' '); do
    git checkout "$branch"
    ./scripts/score.sh --json | jq -r '.total' >> branch_scores.txt
done
git checkout main
```

---

## 下一步

- [进阶模式（下）](05-advanced-patterns-2.md)
- [常见问题解决](06-troubleshooting.md)
