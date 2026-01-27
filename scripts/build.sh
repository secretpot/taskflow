#!/bin/bash
set -e

# Unified build and distribute script for taskflow
# Usage: 
#   ./scripts/build.sh          - Build for current host only
#   ./scripts/build.sh --all    - Build for all supported platforms

BUILD_ALL=false
if [[ "$1" == "--all" ]]; then
    BUILD_ALL=true
fi

# Detect host info
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

case "$OS_TYPE" in
    darwin*)  HOST_OS="macos" ;;
    linux*)   HOST_OS="linux" ;;
    msys*|cygwin*|mingw*) HOST_OS="windows" ;;
    *)        HOST_OS="unknown" ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64) HOST_ARCH="amd64" ;;
    arm64|aarch64) HOST_ARCH="arm64" ;;
    *)             HOST_ARCH="unknown" ;;
esac

package() {
    TARGET_NAME=$1
    TARGET_PLATFORM=$2 # e.g. macos_arm64, windows_amd64
    BIN_SRC=$3         # relative to project root
    IS_WINDOWS=$4

    DIST_DIR="dist/${TARGET_PLATFORM}/taskflow"
    echo "📦 Packaging for ${TARGET_PLATFORM} -> ${DIST_DIR}"
    
    mkdir -p "${DIST_DIR}/bin"
    
    if [ "$IS_WINDOWS" = true ]; then
        cp "$BIN_SRC" "${DIST_DIR}/bin/taskflow.exe"
    else
        cp "$BIN_SRC" "${DIST_DIR}/bin/taskflow"
    fi
    
    cp "assets/SKILL.md" "${DIST_DIR}/SKILL.md"
}

build_and_package() {
    TARGET=$1          # rust target triple
    PLATFORM_DIR=$2    # e.g. macos_arm64
    IS_WIN=$3          # true/false
    
    echo "🚀 Building for $TARGET..."
    cargo build --release --target "$TARGET"
    
    BIN_NAME="taskflow"
    if [ "$IS_WIN" = true ]; then BIN_NAME="taskflow.exe"; fi
    
    package "taskflow" "$PLATFORM_DIR" "target/$TARGET/release/$BIN_NAME" "$IS_WIN"
}

# Clean dist
rm -rf dist

if [ "$BUILD_ALL" = true ]; then
    echo "--- Building all platforms ---"
    build_and_package "aarch64-apple-darwin" "macos_arm64" false
    build_and_package "x86_64-apple-darwin" "macos_amd64" false
    build_and_package "x86_64-unknown-linux-gnu" "linux_amd64" false
    build_and_package "x86_64-pc-windows-msvc" "windows_amd64" true
else
    echo "--- Building for current host ($HOST_OS-$HOST_ARCH) ---"
    cargo build --release
    
    BIN_EXT=""
    IS_WIN=false
    if [ "$HOST_OS" = "windows" ]; then BIN_EXT=".exe"; IS_WIN=true; fi
    
    package "taskflow" "${HOST_OS}_${HOST_ARCH}" "target/release/taskflow${BIN_EXT}" "$IS_WIN"
fi

echo "✅ Build and distribution complete!"
