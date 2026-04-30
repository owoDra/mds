#!/bin/sh
# mds installer script
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh
#   curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh -s -- --version 0.3.0
set -e

REPO="owo-x-project/owox-mds"
INSTALL_DIR="${MDS_INSTALL_DIR:-$HOME/.local/bin}"
VERSION=""

# Parse arguments
while [ $# -gt 0 ]; do
  case "$1" in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --install-dir)
      INSTALL_DIR="$2"
      shift 2
      ;;
    --help)
      echo "mds installer"
      echo ""
      echo "Usage: curl -fsSL https://raw.githubusercontent.com/$REPO/main/install.sh | sh -s -- [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --version VERSION    Install a specific version (default: latest)"
      echo "  --install-dir DIR    Installation directory (default: ~/.local/bin)"
      echo "  --help               Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Detect platform
detect_platform() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Linux)   OS="linux" ;;
    Darwin)  OS="darwin" ;;
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
    *)
      echo "Error: Unsupported operating system: $OS"
      exit 1
      ;;
  esac

  case "$ARCH" in
    x86_64|amd64)  ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)
      echo "Error: Unsupported architecture: $ARCH"
      exit 1
      ;;
  esac

  if [ "$OS" = "windows" ]; then
    TARGET="${ARCH}-pc-windows-msvc"
    EXT=".exe"
  elif [ "$OS" = "darwin" ]; then
    TARGET="${ARCH}-apple-darwin"
    EXT=""
  else
    TARGET="${ARCH}-unknown-linux-gnu"
    EXT=""
  fi
}

# Get latest version from GitHub API
get_latest_version() {
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/'
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/'
  else
    echo "Error: curl or wget is required"
    exit 1
  fi
}

# Download and install
install() {
  detect_platform

  if [ -z "$VERSION" ]; then
    echo "Fetching latest version..."
    VERSION="$(get_latest_version)"
    if [ -z "$VERSION" ]; then
      echo "Error: Could not determine latest version"
      exit 1
    fi
  fi

  echo "Installing mds v${VERSION} for ${TARGET}..."

  ARCHIVE_NAME="mds-v${VERSION}-${TARGET}.tar.gz"
  DOWNLOAD_URL="https://github.com/$REPO/releases/download/v${VERSION}/${ARCHIVE_NAME}"

  # Create temp directory
  TMP_DIR="$(mktemp -d)"
  trap 'rm -rf "$TMP_DIR"' EXIT

  # Download
  echo "Downloading ${DOWNLOAD_URL}..."
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE_NAME"
  elif command -v wget >/dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$TMP_DIR/$ARCHIVE_NAME"
  fi

  # Extract
  tar -xzf "$TMP_DIR/$ARCHIVE_NAME" -C "$TMP_DIR"

  # Install binaries
  mkdir -p "$INSTALL_DIR"
  for binary in mds mds-lsp; do
    if [ -f "$TMP_DIR/${binary}${EXT}" ]; then
      mv "$TMP_DIR/${binary}${EXT}" "$INSTALL_DIR/${binary}${EXT}"
      chmod +x "$INSTALL_DIR/${binary}${EXT}"
    fi
  done

  echo ""
  echo "Installed mds v${VERSION} to ${INSTALL_DIR}/mds"
  echo ""

  # Check if install dir is in PATH
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      echo "Note: Add ${INSTALL_DIR} to your PATH:"
      echo ""
      echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
      echo ""
      ;;
  esac

  echo "Verify installation:"
  echo "  mds --version"
}

install
