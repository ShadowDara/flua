#!/bin/bash

echo "Script to test the flua Terminal Functions"

echo "Checking Version:"
./target/debug/flua --version

echo "Checking Help:"
./target/debug/flua --help
