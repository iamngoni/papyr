#!/bin/bash
#
# papyr_core build script
#

set -e

echo "Building papyr_core..."

# Detect platform
PLATFORM=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    FEATURES="ica"
    EXT="dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    FEATURES="sane"
    EXT="so"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows"
    FEATURES="wia"
    EXT="dll"
else
    echo "Unsupported platform: $OSTYPE"
    exit 1
fi

echo "Platform: $PLATFORM"
echo "Features: $FEATURES"

# Build release version
echo "Building release version..."
cargo build --release --features "$FEATURES"

# Copy header and library to output directory
OUTPUT_DIR="dist/$PLATFORM"
mkdir -p "$OUTPUT_DIR"

cp "include/papyr_core.h" "$OUTPUT_DIR/"

if [[ "$PLATFORM" == "windows" ]]; then
    cp "target/release/papyr_core.dll" "$OUTPUT_DIR/"
else
    cp "target/release/libpapyr_core.$EXT" "$OUTPUT_DIR/"
fi

echo "Build complete! Output in: $OUTPUT_DIR"

# Test the build
echo "Testing C FFI..."
cd test_c

if [[ "$PLATFORM" == "macos" ]]; then
    gcc -o test_ffi test_ffi.c -L../target/release -lpapyr_core
    DYLD_LIBRARY_PATH=../target/release ./test_ffi
elif [[ "$PLATFORM" == "linux" ]]; then
    gcc -o test_ffi test_ffi.c -L../target/release -lpapyr_core
    LD_LIBRARY_PATH=../target/release ./test_ffi
elif [[ "$PLATFORM" == "windows" ]]; then
    # Windows testing would need different approach
    echo "Windows C testing not implemented in this script"
fi

echo "✓ Build and test complete!"
