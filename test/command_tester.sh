#!/bin/bash

echo "Script to test the luajit Terminal Functions"

echo "Checking Version:"
./target/debug/luajit --version

echo "Checking Help:"
./target/debug/luajit --help
