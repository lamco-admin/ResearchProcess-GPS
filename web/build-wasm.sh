#!/bin/bash
# Build script for WASM module

set -e

echo "Building WASM module..."

cd "$(dirname "$0")/.."

# Build with wasm-pack
if command -v wasm-pack &> /dev/null; then
    wasm-pack build \
        --target web \
        --out-dir ../web/pkg \
        crates/rp-wasm

    echo "✓ WASM module built successfully!"
    echo "  Output: web/pkg/"
else
    echo "Error: wasm-pack not found!"
    echo "Install with: cargo install wasm-pack"
    exit 1
fi
