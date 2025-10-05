#!/bin/bash

# Install Script for LuaJIT
# Requires Curl

# Argument 1 = Version
# Argument 2 = Output file

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <version> <output-file>"
    exit 1
fi

if ! command -v curl &> /dev/null; then
    echo "curl is not installed. Please install curl and try again."
    exit 1
fi

echo "Downloading LuaJIT version $1 with curl..."

curl -L -o "$2" "https://github.com/ShadowDara/LuaAPI-Rust/releases/download/$1/luajit-linux-x86_64"

if [ $? -eq 0 ]; then
    echo "Download successful: $2"
else
    echo "Download failed!"
    exit 1
fi
