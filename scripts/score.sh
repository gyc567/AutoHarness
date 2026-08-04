#!/bin/bash
# AutoHarness GOAL.md Fitness Function
# Evaluates code quality along multiple dimensions

JSON_OUTPUT=false
[[ "$1" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

# Initialize scores
FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0
DOC_SCORE=0; MAINTENANCE_SCORE=0

# 1. Format Check (max 20 points)
if cargo fmt -- --check 2>/dev/null; then
    FORMAT_SCORE=20; FORMAT_DETAIL="All code properly formatted"
else
    FORMAT_SCORE=0; FORMAT_DETAIL="Formatting issues detected"
fi

# 2. Clippy Check (max 20 points) - timeout after 60s
echo "Running clippy..."
if CLIPPY_OUTPUT=$(timeout 60 cargo clippy 2>&1 || true); then
    CLIPPY_WARNINGS=$(echo "$CLIPPY_OUTPUT" | grep -c "warning:" || echo "0")
    case "$CLIPPY_WARNINGS" in
        0) CLIPPY_SCORE=20; CLIPPY_DETAIL="No clippy warnings" ;;
        [1-3]) CLIPPY_SCORE=15; CLIPPY_DETAIL="$CLIPPY_WARNINGS warnings (minor)" ;;
        [4-9]) CLIPPY_SCORE=10; CLIPPY_DETAIL="$CLIPPY_WARNINGS warnings (moderate)" ;;
        *) CLIPPY_SCORE=5; CLIPPY_DETAIL="$CLIPPY_WARNINGS warnings (significant)" ;;
    esac
else
    CLIPPY_SCORE=0; CLIPPY_DETAIL="Clippy timeout or error"
fi

# 3. Test Coverage (max 25 points) - timeout after 90s
echo "Running tests..."
if TEST_OUTPUT=$(timeout 90 cargo test 2>&1 || true); then
    if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
        FAILED=$(echo "$TEST_OUTPUT" | grep -oP '\d+ failed' | grep -oP '\d+' || echo "0")
        PASSED=$(echo "$TEST_OUTPUT" | grep -oP '\d+ passed' | grep -oP '\d+' | tail -1 || echo "0")
        TOTAL=$((FAILED + PASSED))
        if [[ "$FAILED" -eq 0 && "$TOTAL" -gt 0 ]]; then
            TEST_SCORE=25; TEST_DETAIL="All $PASSED tests passing"
        elif [[ "$FAILED" -eq 0 ]]; then
            TEST_SCORE=20; TEST_DETAIL="Tests passed"
        else
            [[ "$TOTAL" -gt 0 ]] && TEST_SCORE=$(((PASSED * 100 / TOTAL) * 25 / 100)) || TEST_SCORE=0
            TEST_DETAIL="$FAILED failed, $PASSED passed"
        fi
    else
        TEST_SCORE=0; TEST_DETAIL="Test suite failed"
    fi
else
    TEST_SCORE=0; TEST_DETAIL="Test timeout or error"
fi

# 4. Documentation (max 15 points)
[[ -f "AGENTS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -f "DOCS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -d "docs" ]] && [[ -n "$(ls -A docs 2>/dev/null)" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ "$DOC_SCORE" -eq 0 ]] && DOC_DETAIL="Missing documentation"
[[ "$DOC_SCORE" -eq 5 ]] && DOC_DETAIL="1 doc file"
[[ "$DOC_SCORE" -eq 10 ]] && DOC_DETAIL="2 doc files"
[[ "$DOC_SCORE" -eq 15 ]] && DOC_DETAIL="All docs present"

# 5. Maintenance (max 20 points)
grep -q 'name = "autoharness"' Cargo.toml 2>/dev/null && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ -f ".gitignore" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ -f "Cargo.lock" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
SRC_FILES=$(find src -name "*.rs" 2>/dev/null | wc -l)
[[ "$SRC_FILES" -gt 5 ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ "$MAINTENANCE_SCORE" -eq 0 ]] && MAINTENANCE_DETAIL="Basic checks"
[[ "$MAINTENANCE_SCORE" -gt 0 ]] && MAINTENANCE_DETAIL="$MAINTENANCE_SCORE/20 checks passed"

# Calculate Total Score
TOTAL_SCORE=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE))
MAX_SCORE=100

# Output Results
if [[ "$JSON_OUTPUT" == "true" ]]; then
    cat << EOF
{"total":$TOTAL_SCORE,"max":$MAX_SCORE,"percentage":$((TOTAL_SCORE)),"components":{"format":{"score":$FORMAT_SCORE,"max":20,"detail":"$FORMAT_DETAIL"},"clippy":{"score":$CLIPPY_SCORE,"max":20,"detail":"$CLIPPY_DETAIL"},"tests":{"score":$TEST_SCORE,"max":25,"detail":"$TEST_DETAIL"},"docs":{"score":$DOC_SCORE,"max":15,"detail":"$DOC_DETAIL"},"maintenance":{"score":$MAINTENANCE_SCORE,"max":20,"detail":"$MAINTENANCE_DETAIL"}}}
EOF
else
    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  AutoHarness Code Quality: $TOTAL_SCORE / $MAX_SCORE ($((TOTAL_SCORE))%)"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    printf "  %-25s ◐ %2d / 20  (%s)\n" "format" "$FORMAT_SCORE" "$FORMAT_DETAIL"
    printf "  %-25s ◐ %2d / 20  (%s)\n" "clippy" "$CLIPPY_SCORE" "$CLIPPY_DETAIL"
    printf "  %-25s ◐ %2d / 25  (%s)\n" "tests" "$TEST_SCORE" "$TEST_DETAIL"
    printf "  %-25s ◐ %2d / 15  (%s)\n" "docs" "$DOC_SCORE" "$DOC_DETAIL"
    printf "  %-25s ◐ %2d / 20  (%s)\n" "maintenance" "$MAINTENANCE_SCORE" "$MAINTENANCE_DETAIL"
    echo ""
fi