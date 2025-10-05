#!/bin/bash

# Devcontainer Skript

rustup component add rustfmt clippy

cargo build

# Download the script
curl -L -o "LuajitInstall.sh" "https://raw.githubusercontent.com/ShadowDara/LuaAPI-Rust/refs/heads/main/installer/install.sh"

# Make it executable
chmod +x LuajitInstall.sh

# Execute the script
./LuajitInstall.sh "v0.1.11"

# Test the Install
luajit build.lua
