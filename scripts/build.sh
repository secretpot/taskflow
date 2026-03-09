#!/bin/bash
set -e

# Unified build and distribute script for taskflow
# Usage: 
#   ./scripts/build.sh          - Build for current host only
#   ./scripts/build.sh --all    - Build for all supported platforms
#   ./scripts/build.sh <target> - Build for a specific target (e.g., macos_arm64, linux_amd64, etc.)

BUILD_TARGET=$1

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
    TARGET_PLATFORM=$1 # e.g. macos_arm64, windows_amd64
    BIN_SRC=$2         # relative to project root
    IS_WINDOWS=$3

    DIST_DIR="dist/${TARGET_PLATFORM}/taskflow"
    echo "📦 Packaging for ${TARGET_PLATFORM} -> ${DIST_DIR}"
    
    mkdir -p "${DIST_DIR}/bin"
    mkdir -p "${DIST_DIR}/scripts"
    
    if [ "$IS_WINDOWS" = true ]; then
        cp "$BIN_SRC" "${DIST_DIR}/bin/taskflow.exe"
    else
        cp "$BIN_SRC" "${DIST_DIR}/bin/taskflow"
    fi
    
    cp "assets/SKILL.md" "${DIST_DIR}/SKILL.md"
    cp "scripts/install.sh" "${DIST_DIR}/scripts/install.sh"
    cp "scripts/install.bat" "${DIST_DIR}/scripts/install.bat"
}

build_and_package() {
    PLATFORM_NAME=$1   # e.g. macos_arm64
    TARGET=$2          # rust target triple
    IS_WIN=$3          # true/false
    
    echo "🚀 Building for $PLATFORM_NAME ($TARGET)..."
    cargo build --release --target "$TARGET"
    
    BIN_NAME="taskflow"
    if [ "$IS_WIN" = true ]; then BIN_NAME="taskflow.exe"; fi
    
    package "$PLATFORM_NAME" "target/$TARGET/release/$BIN_NAME" "$IS_WIN"
}

# Clean dist only if building all or if explicitly requested
if [[ "$BUILD_TARGET" == "--all" ]] || [[ -z "$BUILD_TARGET" ]]; then
    rm -rf dist
fi

case "$BUILD_TARGET" in
    "--all")
        echo "--- Building all platforms ---"
        build_and_package "macos_arm64" "aarch64-apple-darwin" false
        build_and_package "macos_amd64" "x86_64-apple-darwin" false
        build_and_package "linux_amd64" "x86_64-unknown-linux-gnu" false
        build_and_package "linux_arm64" "aarch64-unknown-linux-musl" false
        build_and_package "windows_amd64" "x86_64-pc-windows-gnu" true
        ;;
    "macos_arm64")
        build_and_package "macos_arm64" "aarch64-apple-darwin" false
        ;;
    "macos_amd64")
        build_and_package "macos_amd64" "x86_64-apple-darwin" false
        ;;
    "linux_amd64")
        build_and_package "linux_amd64" "x86_64-unknown-linux-gnu" false
        ;;
    "linux_arm64")
        build_and_package "linux_arm64" "aarch64-unknown-linux-musl" false
        ;;
    "windows_amd64")
        build_and_package "windows_amd64" "x86_64-pc-windows-gnu" true
        ;;
    "")
        echo "--- Building for current host ($HOST_OS-$HOST_ARCH) ---"
        cargo build --release
        
        BIN_EXT=""
        IS_WIN=false
        if [ "$HOST_OS" = "windows" ]; then BIN_EXT=".exe"; IS_WIN=true; fi
        
        package "${HOST_OS}_${HOST_ARCH}" "target/release/taskflow${BIN_EXT}" "$IS_WIN"
        ;;
    *)
        echo "❌ Unknown build target: $BUILD_TARGET"
        echo "Usage: ./scripts/build.sh [--all | macos_arm64 | macos_amd64 | linux_amd64 | linux_arm64 | windows_amd64]"
        exit 1
        ;;
esac

echo "✅ Build and distribution complete!"
