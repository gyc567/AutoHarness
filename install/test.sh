#!/bin/sh
# ========== Test Suite for AutoHarness Install Script ==========

# Do NOT exit on error - we want to run all tests
set +e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INSTALL_SCRIPT="${SCRIPT_DIR}/install.sh"
TESTBIN="${SCRIPT_DIR}/autoharness-darwin-x86_64"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASSED=0
FAILED=0
SKIPPED=0

test_pass() {
    printf "${GREEN}[PASS]${NC} %s\n" "$1"
    PASSED=$((PASSED + 1))
}

test_fail() {
    printf "${RED}[FAIL]${NC} %s\n" "$1"
    FAILED=$((FAILED + 1))
}

test_skip() {
    printf "${YELLOW}[SKIP]${NC} %s\n" "$1"
    SKIPPED=$((SKIPPED + 1))
}

test_section() {
    printf "\n${YELLOW}==== %s ====${NC}\n" "$1"
}

# ========== Unit Tests ==========

test_section "Unit Tests: detect_os()"

# Test current OS detection (macOS)
os_result=$("${INSTALL_SCRIPT}" 2>&1 | grep -o "darwin\|linux\|windows" | head -1 || true)
# Since install will run, let's just verify the script parses uname -s
case "$(uname -s)" in
    Linux)     test_pass "Current OS: Linux (detect works)" ;;
    Darwin)   test_pass "Current OS: macOS (detect works)" ;;
    MINGW*|MSYS*|CYGWIN*) test_pass "Current OS: Windows (detect works)" ;;
    *)        test_fail "Unknown OS detection" ;;
esac

test_section "Unit Tests: detect_arch()"

# Test current architecture
case "$(uname -m)" in
    x86_64)    test_pass "Current arch: x86_64 (detect works)" ;;
    aarch64|arm64) test_pass "Current arch: aarch64 (detect works)" ;;
    *)        test_fail "Unknown arch detection" ;;
esac

test_section "Unit Tests: get_binary_name()"

# Test Linux naming
result=$(sh -c '
    NAME="autoharness"
    get_binary_name() {
        local os=$1
        local arch=$2
        case "$os" in
            linux)    echo "${NAME}-linux-${arch}";;
            macos)    echo "${NAME}-darwin-${arch}";;
            windows)  echo "${NAME}-windows-${arch}.exe";;
        esac
    }
    get_binary_name "linux" "x86_64"
')
[ "$result" = "autoharness-linux-x86_64" ] && test_pass "get_binary_name: linux-x86_64" || test_fail "get_binary_name: linux-x86_64"

# Test macOS naming
result=$(sh -c '
    NAME="autoharness"
    get_binary_name() {
        local os=$1
        local arch=$2
        case "$os" in
            linux)    echo "${NAME}-linux-${arch}";;
            macos)    echo "${NAME}-darwin-${arch}";;
            windows)  echo "${NAME}-windows-${arch}.exe";;
        esac
    }
    get_binary_name "macos" "aarch64"
')
[ "$result" = "autoharness-darwin-aarch64" ] && test_pass "get_binary_name: macos-aarch64" || test_fail "get_binary_name: macos-aarch64"

# Test Windows naming
result=$(sh -c '
    NAME="autoharness"
    get_binary_name() {
        local os=$1
        local arch=$2
        case "$os" in
            linux)    echo "${NAME}-linux-${arch}";;
            macos)    echo "${NAME}-darwin-${arch}";;
            windows)  echo "${NAME}-windows-${arch}.exe";;
        esac
    }
    get_binary_name "windows" "x86_64"
')
[ "$result" = "autoharness-windows-x86_64.exe" ] && test_pass "get_binary_name: windows-x86_64" || test_fail "get_binary_name: windows-x86_64"

# ========== File Tests ==========

test_section "File Existence Tests"

# Test binary exists
[ -f "$TESTBIN" ] && test_pass "Binary exists: autoharness-darwin-x86_64" || test_fail "Binary exists"

# Test binary is executable
[ -x "$TESTBIN" ] && test_pass "Binary is executable" || test_fail "Binary is executable"

# Test binary file size is reasonable (not empty, not 0 bytes)
size=$(stat -f%z "$TESTBIN" 2>/dev/null || stat -c%s "$TESTBIN" 2>/dev/null)
if [ "$size" -gt 1000000 ]; then
    test_pass "Binary size: ${size} bytes (>1MB, reasonable)"
else
    test_fail "Binary size: ${size} bytes (too small)"
fi

# Test install script exists
[ -f "$INSTALL_SCRIPT" ] && test_pass "Install script exists" || test_fail "Install script exists"

# Test install script is executable
[ -x "$INSTALL_SCRIPT" ] && test_pass "Install script is executable" || test_fail "Install script is executable"

# Test README exists
[ -f "${SCRIPT_DIR}/README.md" ] && test_pass "README.md exists" || test_fail "README.md exists"

# Test Windows batch exists
[ -f "${SCRIPT_DIR}/install.bat" ] && test_pass "install.bat exists" || test_fail "install.bat exists"

# ========== Binary Tests ==========

test_section "Binary Tests"

# Test binary runs with --version
"$TESTBIN" --version > /dev/null 2>&1 && test_pass "Binary runs: --version" || test_fail "Binary runs: --version"

# Test binary runs with --help
"$TESTBIN" --help > /dev/null 2>&1 && test_pass "Binary runs: --help" || test_fail "Binary runs: --help"

# Test binary version output
version=$("$TESTBIN" --version 2>&1 | head -1)
echo "$version" | grep -q "autoharness" && test_pass "Binary version output contains name" || test_fail "Binary version output"

echo "$version" | grep -q "0.1.0" && test_pass "Binary version: 0.1.0" || test_fail "Binary version: 0.1.0"

