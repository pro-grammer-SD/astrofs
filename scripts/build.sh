#!/bin/bash

echo "🦀 Building AstroFS..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust/Cargo found"
echo ""

# Build in release mode
echo "📦 Compiling in release mode (this may take a few minutes)..."
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🚀 Run the application with:"
    echo "   ./target/release/astrofs"
    echo ""
    echo "Or use:"
    echo "   cargo run --release"
    echo ""
else
    echo ""
    echo "❌ Build failed. Check the errors above."
    exit 1
fi
