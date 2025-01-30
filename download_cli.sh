#!/usr/bin/env bash
set -eu

##############################################################################
# snake CLI Install Script
#
# This script downloads the latest stable 'snake' CLI binary from GitHub releases
# and installs it to your system.
#
# Supported OS: macOS (darwin), Linux
# Supported Architectures: x86_64, arm64
#
# Usage:
#   curl -fsSL https://github.com/block/snake/releases/download/stable/download_cli.sh | bash
#
# Environment variables:
#   snake_BIN_DIR  - Directory to which snake will be installed (default: $HOME/.local/bin)
#   snake_PROVIDER - Optional: provider for snake
#   snake_MODEL    - Optional: model for snake
#   CANARY         - Optional: if set to "true", downloads from canary release instead of stable
#   ** other provider specific environment variables (eg. DATABRICKS_HOST)
##############################################################################

# --- 1) Check for curl ---
if ! command -v curl >/dev/null 2>&1; then
  echo "Error: 'curl' is required to download snake. Please install curl and try again."
  exit 1
fi

# --- 2) Variables ---
REPO="block/snake"
OUT_FILE="snake"
snake_BIN_DIR="${snake_BIN_DIR:-"$HOME/.local/bin"}"
RELEASE="${CANARY:-false}"
RELEASE_TAG="$([[ "$RELEASE" == "true" ]] && echo "canary" || echo "stable")"

# --- 3) Detect OS/Architecture ---
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux|darwin) ;;
  *) 
    echo "Error: Unsupported OS '$OS'. snake currently only supports Linux and macOS."
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64)
    ARCH="x86_64"
    ;;
  arm64|aarch64)
    # Some systems use 'arm64' and some 'aarch64' – standardize to 'aarch64'
    ARCH="aarch64"
    ;;
  *)
    echo "Error: Unsupported architecture '$ARCH'."
    exit 1
    ;;
esac

# Build the filename and URL for the stable release
if [ "$OS" = "darwin" ]; then
  FILE="snake-$ARCH-apple-darwin.tar.bz2"
else
  FILE="snake-$ARCH-unknown-linux-gnu.tar.bz2"
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$RELEASE_TAG/$FILE"

# --- 4) Download & extract 'snake' binary ---
echo "Downloading $RELEASE_TAG release: $FILE..."
if ! curl -sLf "$DOWNLOAD_URL" --output "$FILE"; then
  echo "Error: Failed to download $DOWNLOAD_URL"
  exit 1
fi

echo "Extracting $FILE..."
tar -xjf "$FILE"
rm "$FILE" # clean up the downloaded tarball

# Make binaries executable
chmod +x snake

# --- 5) Install to $snake_BIN_DIR ---
if [ ! -d "$snake_BIN_DIR" ]; then
  echo "Creating directory: $snake_BIN_DIR"
  mkdir -p "$snake_BIN_DIR"
fi

echo "Moving snake to $snake_BIN_DIR/$OUT_FILE"
mv snake "$snake_BIN_DIR/$OUT_FILE"

# --- 6) Configure snake (Optional) ---
echo ""
echo "Configuring snake"
echo ""
"$snake_BIN_DIR/$OUT_FILE" configure

# --- 7) Check PATH and give instructions if needed ---
if [[ ":$PATH:" != *":$snake_BIN_DIR:"* ]]; then
  echo ""
  echo "Warning: snake installed, but $snake_BIN_DIR is not in your PATH."
  echo "Add it to your PATH by editing ~/.bashrc, ~/.zshrc, or similar:"
  echo "    export PATH=\"$snake_BIN_DIR:\$PATH\""
  echo "Then reload your shell (e.g. 'source ~/.bashrc', 'source ~/.zshrc') to apply changes."
  echo ""
fi
