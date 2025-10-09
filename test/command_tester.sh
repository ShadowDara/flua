#!/bin/bash

echo "Script to test the luajit Terminal Functions"

# Build First
echo "Build the executable First"
cargo build

echo "Checking Version:"
./target/debug/luajit --version

echo "Checking Help:"
./target/debug/luajit --help
