#!/bin/bash

# Finance Manager Installation Script

set -e

APP_NAME="finance"
REPO_URL="https://github.com/tuusuario/finance"
INSTALL_DIR="$HOME/.local/bin"

echo "🚀 Installing Finance Manager..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Check if we're in the source directory
if [ -f "Cargo.toml" ] && grep -q "name = \"finance\"" Cargo.toml; then
    echo "📦 Building from source..."
    cargo build --release
    cp target/release/$APP_NAME "$INSTALL_DIR/"
    echo "✅ Installed $APP_NAME to $INSTALL_DIR"
else
    echo "❌ This script should be run from the finance project directory"
    exit 1
fi

# Make sure the install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  Add $INSTALL_DIR to your PATH:"
    echo "   echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
    echo "   source ~/.bashrc"
fi

echo "🎉 Installation complete!"
echo "📄 Run 'finance' to start the application"

# Set production mode
echo "🔧 Setting up production mode..."
echo 'export RUST_ENV=production' >> ~/.bashrc
echo "✅ Production mode enabled"

echo ""
echo "🚀 Quick start:"
echo "   1. Run: source ~/.bashrc"
echo "   2. Run: finance"
echo "   3. Start managing your finances!"
