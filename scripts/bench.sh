#!/bin/bash
# AutoHarness Performance Fitness Function
# Measures synthesis time and CLI response time

set -uo pipefail

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Initialize scores
SYNTHESIS_TIME_SCORE=0; CLI_RESPONSE_SCORE=0
SYNTHESIS_TIME_MS=0; CLI_RESPONSE_MS=0
SYNTHESIS_DETAIL=""; CLI_DETAIL=""

# Target thresholds (from North Star Metrics)
SYNTHESIS_TARGET_MS=500
CLI_RESPONSE_TARGET_MS=100

# Check if benchmarks exist
if [[ -d "benches" ]] && [[ -n "$(ls -A benches 2>/dev/null)" ]]; then
    # Run benchmarks
    BENCH_OUTPUT=$(cargo bench 2>&1 || true)
    
    # Parse synthesis time (rough extraction)
    SYNTHESIS_TIME_MS=$(echo "$BENCH_OUTPUT" | grep -oP 'time: \d+\.\d+' | head -1 | grep -oP '\d+\.\d+' || echo "0")
    
    if [[ -n "$SYNTHESIS_TIME_MS" ]]; then
        SYNTHESIS_TIME_MS_INT=$(echo "$SYNTHESIS_TIME_MS" | cut -d. -f1)
        if [[ "$SYNTHESIS_TIME_MS_INT" -lt "$SYNTHESIS_TARGET_MS" ]]; then
            SYNTHESIS_TIME_SCORE=25
            SYNTHESIS_DETAIL="Synthesis time: ${SYNTHESIS_TIME_MS}ms (target: <${SYNTHESIS_TARGET_MS}ms)"
        else
            SYNTHESIS_TIME_SCORE=$((25 - ((SYNTHESIS_TIME_MS_INT - SYNTHESIS_TARGET_MS) / 50)))
            [[ $SYNTHESIS_TIME_SCORE -lt 0 ]] && SYNTHESIS_TIME_SCORE=0
            SYNTHESIS_DETAIL="Synthesis time: ${SYNTHESIS_TIME_MS}ms (target: <${SYNTHESIS_TARGET_MS}ms)"
        fi
    else
        SYNTHESIS_DETAIL="Benchmark output not parseable"
    fi
else
    SYNTHESIS_DETAIL="No benchmarks found"
fi

# CLI response time (simple measurement)
if command -v hyperfine &>/dev/null; then
    CLI_RESPONSE_MS=$(hyperfine 'cargo run --quiet -- --help' 2>&1 | grep -oP 'Mean.*: \d+\.\d+ ms' | grep -oP '\d+\.\d+' || echo "0")
else
    # Fallback: time cargo run
    START=$(date +%s%N)
    cargo run --quiet -- --help &>/dev/null
    END=$(date +%s%N)
    CLI_RESPONSE_MS=$(( (END - START) / 1000000 ))
fi

if [[ "$CLI_RESPONSE_MS" -lt "$CLI_RESPONSE_TARGET_MS" ]]; then
    CLI_RESPONSE_SCORE=25
    CLI_DETAIL="CLI response: ${CLI_RESPONSE_MS}ms (target: <${CLI_RESPONSE_TARGET_MS}ms)"
else
    CLI_RESPONSE_SCORE=$((25 - ((CLI_RESPONSE_MS - CLI_RESPONSE_TARGET_MS) / 20)))
    [[ $CLI_RESPONSE_SCORE -lt 0 ]] && CLI_RESPONSE_SCORE=0
    CLI_DETAIL="CLI response: ${CLI_RESPONSE_MS}ms (target: <${CLI_RESPONSE_TARGET_MS}ms)"
fi

# Calculate total
TOTAL_SCORE=$((SYNTHESIS_TIME_SCORE + CLI_RESPONSE_SCORE))
MAX_SCORE=50

# Output
if [[ "$JSON_OUTPUT" == "true" ]]; then
    cat << EOF
{"timestamp":"$TIMESTAMP","version":"0.1.0","total":$TOTAL_SCORE,"max":$MAX_SCORE,"components":{"synthesis_time":{"score":$SYNTHESIS_TIME_SCORE,"max":25,"value_ms":$SYNTHESIS_TIME_MS,"target_ms":$SYNTHESIS_TARGET_MS,"detail":"$SYNTHESIS_DETAIL"},"cli_response":{"score":$CLI_RESPONSE_SCORE,"max":25,"value_ms":$CLI_RESPONSE_MS,"target_ms":$CLI_RESPONSE_TARGET_MS,"detail":"$CLI_DETAIL"}}}
EOF
else
    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
    printf "  AutoHarness Performance: %d / %d\n" "$TOTAL_SCORE" "$MAX_SCORE"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    [[ $SYNTHESIS_TIME_SCORE -eq 25 ]] && SYN_ICON="✓" || SYN_ICON="◐"
    [[ $CLI_RESPONSE_SCORE -eq 25 ]] && CLI_ICON="✓" || CLI_ICON="◐"
    printf "  %-25s %s %2d / 25  (%s)\n" "synthesis_time" "$SYN_ICON" "$SYNTHESIS_TIME_SCORE" "$SYNTHESIS_DETAIL"
    printf "  %-25s %s %2d / 25  (%s)\n" "cli_response" "$CLI_ICON" "$CLI_RESPONSE_SCORE" "$CLI_DETAIL"
    echo ""
fi