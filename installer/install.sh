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
    echo "Adding $INSTALL_DIR to PATH..."

    # Detect shell config file
    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
        bash)
            PROFILE_FILE="$HOME/.bashrc"
            ;;
        zsh)
            PROFILE_FILE="$HOME/.zshrc"
            ;;
        *)
            PROFILE_FILE="$HOME/.profile"
            ;;
    esac

    # Add export only if not already present
    if ! grep -Fxq "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$PROFILE_FILE"; then
        echo "" >> "$PROFILE_FILE"
        echo "# Added by LuaJIT installer" >> "$PROFILE_FILE"
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$PROFILE_FILE"
        echo "Updated $PROFILE_FILE to include $INSTALL_DIR in PATH."
    else
        echo "$INSTALL_DIR is already set in $PROFILE_FILE"
    fi

    echo "To apply the changes now, run:"
    echo "  source $PROFILE_FILE"
else
    echo "$INSTALL_DIR is already in your PATH."
fi

echo "Done! You can now run '$OUTFILE' from anywhere."
