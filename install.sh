#!/bin/bash
# install.sh - Install rustlink locally
# Usage: ./install.sh

set -e

echo "=========================================="
echo "  RustLink Local Installation"
echo "=========================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed."
    echo ""
    echo "Install Rust with:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust found: $(cargo --version)"

# Get script directory (resolve symlinks)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Build release
echo ""
echo "Building rustlink..."
cargo build --release

# Find binary location
BINARY_PATH="$SCRIPT_DIR/target/release/rustlink"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Build failed - binary not found"
    exit 1
fi

# Suggest adding to PATH
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

if [ ! -L "$INSTALL_DIR/rustlink" ]; then
    ln -sf "$BINARY_PATH" "$INSTALL_DIR/rustlink"
    echo "✓ Linked to $INSTALL_DIR/rustlink"
    echo ""
    echo "Add to your PATH if not already:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
else
    echo "✓ Binary already linked at $INSTALL_DIR/rustlink"
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  rustlink init <username>   # Create identity"
echo "  rustlink status            # Check status"
echo "  rustlink run               # Start P2P node"
