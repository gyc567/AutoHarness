#!/bin/sh
# -*- mode: sh; -*-
# AutoHarness One-Click Installer v0.1.0
# Supports: Linux (x86_64), macOS (x86_64 + ARM), Windows (x86_64)
#
# One-line install (recommended):
#   curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/main/install/install.sh | bash
#
# Or with specific version:
#   curl -fsSL https://raw.githubusercontent.com/gyc567/AutoHarness/v0.1.0/install/install.sh | bash
#
# Local install:
#   ./install.sh
#   ./install.sh uninstall

set -e

# ========== CONSTANTS ==========
NAME="autoharness"
VERSION="0.1.0"
INSTALL_DIR="${HOME}/.local/bin"
REPO_OWNER="gyc567"
REPO_NAME="AutoHarness"

# CDN/Release URLs
RAW_BASE="https://raw.githubusercontent.com/${REPO_OWNER}/${REPO_NAME}"
RELEASE_BASE="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases"

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

get_binary_name() {
    local os=$1
    local arch=$2

    case "$os" in
        linux)    echo "${NAME}-linux-${arch}";;
        macos)    echo "${NAME}-darwin-${arch}";;
        windows)  echo "${NAME}-windows-${arch}.exe";;
    esac
}

# ========== DOWNLOAD ==========
download_binary() {
    local os=$1
    local arch=$2
    local dest=$3

    local bin_name=$(get_binary_name "$os" "$arch")
    local temp_dir
    temp_dir=$(mktemp -d)
    local temp_file="${temp_dir}/${bin_name}"

    log_step "Downloading ${bin_name}..."

    # Try multiple download sources in order of preference

    # 1. Try GitHub raw (for development/testing)
    local raw_url="${RAW_BASE}/main/install/${bin_name}"
    if curl -fsSL --retry 3 --retry-delay 2 -o "$temp_file" "$raw_url" 2>/dev/null; then
        if [ -s "$temp_file" ]; then
            mv "$temp_file" "$dest"
            rm -rf "$temp_dir"
            return 0
        fi
    fi
    rm -f "$temp_file"

    # 2. Try jsDelivr CDN (recommended for production)
    local jsdelivr_url="https://cdn.jsdelivr.net/gh/${REPO_OWNER}/${REPO_NAME}@main/install/${bin_name}"
    if curl -fsSL --retry 3 --retry-delay 2 -o "$temp_file" "$jsdelivr_url" 2>/dev/null; then
        if [ -s "$temp_file" ]; then
            mv "$temp_file" "$dest"
            rm -rf "$temp_dir"
            return 0
        fi
    fi
    rm -f "$temp_file"

    # 3. Try GitHub Releases (for stable versions)
    local release_url="${RELEASE_BASE}/download/v${VERSION}/${bin_name}"
    if curl -fsSL --retry 3 --retry-delay 2 -L -o "$temp_file" "$release_url" 2>/dev/null; then
        if [ -s "$temp_file" ]; then
            mv "$temp_file" "$dest"
            rm -rf "$temp_dir"
            return 0
        fi
    fi
    rm -f "$temp_file"
    rm -rf "$temp_dir"

    return 1
}

# ========== INSTALL ==========
do_install() {
    local os=$(detect_os)
    local arch=$(detect_arch)

    # Determine destination path
    local dest="${INSTALL_DIR}/${NAME}"
    [ "$os" = "windows" ] && dest="${dest}.exe"

    # Check if already installed
    if [ -f "$dest" ]; then
        log_warn "${NAME} is already installed at ${dest}"
        printf "Overwrite? [y/N]: "
        read -r answer
        if [ "$answer" != "y" ] && [ "$answer" != "Y" ]; then
            log_info "Aborted."
            exit 0
        fi
    fi

    # Create install directory
    log_step "Creating install directory..."
    mkdir -p "$INSTALL_DIR"

    # Try remote download first, fallback to local
    if ! download_binary "$os" "$arch" "$dest" 2>/dev/null; then
        # Fallback: try local binary
        local script_dir="$(cd "$(dirname "$0")" && pwd)"
        local local_bin="${script_dir}/$(get_binary_name "$os" "$arch")"

        if [ -f "$local_bin" ]; then
            log_step "Using local binary..."
            cp "$local_bin" "$dest"
        else
            log_error "No suitable binary found for ${os}-${arch}"
            log_error "Please build from source: cargo build --release"
            exit 1
        fi
    fi

    chmod +x "$dest"

    log_info "Installed: ${dest}"

    # Check PATH
    case ":$PATH:" in
        *:"${INSTALL_DIR}":*) ;;
        *)
            log_warn "${INSTALL_DIR} is not in your PATH."
            log_info "Add to your shell profile (.bashrc/.zshrc):"
            echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
            ;;
    esac

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
  curl -fsSL https://raw.githubusercontent.com/${REPO_OWNER}/${REPO_NAME}/main/install/install.sh | bash
  curl -fsSL https://cdn.jsdelivr.net/gh/${REPO_OWNER}/${REPO_NAME}/main/install/install.sh | bash

  $(basename "$0")           Install ${NAME}
  $(basename "$0") install    Install ${NAME} (same as above)
  $(basename "$0") uninstall  Uninstall ${NAME}
  $(basename "$0") --help     Show this help

Install Location: ${INSTALL_DIR}

Examples:
  # One-line install (recommended)
  curl -fsSL https://raw.githubusercontent.com/${REPO_OWNER}/${REPO_NAME}/main/install/install.sh | bash

  # Or use jsDelivr CDN
  curl -fsSL https://cdn.jsdelivr.net/gh/${REPO_OWNER}/${REPO_NAME}/main/install/install.sh | bash

  # Local install
  ./install.sh
  ./install.sh uninstall
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
