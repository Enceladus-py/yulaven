#!/bin/bash

# Configuration
TARGET_ARCH="arm64-v8a"
RUST_TARGET="aarch64-linux-android"
OUTPUT_DIR="android/app/src/main/jniLibs"

# Check if cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null
then
    echo "cargo-ndk could not be found. Installing it..."
    cargo install cargo-ndk
fi

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR/$TARGET_ARCH"

echo "Building for $TARGET_ARCH..."

# Build using cargo ndk
cargo ndk -t "$TARGET_ARCH" -o "$OUTPUT_DIR" build --lib --link-libcxx-shared

echo "Build complete. Shared libraries are in $OUTPUT_DIR"

echo "Building APK..."
cd android && ./gradlew assembleDebug && cd ..

echo "APK build complete."
