#!/bin/bash
set -e

# Taskflow Installation Script for Unix-like systems (macOS/Linux)
# This script installs taskflow to /usr/local/bin

OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

case "$OS_TYPE" in
    darwin*)  OS="macos" ;;
    linux*)   OS="linux" ;;
    *)        echo "❌ Unsupported OS: $OS_TYPE"; exit 1 ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64) ARCH="amd64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *)             echo "❌ Unsupported Architecture: $ARCH_TYPE"; exit 1 ;;
esac

PLATFORM="${OS}_${ARCH}"
BINARY_SRC="dist/${PLATFORM}/taskflow/bin/taskflow"
TARGET_DIR="/usr/local/bin"
TARGET_PATH="${TARGET_DIR}/taskflow"

echo "🚀 Installing taskflow for ${PLATFORM}..."

if [ ! -f "$BINARY_SRC" ]; then
    echo "❌ Binary not found at $BINARY_SRC"
    echo "Please run './scripts/build.sh' (or './scripts/build.sh --all') first."
    exit 1
fi

# Use sudo if necessary
if [ -w "$TARGET_DIR" ]; then
    cp "$BINARY_SRC" "$TARGET_PATH"
    chmod +x "$TARGET_PATH"
else
    echo "🔑 Requesting sudo permission to install to $TARGET_DIR..."
    sudo cp "$BINARY_SRC" "$TARGET_PATH"
    sudo chmod +x "$TARGET_PATH"
fi

echo "✅ Taskflow installed successfully to $TARGET_PATH"
echo "You can now run 'taskflow' from anywhere!"
