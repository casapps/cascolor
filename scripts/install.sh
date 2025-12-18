#!/usr/bin/env bash
set -e

# Universal installer for cascolor
# Detects platform and downloads appropriate binary

REPO="casapps/cascolor"
INSTALL_DIR="/usr/local/bin"
BIN_NAME="cascolor"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "cascolor installer"
echo "=================="
echo ""

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}Error: Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

case "$OS" in
    linux)
        BINARY_NAME="${BIN_NAME}-linux-${ARCH}"
        ;;
    darwin)
        OS="macos"
        BINARY_NAME="${BIN_NAME}-macos-${ARCH}"
        ;;
    freebsd)
        BINARY_NAME="${BIN_NAME}-freebsd-${ARCH}"
        ;;
    *)
        echo -e "${RED}Error: Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

echo "Detected platform: $OS-$ARCH"
echo "Binary: $BINARY_NAME"
echo ""

# Get latest release
echo "Fetching latest release..."
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST" ]; then
    echo -e "${RED}Error: Could not fetch latest release${NC}"
    exit 1
fi

echo -e "${GREEN}Latest version: $LATEST${NC}"
echo ""

# Download binary
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST/$BINARY_NAME"
echo "Downloading from: $DOWNLOAD_URL"

TMP_FILE=$(mktemp)
if ! curl -sL "$DOWNLOAD_URL" -o "$TMP_FILE"; then
    echo -e "${RED}Error: Failed to download binary${NC}"
    rm -f "$TMP_FILE"
    exit 1
fi

# Make executable
chmod +x "$TMP_FILE"

# Install
echo ""
echo "Installing to $INSTALL_DIR/$BIN_NAME..."

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"
else
    echo -e "${YELLOW}Requires sudo for installation...${NC}"
    sudo mv "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"
fi

echo -e "${GREEN}✓ Installation complete!${NC}"
echo ""
echo "Run: $BIN_NAME --help"
