# 进阶模式

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

---

## 2. 渐进式目标

目标随进度调整。

```markdown
## Operating Mode

- [x] **Continuous** — 持续运行

Progressive targets:
- Week 1: Reach 60/100
- Week 2: Reach 75/100
- Week 3: Reach 90/100
- Week 4: Reach 100/100
```

---

## 3. 加权评分

不同组件有不同权重。

```markdown
## Fitness Function

score = performance*2 + reliability*2 + quality*1

| Component | Weight | Max |
|-----------|--------|-----|
| performance | 2x | 50 |
| reliability | 2x | 50 |
| quality | 1x | 50 |

Final = score / 5 (normalized to 100)
```

---

## 4. 时间限制评分

在时间限制内尽可能提高分数。

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

```bash
#!/bin/bash
QUALITY=$(./scripts/quality.sh --json | jq '.total')
PERF=$(./scripts/bench.sh --json | jq '.score')
COMBINED=$(((QUALITY + PERF) / 2))
echo "{\"quality\":$QUALITY,\"performance\":$PERF,\"combined\":$COMBINED}"
```

---

## 6. 回归检测

检测分数是否下降。

```bash
#!/bin/bash
BASELINE_FILE=".goal-baseline.json"

if [[ -f "$BASELINE_FILE" ]]; then
    BASELINE=$(cat "$BASELINE_FILE" | jq '.total')
else
    BASELINE=0
fi

CURRENT=$(./scripts/score.sh --json | jq '.total')

if [[ $CURRENT -lt $BASELINE ]]; then
    echo "REGRESSION: $BASELINE -> $CURRENT"
    exit 1
fi

echo "Score: $CURRENT (baseline: $BASELINE)"
```

---

## 7. 竞赛模式

多个 Agent 竞争提高分数。

```bash
#!/bin/bash
CONTESTANTS=("agent-alpha" "agent-beta")

for contestant in "${CONTESTANTS[@]}"; do
    git checkout -b "contestant/$contestant"
    run_improvements
    score=$(./scripts/score.sh --json | jq '.total')
    echo "{\"contestant\":\"$contestant\",\"score\":$score}" >> contest_scores.jsonl
    git checkout main
done
```

---

## 8. 阈值告警

分数低于阈值时告警。

```bash
#!/bin/bash
THRESHOLD=70
SCORE=$(./scripts/score.sh --json | jq '.total')

if [[ $SCORE -lt $THRESHOLD ]]; then
    echo "ALERT: Score $SCORE below threshold $THRESHOLD"
fi
```

---

## 9. 增量验证

每次改动后验证特定组件。

```bash
#!/bin/bash
BEFORE=$(./scripts/score.sh --json)

# 假设要修复 clippy
cargo clippy --fix

CLIPPY_AFTER=$(./scripts/score.sh --json | jq '.components.clippy.score')
CLIPPY_BEFORE=$(echo "$BEFORE" | jq '.components.clippy.score')

if [[ $CLIPPY_AFTER -gt $CLIPPY_BEFORE ]]; then
    git commit -am "fix: clippy warnings"
else
    git checkout .
fi
```

---

## 下一步

- [常见问题解决](06-troubleshooting.md)
