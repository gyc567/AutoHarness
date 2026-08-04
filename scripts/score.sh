#!/bin/bash
# AutoHarness Code Quality Fitness Function
set -uo pipefail

JSON_OUTPUT=false
[[ "${1:-}" == "--json" ]] && JSON_OUTPUT=true

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

FORMAT_SCORE=0; CLIPPY_SCORE=0; TEST_SCORE=0
DOC_SCORE=0; MAINTENANCE_SCORE=0; SAFETY_SCORE=0
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Format Check
cargo fmt -- --check 2>/dev/null && FORMAT_SCORE=20

# Clippy Check
CLIPPY_OUTPUT=$(cargo clippy 2>&1 || true)
WARN_COUNT=$(echo "$CLIPPY_OUTPUT" | grep -c "warning:" || true)
WARN_COUNT=${WARN_COUNT:-0}
if [[ "$WARN_COUNT" -eq 0 ]]; then
    CLIPPY_SCORE=20
elif [[ "$WARN_COUNT" -le 3 ]]; then
    CLIPPY_SCORE=15
elif [[ "$WARN_COUNT" -le 9 ]]; then
    CLIPPY_SCORE=10
else
    CLIPPY_SCORE=5
fi

# Test Coverage (check compilation)
TEST_BUILD=$(cargo test --no-run 2>&1 || true)
if echo "$TEST_BUILD" | grep -E "^error\[|^error:" | grep -v "unused" | grep -q .; then
    TEST_SCORE=0
else
    TEST_SCORE=25
fi

# Documentation
[[ -f "AGENTS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -f "DOCS.md" ]] && DOC_SCORE=$((DOC_SCORE + 5))
[[ -d "docs" && -n "$(ls -A docs 2>/dev/null)" ]] && DOC_SCORE=$((DOC_SCORE + 5))

# Maintenance
grep -q 'name = "autoharness"' Cargo.toml 2>/dev/null && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ -f ".gitignore" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ -f "Cargo.lock" ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))
[[ $(find src -name "*.rs" 2>/dev/null | wc -l) -gt 5 ]] && MAINTENANCE_SCORE=$((MAINTENANCE_SCORE + 5))

# Safety
UNSAFE_COUNT=$(grep -r "unsafe" src/ 2>/dev/null | grep -cv "// unsafe" || echo "0")
if [[ "$UNSAFE_COUNT" -eq 0 ]]; then
    SAFETY_SCORE=10
elif [[ "$UNSAFE_COUNT" -le 10 ]]; then
    SAFETY_SCORE=7
else
    SAFETY_SCORE=5
fi

# Calculate total (capped at 100)
RAW_SCORE=$((FORMAT_SCORE + CLIPPY_SCORE + TEST_SCORE + DOC_SCORE + MAINTENANCE_SCORE + SAFETY_SCORE))
TOTAL_SCORE=$((RAW_SCORE > 100 ? 100 : RAW_SCORE))

if [[ "$JSON_OUTPUT" == "true" ]]; then
    echo "{\"timestamp\":\"$TIMESTAMP\",\"total\":$TOTAL_SCORE,\"max\":100,\"components\":{\"format\":{\"score\":$FORMAT_SCORE,\"max\":20},\"clippy\":{\"score\":$CLIPPY_SCORE,\"max\":20},\"tests\":{\"score\":$TEST_SCORE,\"max\":25},\"docs\":{\"score\":$DOC_SCORE,\"max\":15},\"maintenance\":{\"score\":$MAINTENANCE_SCORE,\"max\":20},\"safety\":{\"score\":$SAFETY_SCORE,\"max\":10}}}"
else
    echo ""
    echo "═══════════════════════════════════════════════════════════════════"
    echo "  AutoHarness Code Quality: $TOTAL_SCORE / 100"
    echo "═══════════════════════════════════════════════════════════════════"
    echo ""
    echo "  format      : $FORMAT_SCORE / 20"
    echo "  clippy      : $CLIPPY_SCORE / 20  ($WARN_COUNT warnings)"
    echo "  tests       : $TEST_SCORE / 25"
    echo "  docs        : $DOC_SCORE / 15"
    echo "  maintenance : $MAINTENANCE_SCORE / 20"
    echo "  safety      : $SAFETY_SCORE / 10  ($UNSAFE_COUNT unsafe blocks)"
    echo ""
fi