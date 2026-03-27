#!/bin/sh
# -*- mode: sh; -*-
# AutoHarness One-Click Installer v0.1.0
# Supports: Linux (x86_64), macOS (x86_64 + ARM), Windows (x86_64)
#
# Usage:
#   ./install.sh              # Install
#   ./install.sh uninstall    # Uninstall
#   ./install.sh --help       # Show help

set -e

# ========== CONSTANTS ==========
NAME="autoharness"
VERSION="0.1.0"
INSTALL_DIR="${HOME}/.local/bin"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ========== COLORS ==========
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { printf "${GREEN}[INFO]${NC} %s\n" "$1"; }
log_warn() { printf "${YELLOW}[WARN]${NC} %s\n" "$1"; }
log_error() { printf "${RED}[ERROR]${NC} %s\n" "$1" 1>&2; }
log_step() { printf "${BLUE}[STEP]${NC} %s\n" "$1"; }

# ========== DETECTION ==========
detect_os() {
    case "$(uname -s)" in
        Linux)     echo "linux";;
        Darwin)    echo "macos";;
        MINGW*|MSYS*|CYGWIN*) echo "windows";;
        *)         log_error "Unsupported OS: $(uname -s)"; exit 1;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64)    echo "x86_64";;
        aarch64|arm64) echo "aarch64";;
        *)         log_error "Unsupported architecture: $(uname -m)"; exit 1;;
    esac
}

# Fallback architecture for cross-platform compatibility
get_fallback_arch() {
    echo "x86_64"
}

get_binary_name() {
    local os=$1
    local arch=$2

    case "$os" in
        linux)    echo "${NAME}-linux-${arch}";;
        macos)    echo "${NAME}-darwin-${arch}";;
        windows)  echo "${NAME}-windows-${arch}.exe";;
    esac
}

# Find available binary (try native, fallback to x86_64)
find_binary() {
    local os=$1
    local arch=$2

    # Try native architecture first
    local native=$(get_binary_name "$os" "$arch")
    if [ -f "${SCRIPT_DIR}/${native}" ]; then
        echo "$native"
        return 0
    fi

    # Try fallback (x86_64)
    if [ "$arch" != "x86_64" ]; then
        local fallback=$(get_binary_name "$os" "x86_64")
        if [ -f "${SCRIPT_DIR}/${fallback}" ]; then
            log_warn "No native binary for ${arch}, trying ${fallback}" >&2
            echo "$fallback"
            return 0
        fi
    fi

    return 1
}

# ========== INSTALL ==========
do_install() {
    local os=$(detect_os)
    local arch=$(detect_arch)

    # Find available binary
    local bin_name
    if ! bin_name=$(find_binary "$os" "$arch"); then
        log_error "No suitable binary found for ${os}-${arch}"
        log_error "Available binaries in ${SCRIPT_DIR}:"
        ls -1 "${SCRIPT_DIR}/${NAME}"-* 2>/dev/null | grep -v '\.txt$' || true
        exit 1
    fi

    local src="${SCRIPT_DIR}/${bin_name}"

    # Determine destination path
    local dest="${INSTALL_DIR}/${NAME}"
    [ "$os" = "windows" ] && dest="${dest}.exe"

    # Check if already installed
    if [ -f "$dest" ]; then
        log_warn "${NAME} is already installed at ${dest}"
        printf "Overwrite? [y/N]: "
        read -r answer
        if ! [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
            log_info "Aborted."
            exit 0
        fi
    fi

    # Create install directory
    log_step "Creating install directory..."
    mkdir -p "$INSTALL_DIR"

    # Copy binary
    log_step "Installing ${bin_name}..."
    cp "$src" "$dest"
    chmod +x "$dest"

    log_info "Installed: ${dest}"

    # Check PATH
    if ! echo "$PATH" | grep -qF "$INSTALL_DIR"; then
        log_warn "${INSTALL_DIR} is not in your PATH."
        log_info "Add to your shell profile (.bashrc/.zshrc):"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi

    log_info "Installation complete!"
    log_info "Run '${NAME}' to get started."
}

# ========== UNINSTALL ==========
do_uninstall() {
    local os=$(detect_os)
    local dest="${INSTALL_DIR}/${NAME}"

    [ "$os" = "windows" ] && dest="${dest}.exe"

    if [ -f "$dest" ]; then
        rm -f "$dest"
        log_info "Uninstalled: ${dest}"
    else
        log_warn "${NAME} is not installed."
    fi

    # Optionally remove empty directory
    if [ -d "$INSTALL_DIR" ] && [ -z "$(ls -A "$INSTALL_DIR" 2>/dev/null)" ]; then
        rmdir "$INSTALL_DIR" 2>/dev/null || true
    fi
}

# ========== MAIN ==========
show_help() {
    cat <<EOF
AutoHarness One-Click Installer v${VERSION}

Usage:
  $(basename "$0")           Install ${NAME}
  $(basename "$0") install    Install ${NAME} (same as above)
  $(basename "$0") uninstall  Uninstall ${NAME}
  $(basename "$0") --help     Show this help

Install Location: ${INSTALL_DIR}

Examples:
  ./install.sh               # Install
  ./install.sh uninstall      # Remove
EOF
}

case "${1:-install}" in
    install|--install)
        do_install
        ;;
    uninstall|--uninstall)
        do_uninstall
        ;;
    -h|--help|help)
        show_help
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Run '$(basename "$0") --help' for usage."
        exit 1
        ;;
esac