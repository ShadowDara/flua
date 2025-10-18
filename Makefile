# Because i am to lazy to remember all those commands

# The commands are always forced
.PHONY: test docs cl

# Standard run
all: build

# Testing
test:
	cargo fmt
	cargo test

# Debug Build
b:
	cargo build

# Run Lua Build Script
build:
	cargo run build.lua

# Start the Docs Server
docs:
	mmkdocs serve --dev-addr 0.0.0.0:9000

# Delete the Existing Build
cl:
	cargo clean
