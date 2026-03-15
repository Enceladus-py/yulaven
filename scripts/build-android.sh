#!/bin/bash
set -e

# Configuration
TARGET_ARCH="arm64-v8a"
RUST_TARGET="aarch64-linux-android"
OUTPUT_DIR="android/app/src/main/jniLibs"

DEPLOY=false
RELEASE=false

# Parse arguments
for arg in "$@"; do
    if [ "$arg" == "--deploy" ]; then
        DEPLOY=true
    fi
    if [ "$arg" == "--release" ]; then
        RELEASE=true
    fi
done

# Check if cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null
then
    echo "cargo-ndk could not be found. Installing it..."
    cargo install cargo-ndk
fi

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR/$TARGET_ARCH"

export CARGO_TARGET_DIR="target/android"

# Configure build flags based on mode
if [ "$RELEASE" = true ]; then
    echo "Building for $TARGET_ARCH in RELEASE mode..."
    CARGO_FLAGS="--release"
    GRADLE_TASK="assembleRelease"
else
    echo "Building for $TARGET_ARCH in DEBUG mode..."
    CARGO_FLAGS=""
    GRADLE_TASK="assembleDebug"
fi

# Build using cargo ndk
# Note: $CARGO_FLAGS is deliberately unquoted so that if it's empty, bash ignores it safely
cargo ndk -t "$TARGET_ARCH" -o "$OUTPUT_DIR" build --lib --link-libcxx-shared $CARGO_FLAGS

echo "Build complete. Shared libraries are in $OUTPUT_DIR"

echo "Building APK..."
cd android && ./gradlew $GRADLE_TASK && cd ..

echo "APK build complete."

if [ "$DEPLOY" = true ]; then
    echo "Running deployment script..."
    # You might want to pass the release flag to your deploy script too!
    if [ "$RELEASE" = true ]; then
        ./scripts/deploy-android.sh --release
    else
        ./scripts/deploy-android.sh
    fi
fi