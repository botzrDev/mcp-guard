#!/bin/bash
set -e

# mcp-guard installer
# Usage: curl -fsSL https://raw.githubusercontent.com/botzrdev/mcp-guard/main/install.sh | bash

VERSION="${MCP_GUARD_VERSION:-latest}"
INSTALL_DIR="${MCP_GUARD_INSTALL_DIR:-/usr/local/bin}"

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    linux)
        case "$ARCH" in
            x86_64) PLATFORM="x86_64-linux" ;;
            aarch64) PLATFORM="aarch64-linux" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    darwin)
        case "$ARCH" in
            x86_64) PLATFORM="x86_64-darwin" ;;
            arm64) PLATFORM="aarch64-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please install manually from https://github.com/botzrdev/mcp-guard/releases"
        exit 1
        ;;
esac

# Get download URL
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/botzrdev/mcp-guard/releases/latest/download/mcp-guard-${PLATFORM}.tar.gz"
else
    DOWNLOAD_URL="https://github.com/botzrdev/mcp-guard/releases/download/v${VERSION}/mcp-guard-${PLATFORM}.tar.gz"
fi

echo "Installing mcp-guard for ${PLATFORM}..."
echo "Download URL: ${DOWNLOAD_URL}"

# Create temp directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download and extract
curl -fsSL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"

# Install
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/mcp-guard" "$INSTALL_DIR/"
else
    echo "Installing to $INSTALL_DIR requires sudo..."
    sudo mv "$TMP_DIR/mcp-guard" "$INSTALL_DIR/"
fi

chmod +x "$INSTALL_DIR/mcp-guard"

echo ""
echo "mcp-guard installed successfully!"
echo ""
echo "Get started:"
echo "  mcp-guard init"
echo "  mcp-guard run"
echo ""
echo "Documentation: https://github.com/botzrdev/mcp-guard"
