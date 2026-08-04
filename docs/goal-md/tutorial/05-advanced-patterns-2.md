# 进阶模式（下）

**更多高级技巧**

---

## 7. 回归检测

检测分数是否下降。

### 场景

你想确保改动不会导致分数下降。

### 解决方案

```bash
#!/bin/bash
# 回归检测

BASELINE_FILE=".goal-baseline.json"

# 读取基线
if [[ -f "$BASELINE_FILE" ]]; then
    BASELINE=$(cat "$BASELINE_FILE" | jq '.total')
else
    BASELINE=0
fi

# 当前分数
CURRENT=$(./scripts/score.sh --json | jq '.total')

# 检查回归
if [[ $CURRENT -lt $BASELINE ]]; then
    echo "REGRESSION DETECTED!"
    echo "  Baseline: $BASELINE"
    echo "  Current: $CURRENT"
    echo "  Delta: $((BASELINE - CURRENT))"
    exit 1
fi

# 更新基线
echo "{\"total\":$CURRENT}" > "$BASELINE_FILE"
echo "Score: $CURRENT (baseline: $BASELINE)"
```

---

## 8. 自适应 Action Catalog

根据历史数据调整行动影响。

### 场景

你想让 Action Catalog 更准确。

### 解决方案

```python
#!/usr/bin/env python3
# analyze_catalog.py

import json
from collections import defaultdict

def analyze_iterations():
    with open('iterations.jsonl') as f:
        iterations = [json.loads(line) for line in f]
    
    actions = defaultdict(list)
    for it in iterations:
        action = it.get('action', 'unknown')
        actions[action].append(it['after'] - it['before'])
    
    print("## Updated Action Impact\n")
    print("| Action | Avg Impact | Count |")
    print("|--------|-----------|-------|")
    for action, impacts in actions.items():
        avg = sum(impacts) / len(impacts)
        print(f"| {action} | {avg:.1f} | {len(impacts)} |")

analyze_iterations()
```

---

## 9. 条件评分

根据条件启用/禁用组件。

### 场景

你想根据项目类型调整评分。

### 解决方案

```bash
#!/bin/bash
TOTAL=0
TOTAL=$((TOTAL + $(check_format)))

# 只有库项目才检查 API 文档
if grep -q '\[lib\]' Cargo.toml 2>/dev/null; then
    TOTAL=$((TOTAL + $(check_api_docs)))
fi

# 只有 CLI 工具才检查 shell completion
if [[ -f "src/main.rs" ]]; then
    TOTAL=$((TOTAL + $(check_shell_completion)))
fi

echo "Score: $TOTAL"
```

---

## 10. 竞赛模式

多个 Agent 竞争提高分数。

### 场景

你想让多个 Agent 竞争改进同一项目。

### 解决方案

```bash
#!/bin/bash
CONTESTANTS=("agent-alpha" "agent-beta" "agent-gamma")

for contestant in "${CONTESTANTS[@]}"; do
    git checkout -b "contestant/$contestant"
    run_improvements
    score=$(./scripts/score.sh --json | jq '.total')
    echo "{\"contestant\":\"$contestant\",\"score\":$score}" >> contest_scores.jsonl
    git checkout main
done

cat contest_scores.jsonl | jq -s 'sort_by(.score) | reverse | .[0]'
```

---

## 11. 阈值告警

分数低于阈值时告警。

### 解决方案

```bash
#!/bin/bash
THRESHOLD=70
SCORE=$(./scripts/score.sh --json | jq '.total')

if [[ $SCORE -lt $THRESHOLD ]]; then
    echo "ALERT: Score $SCORE below threshold $THRESHOLD"
    # 发送通知
    curl -X POST "https://notify.example.com" \
        -d "Score $SCORE below threshold $THRESHOLD"
fi
```

---

## 12. 增量验证

每次改动后验证特定组件。

### 解决方案

```bash
#!/bin/bash
# 在执行改动后运行针对性验证

BEFORE=$(./scripts/score.sh --json)

# 假设要修复 clippy
cargo clippy --fix

# 只验证 clippy 组件
CLIPPY_AFTER=$(./scripts/score.sh --json | jq '.components.clippy.score')
CLIPPY_BEFORE=$(echo "$BEFORE" | jq '.components.clippy.score')

if [[ $CLIPPY_AFTER -gt $CLIPPY_BEFORE ]]; then
    echo "Clippy improved: $CLIPPY_BEFORE → $CLIPPY_AFTER"
    git commit -am "fix: clippy warnings"
else
    echo "No clippy improvement, reverting"
    git checkout .
fi
```

---

## 13. 并行评分

同时运行多个评分脚本。

### 解决方案

```bash
#!/bin/bash
# 并行评分

# 启动所有评分任务
./scripts/quality.sh --json > /tmp/quality.json &
PID1=$!

./scripts/bench.sh --json > /tmp/bench.json &
PID2=$!

./scripts/coverage.sh --json > /tmp/coverage.json &
PID3=$!

# 等待完成
wait $PID1 $PID2 $PID3

# 汇总结果
QUALITY=$(cat /tmp/quality.json | jq '.total')
BENCH=$(cat /tmp/bench.json | jq '.total')
COVERAGE=$(cat /tmp/coverage.json | jq '.total')

TOTAL=$(((QUALITY + BENCH + COVERAGE) / 3))
echo "{\"quality\":$QUALITY,\"bench\":$BENCH,\"coverage\":$COVERAGE,\"avg\":$TOTAL}"
```

---

## 14. 版本对比

对比不同版本/分支的分数。

### 解决方案

```bash
#!/bin/bash
# version_compare.sh

echo "=== Version Comparison ==="

for tag in $(git tag | sort -V); do
    git checkout "$tag"
    score=$(./scripts/score.sh --json | jq '.total')
    echo "$tag: $score"
done | column -t

git checkout -  # 返回原分支
```

---

## 15. 预测改进

基于历史数据预测何时达到目标。

### 解决方案

```python
#!/usr/bin/env python3
import json

def predict_completion():
    with open('iterations.jsonl') as f:
        data = [json.loads(line) for line in f]
    
    if not data:
        print("No data yet")
        return
    
    # 计算平均每次迭代的改进
    improvements = [d['after'] - d['before'] for d in data]
    avg_improvement = sum(improvements) / len(improvements)
    
    if avg_improvement <= 0:
        print("No progress being made")
        return
    
    current = data[-1]['after']
    target = 100
    remaining = target - current
    iterations_needed = int(remaining / avg_improvement) + 1
    
    print(f"Current: {current}/100")
    print(f"Avg improvement per iteration: {avg_improvement:.2f}")
    print(f"Estimated iterations to target: {iterations_needed}")
    print(f"Progress: {current/100*100:.1f}%")

predict_completion()
```

---

## 下一步

- [常见问题解决](06-troubleshooting.md)
