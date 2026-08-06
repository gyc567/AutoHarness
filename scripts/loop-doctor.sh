#!/bin/bash
# AutoHarness Loop Readiness Score
# 详见 docs/loop-engineering/patterns-and-levels.md §3.2
set -uo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

TOTAL=0
MAX=100

# 1. LOOP.md (15)
SCORE=0
REASON=""
if [[ -f "LOOP.md" ]]; then
  if grep -q "## Active Loops" LOOP.md 2>/dev/null; then
    SCORE=15
    REASON="exists and has Active Loops section"
  else
    SCORE=8
    REASON="exists but missing Active Loops section"
  fi
else
  REASON="LOOP.md not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_LOOP="{\"name\":\"LOOP.md\",\"score\":$SCORE,\"max\":15,\"reason\":\"$REASON\"}"

# 2. STATE.md (15)
SCORE=0
REASON=""
if [[ -f "STATE.md" ]]; then
  # 检查 24h 内更新
  if find STATE.md -mmin -1440 2>/dev/null | grep -q .; then
    SCORE=15
    REASON="exists and updated within 24h"
  else
    SCORE=8
    REASON="exists but stale (>24h)"
  fi
else
  REASON="STATE.md not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_STATE="{\"name\":\"STATE.md\",\"score\":$SCORE,\"max\":15,\"reason\":\"$REASON\"}"

# 3. loop-budget.md (10)
SCORE=0
REASON=""
if [[ -f "loop-budget.md" ]]; then
  SCORE=10
  REASON="exists"
else
  REASON="loop-budget.md not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_BUDGET="{\"name\":\"loop-budget.md\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 4. loop-run-log.jsonl (10)
SCORE=0
REASON=""
if [[ -f "loop-run-log.jsonl" ]]; then
  RECORD_COUNT=$(wc -l < loop-run-log.jsonl 2>/dev/null | tr -d ' ')
  RECORD_COUNT=${RECORD_COUNT:-0}
  if [[ "$RECORD_COUNT" -gt 0 ]]; then
    SCORE=10
    REASON="exists with $RECORD_COUNT records"
  else
    SCORE=5
    REASON="exists but empty"
  fi
else
  REASON="loop-run-log.jsonl not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_LOG="{\"name\":\"loop-run-log.jsonl\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 5. loop-constraints.md (10)
SCORE=0
REASON=""
if [[ -f "loop-constraints.md" ]]; then
  SCORE=10
  REASON="exists"
else
  REASON="loop-constraints.md not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_CONSTRAINTS="{\"name\":\"loop-constraints.md\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 6. gate.yaml (10)
SCORE=0
REASON=""
if [[ -f "gate.yaml" ]]; then
  # 简化：检查包含 denylist 段
  if grep -q "denylist" gate.yaml 2>/dev/null; then
    SCORE=10
    REASON="exists and parses"
  else
    SCORE=3
    REASON="exists but parse error"
  fi
else
  REASON="gate.yaml not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_GATE="{\"name\":\"gate.yaml\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 7. patterns/registry.yaml (10)
SCORE=0
REASON=""
if [[ -f "patterns/registry.yaml" ]]; then
  SCORE=10
  REASON="exists"
else
  REASON="patterns/registry.yaml not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_PATTERNS="{\"name\":\"patterns/registry.yaml\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 8. ≥ 1 个 Pattern 跑过 ≥ 3 次 L1 (10)
SCORE=0
REASON=""
if [[ -f "loop-run-log.jsonl" ]]; then
  L1_COUNT=$(grep -c '"level":"L1"' loop-run-log.jsonl 2>/dev/null | head -1)
  L1_COUNT=${L1_COUNT:-0}
  # 清理可能的换行
  L1_COUNT=$(echo "$L1_COUNT" | tr -d '\n' | grep -oE '^[0-9]*')
  L1_COUNT=${L1_COUNT:-0}
  if [[ "$L1_COUNT" -ge 3 ]]; then
    SCORE=10
    REASON="$L1_COUNT L1 runs recorded"
  else
    SCORE=3
    REASON="only $L1_COUNT L1 runs (< 3)"
  fi
else
  REASON="no run log"
fi
TOTAL=$((TOTAL + SCORE))
DIM_L1RUNS="{\"name\":\"L1 runs >= 3\",\"score\":$SCORE,\"max\":10,\"reason\":\"$REASON\"}"

# 9. maker/checker 分离 (5)
SCORE=0
REASON=""
if grep -r "Refiner" src/ 2>/dev/null | grep -q . && \
   grep -r "Verifier" src/ 2>/dev/null | grep -q . && \
   grep -r "Critic" src/ 2>/dev/null | grep -q .; then
  SCORE=5
  REASON="found HarnessType::{Refiner, Verifier, Critic} refs"
else
  REASON="no maker/checker refs found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_MC="{\"name\":\"maker-checker\",\"score\":$SCORE,\"max\":5,\"reason\":\"$REASON\"}"

# 10. kill switch (5)
SCORE=0
REASON=""
if [[ -f "STATE.md" ]]; then
  if grep -q "pause-all:" STATE.md 2>/dev/null; then
    SCORE=5
    REASON="STATE.md has kill switch flag"
  else
    SCORE=2
    REASON="STATE.md missing kill switch flag"
  fi
else
  REASON="STATE.md not found"
fi
TOTAL=$((TOTAL + SCORE))
DIM_KILL="{\"name\":\"kill-switch\",\"score\":$SCORE,\"max\":5,\"reason\":\"$REASON\"}"

# Readiness
if [[ $TOTAL -lt 40 ]]; then
  READINESS="R0"
elif [[ $TOTAL -lt 60 ]]; then
  READINESS="R1"
elif [[ $TOTAL -lt 80 ]]; then
  READINESS="R2"
else
  READINESS="R3"
fi

if [[ "$JSON_OUTPUT" == "true" ]]; then
  echo "{\"total\":$TOTAL,\"max\":$MAX,\"readiness\":\"$READINESS\",\"dimensions\":[$DIM_LOOP,$DIM_STATE,$DIM_BUDGET,$DIM_LOG,$DIM_CONSTRAINTS,$DIM_GATE,$DIM_PATTERNS,$DIM_L1RUNS,$DIM_MC,$DIM_KILL]}"
else
  echo ""
  echo "═══════════════════════════════════════════════════════════════════"
  echo "  AutoHarness Loop Readiness: $TOTAL / $MAX  ($READINESS)"
  echo "═══════════════════════════════════════════════════════════════════"
  echo ""
  echo "  LOOP.md                 : ${DIM_LOOP#*\"score\":}"
  echo ""
  echo "  See docs/loop-engineering/patterns-and-levels.md §3.2 for full rubric."
  echo ""
fi