#!/bin/bash
# AutoHarness Loop Accuracy Tracker
# 详见 docs/loop-engineering/patterns-and-levels.md §3.4 (Q3=A 数据驱动)
#
# 用法:
#   bash scripts/loop-accuracy.sh           # 报告当前 accuracy
#   bash scripts/loop-accuracy.sh --check  # 检查是否达到 L2 解锁门
set -uo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

STATE_FILE="STATE.md"

if [[ ! -f "$STATE_FILE" ]]; then
  echo "STATE.md not found"
  exit 1
fi

# 解析 ## Accuracy Tracking 段
TOTAL_ITEMS=0
HIT_ITEMS=0
MISS_ITEMS=0

while IFS= read -r line; do
  if [[ "$line" =~ review:\ ([0-9]+)\ items,\ ([0-9]+)\ hit,\ ([0-9]+)\ miss ]]; then
    I=${BASH_REMATCH[1]}
    H=${BASH_REMATCH[2]}
    M=${BASH_REMATCH[3]}
    TOTAL_ITEMS=$((TOTAL_ITEMS + I))
    HIT_ITEMS=$((HIT_ITEMS + H))
    MISS_ITEMS=$((MISS_ITEMS + M))
  fi
done < <(grep "review:" "$STATE_FILE" || true)

if [[ $TOTAL_ITEMS -eq 0 ]]; then
  echo "No accuracy records found in STATE.md"
  echo "Add review lines under '## Accuracy Tracking':"
  echo "  - 2026-08-06 review: 5 items, 4 hit, 1 miss → 80%"
  exit 1
fi

# 漏报权重 2x（与 audit §C14 一致，未来升级 F-score）
WEIGHTED_HIT=$((HIT_ITEMS * 1 + MISS_ITEMS * 0))
WEIGHTED_TOTAL=$((HIT_ITEMS * 1 + MISS_ITEMS * 2))

if [[ $WEIGHTED_TOTAL -eq 0 ]]; then
  ACCURACY=0
else
  ACCURACY=$((WEIGHTED_HIT * 100 / WEIGHTED_TOTAL))
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  Loop Accuracy (with miss weight 2x)"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "  Total items:    $TOTAL_ITEMS"
echo "  Hit items:      $HIT_ITEMS"
echo "  Miss items:     $MISS_ITEMS"
echo "  Accuracy:       $ACCURACY %"
echo ""

if [[ "${1:-}" == "--check" ]]; then
  echo "─── L2 解锁检查 (Q3=A) ───"
  # 门 1: 14 天（检查 STATE.md Last updated 时间）
  # 门 2: ≥ 10 次 L1 runs
  L1_COUNT=$(grep -c '"level":"L1"' loop-run-log.jsonl 2>/dev/null | head -1)
  L1_COUNT=${L1_COUNT:-0}
  L1_COUNT=$(echo "$L1_COUNT" | tr -d '\n' | grep -oE '^[0-9]*')
  L1_COUNT=${L1_COUNT:-0}
  # 门 3: accuracy ≥ 80%
  # 门 4: 人类签字（人工检查 STATE.md 中的 L2-unlock-approved 行）

  echo "门 1 (14天 L1 观察期): 依赖人工时间检查"
  echo "门 2 (≥ 10 次 L1 runs): $L1_COUNT / 10"
  echo "门 3 (accuracy ≥ 80%):  $ACCURACY / 100"
  echo "门 4 (人类签字):       依赖 STATE.md 中 L2-unlock-approved 行"

  if [[ $L1_COUNT -ge 10 ]] && [[ $ACCURACY -ge 80 ]]; then
    echo ""
    echo "✅ 门 2 + 门 3 已过；门 1 + 门 4 需人工确认"
  else
    echo ""
    echo "❌ 至少 1 门未过，暂不可解锁 L2"
  fi
fi