#!/usr/bin/env bash
# macOS-specific installer for cascolor
set -e

REPO="casapps/cascolor"
INSTALL_DIR="/usr/local/bin"
BIN_NAME="cascolor"
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) BINARY_NAME="${BIN_NAME}-macos-x86_64" ;;
    arm64) BINARY_NAME="${BIN_NAME}-macos-aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

echo "Installing cascolor for macOS ($ARCH)..."

LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST/$BINARY_NAME"

TMP_FILE=$(mktemp)
curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"
chmod +x "$TMP_FILE"

sudo mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"
echo "✓ Installed to $INSTALL_DIR/$BIN_NAME"
