#!/bin/bash
# AutoHarness Coverage Fitness Function
# Measures test coverage using cargo tarpaulin

set -uo pipefail

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Target thresholds (from North Star Metrics)
LINE_COVERAGE_TARGET=80
BRANCH_COVERAGE_TARGET=60

LINE_COVERAGE_SCORE=0; BRANCH_COVERAGE_SCORE=0
LINE_COVERAGE_PCT=0; BRANCH_COVERAGE_PCT=0
LINE_DETAIL=""; BRANCH_DETAIL=""

# Check if tarpaulin is available
if command -v cargo-tarpaulin &>/dev/null; then
    COVERAGE_OUTPUT=$(cargo tarpaulin --out json 2>&1 || true)
    LINE_COVERAGE_PCT=$(echo "$COVERAGE_OUTPUT" | jq -r '.line_percent // 0' 2>/dev/null | cut -d. -f1 || echo "0")
    BRANCH_COVERAGE_PCT=$(echo "$COVERAGE_OUTPUT" | jq -r '.branch_percent // 0' 2>/dev/null | cut -d. -f1 || echo "0")
    
    # Line coverage score
    if [[ "$LINE_COVERAGE_PCT" -ge "$LINE_COVERAGE_TARGET" ]]; then
        LINE_COVERAGE_SCORE=50
        LINE_DETAIL="Line coverage: ${LINE_COVERAGE_PCT}% (target: >=${LINE_COVERAGE_TARGET}%)"
    else
        LINE_COVERAGE_SCORE=$((LINE_COVERAGE_PCT / 2))
        LINE_DETAIL="Line coverage: ${LINE_COVERAGE_PCT}% (target: >=${LINE_COVERAGE_TARGET}%)"
    fi
    
    # Branch coverage score
    if [[ "$BRANCH_COVERAGE_PCT" -ge "$BRANCH_COVERAGE_TARGET" ]]; then
        BRANCH_COVERAGE_SCORE=50
        BRANCH_DETAIL="Branch coverage: ${BRANCH_COVERAGE_PCT}% (target: >=${BRANCH_COVERAGE_TARGET}%)"
    else
        BRANCH_COVERAGE_SCORE=$((BRANCH_COVERAGE_PCT / 2))
        BRANCH_DETAIL="Branch coverage: ${BRANCH_COVERAGE_PCT}% (target: >=${BRANCH_COVERAGE_TARGET}%)"
    fi
else
    LINE_DETAIL="cargo-tarpaulin not installed"
    BRANCH_DETAIL="Install with: cargo install cargo-tarpaulin"
fi

# Calculate total
TOTAL_SCORE=$((LINE_COVERAGE_SCORE + BRANCH_COVERAGE_SCORE))
MAX_SCORE=100

# Output
if [[ "$JSON_OUTPUT" == "true" ]]; then
    cat << EOF
{"timestamp":"$TIMESTAMP","version":"0.1.0","total":$TOTAL_SCORE,"max":$MAX_SCORE,"components":{"line_coverage":{"score":$LINE_COVERAGE_SCORE,"max":50,"value_pct":$LINE_COVERAGE_PCT,"target_pct":$LINE_COVERAGE_TARGET,"detail":"$LINE_DETAIL"},"branch_coverage":{"score":$BRANCH_COVERAGE_SCORE,"max":50,"value_pct":$BRANCH_COVERAGE_PCT,"target_pct":$BRANCH_COVERAGE_TARGET,"detail":"$BRANCH_DETAIL"}}}
EOF
else
    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
    printf "  AutoHarness Coverage: %d / %d\n" "$TOTAL_SCORE" "$MAX_SCORE"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    [[ $LINE_COVERAGE_SCORE -ge 40 ]] && LINE_ICON="✓" || LINE_ICON="◐"
    [[ $BRANCH_COVERAGE_SCORE -ge 40 ]] && BRANCH_ICON="✓" || BRANCH_ICON="◐"
    printf "  %-25s %s %2d / 50  (%s)\n" "line_coverage" "$LINE_ICON" "$LINE_COVERAGE_SCORE" "$LINE_DETAIL"
    printf "  %-25s %s %2d / 50  (%s)\n" "branch_coverage" "$BRANCH_ICON" "$BRANCH_COVERAGE_SCORE" "$BRANCH_DETAIL"
    echo ""
fi