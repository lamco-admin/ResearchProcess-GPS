#!/bin/bash
# Setup script to ensure Rust environment is available
# This fixes the issue where cargo isn't found in some sessions

# Source the Rust environment
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "✅ Rust environment loaded"
    echo "Cargo: $(which cargo)"
    echo "Rustc: $(which rustc)"
else
    echo "❌ Rust environment not found at $HOME/.cargo/env"
    echo "Please install Rust: https://rustup.rs/"
    exit 1
fi

# Verify cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ cargo command not found after sourcing environment"
    exit 1
fi

echo "🚀 Ready to use Rust tools!"