#!/bin/bash

# Install Script for LuaJIT (User Installation)

# Argument 1 = Version
# Argument 2 = Output file name (optional, default: luajit)

VERSION="$1"
OUTFILE="${2:-luajit}"
INSTALL_DIR="$HOME/.local/bin"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version> [output-file]"
    exit 1
fi

# Check if curl is available
if ! command -v curl &> /dev/null; then
    echo "curl is not installed. Please install curl and try again."
    exit 1
fi

# Create install dir if it doesn't exist
mkdir -p "$INSTALL_DIR"

echo "Downloading LuaJIT version $VERSION..."

curl -L -o "$OUTFILE" "https://github.com/ShadowDara/LuaAPI-Rust/releases/download/$VERSION/luajit-linux-x86_64"

if [ $? -ne 0 ]; then
    echo "Download failed!"
    exit 1
fi

chmod +x "$OUTFILE"
echo "Set executable permissions for $OUTFILE."

mv "$OUTFILE" "$INSTALL_DIR/"

if [ $? -eq 0 ]; then
    echo "Installed $OUTFILE to $INSTALL_DIR"
else
    echo "Failed to move $OUTFILE to $INSTALL_DIR"
    exit 1
fi

# Check if $INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "To add it, run:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo "And consider adding that line to your shell profile (~/.bashrc, ~/.zshrc, etc.)"
else
    echo "$INSTALL_DIR is already in your PATH."
fi

echo "Done! You can now run '$OUTFILE' from anywhere."
