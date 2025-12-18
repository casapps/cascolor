#!/usr/bin/env bash
# Linux-specific installer for cascolor
set -e

REPO="casapps/cascolor"
INSTALL_DIR="/usr/local/bin"
BIN_NAME="cascolor"
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) BINARY_NAME="${BIN_NAME}-linux-x86_64" ;;
    aarch64) BINARY_NAME="${BIN_NAME}-linux-aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

echo "Installing cascolor for Linux ($ARCH)..."

LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST/$BINARY_NAME"

TMP_FILE=$(mktemp)
curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"
chmod +x "$TMP_FILE"

sudo mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"
echo "✓ Installed to $INSTALL_DIR/$BIN_NAME"
