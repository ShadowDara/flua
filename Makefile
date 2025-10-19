# Because i am to lazy to remember all those commands

# The commands are always forced
.PHONY: test docs cl pre-commit fmt

# Standard run
all: build

# Command to run before committing Code
# Cleaning before to check if its really perfekt
pre-commit:
	$(MAKE) cl
	cargo check
	$(MAKE) test
	$(MAKE) build

# Testing all
test:
	$(MAKE) testrust
	$(MAKE) testmodule

# Run the Rust tests Code
testrust:
	cargo test

# Testing a Module
testmodule:
	cargo run -- run module -path=test/datestmodule

# Debug Build
b:
	$(MAKE) fmt
	cargo build

# Run Lua Build Script
build:
	$(MAKE) fmt
	cargo run build.lua

# Search for todos in the codebase
todo:
	grep --color=auto --exclude-dir=target --exclude-dir=site --exclude-dir=.git -rw TODO .

# Command to Install Dependencies
install:
	pip install -r build/requirements.txt

# Start the Docs Server
docs:
	mmkdocs serve --dev-addr 0.0.0.0:9000

# Count Line Stats with cloc
stats:
	cloc . --csv --out=stats.csv --exclude-dir=target,site,docs

# Delete the Existing Build
cl:
	cargo clean

# Format the Code and clean other stuff
fmt:
	cargo fmt