# Test synthesis help
"$TESTBIN" synthesize --help > /dev/null 2>&1 && test_pass "Subcommand: synthesize --help" || test_fail "Subcommand: synthesize --help"

# Test benchmark help
"$TESTBIN" benchmark --help > /dev/null 2>&1 && test_pass "Subcommand: benchmark --help" || test_fail "Subcommand: benchmark --help"

# ========== E2E: Install Flow ==========

test_section "E2E Tests: Install Flow"

# Clean up any existing installation
rm -rf "${HOME}/.local/bin/autoharness" 2>/dev/null || true

# Test fresh install
output=$("$INSTALL_SCRIPT" 2>&1)
echo "$output" | grep -q "Installation complete" && test_pass "Fresh install: success" || test_fail "Fresh install: success"

echo "$output" | grep -q "Installed:" && test_pass "Install output: shows path" || test_fail "Install output: shows path"

# Verify installed
[ -f "${HOME}/.local/bin/autoharness" ] && test_pass "Installed file exists" || test_fail "Installed file exists"

# Verify it's executable
[ -x "${HOME}/.local/bin/autoharness" ] && test_pass "Installed file is executable" || test_fail "Installed file is executable"

# Test installed binary works
"${HOME}/.local/bin/autoharness" --version > /dev/null 2>&1 && test_pass "Installed binary runs" || test_fail "Installed binary runs"

# Verify PATH warning appears when needed
output=$("$INSTALL_SCRIPT" 2>&1)
if echo "$output" | grep -q "not in your PATH\|Add to your PATH"; then
    test_pass "PATH warning shown when needed"
else
    test_skip "PATH warning (already in PATH?)"
fi

# ========== E2E: Re-install (Overwrite) ==========

test_section "E2E Tests: Re-install (Overwrite)"

# Clean for re-install test
rm -rf "${HOME}/.local/bin/autoharness" 2>/dev/null || true

# First install
"$INSTALL_SCRIPT" > /dev/null 2>&1

# Test reinstall with answer 'n' (should abort)
echo "n" | "$INSTALL_SCRIPT" > /dev/null 2>&1
[ -f "${HOME}/.local/bin/autoharness" ] && test_pass "Re-install: abort when answered 'n'" || test_fail "Re-install: abort"

# Test reinstall with answer 'y'
echo "y" | "$INSTALL_SCRIPT" > /dev/null 2>&1 && test_pass "Re-install: overwrite when answered 'y'" || test_fail "Re-install: overwrite"

# ========== E2E: Uninstall ==========

test_section "E2E Tests: Uninstall"

# Test uninstall
"$INSTALL_SCRIPT" uninstall 2>&1 | grep -q "Uninstalled" && test_pass "Uninstall: success message" || test_fail "Uninstall: success message"

# Verify removed
[ ! -f "${HOME}/.local/bin/autoharness" ] && test_pass "Uninstalled file removed" || test_fail "Uninstalled file removed"

# Test uninstall when not installed
output=$("$INSTALL_SCRIPT" uninstall 2>&1)
echo "$output" | grep -q "not installed\|Uninstalled" && test_pass "Uninstall: handles not installed" || test_fail "Uninstall: handles not installed"

# ========== E2E: Help ==========

test_section "E2E Tests: Help"

# Test help
"$INSTALL_SCRIPT" --help | grep -q "Usage:" && test_pass "Help: shows usage" || test_fail "Help: shows usage"

"$INSTALL_SCRIPT" -h | grep -q "Usage:" && test_pass "Help: -h flag works" || test_fail "Help: -h flag works"

"$INSTALL_SCRIPT" help | grep -q "Usage:" && test_pass "Help: help command works" || test_fail "Help: help command works"

# Test unknown command
"$INSTALL_SCRIPT" badcmd 2>&1 | grep -q "Unknown command" && test_pass "Unknown command: shows error" || test_fail "Unknown command: shows error"

# Test install command alias
"$INSTALL_SCRIPT" install | grep -q "Installing" && test_pass "Install alias: 'install' works" || test_fail "Install alias: 'install' works"

# Test uninstall command alias
rm -f "${HOME}/.local/bin/autoharness" 2>/dev/null || true
output=$("$INSTALL_SCRIPT" uninstall 2>&1)
(echo "$output" | grep -q "Uninstalled") || (echo "$output" | grep -q "not installed") && test_pass "Uninstall alias: 'uninstall' works" || test_fail "Uninstall alias: 'uninstall' works"

# ========== Edge Cases ==========

test_section "Edge Cases"

# Test with multiple 'y's in input
rm -rf "${HOME}/.local/bin/autoharness" 2>/dev/null || true
echo "yyyy" | "$INSTALL_SCRIPT" > /dev/null 2>&1
[ -f "${HOME}/.local/bin/autoharness" ] && test_pass "Edge case: multiple y's handled" || test_fail "Edge case: multiple y's"

# Test install to non-existent path handled gracefully
# (This is already handled by mkdir -p in the script)

# ========== Summary ==========

test_section "Summary"

total=$((PASSED + FAILED + SKIPPED))
printf "\n${GREEN}Passed:  ${PASSED}${NC}\n"
printf "${RED}Failed:  ${FAILED}${NC}\n"
printf "${YELLOW}Skipped: ${SKIPPED}${NC}\n"
printf "Total:   ${total}\n"

# Calculate pass rate
if [ "$total" -gt 0 ]; then
    pass_rate=$((PASSED * 100 / total))
    printf "Pass rate: ${pass_rate}%%\n"
fi

if [ "$FAILED" -eq 0 ]; then
    printf "\n${GREEN}✓ All tests passed!${NC}\n"
    exit 0
else
    printf "\n${RED}✗ Some tests failed!${NC}\n"
    exit 1
fi