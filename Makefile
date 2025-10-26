# Because i am to lazy to remember all those commands

# Standard run
all: testrust

# Run the Rust tests Code
t:
	cargo nextest run
	cargo fmt

# Run Lua Build Script
b:
	cargo run build.lua
	cargo fmt

# Run Benchmarks
# s -> stands for Speed !
s:
	cargo +nightly bench
	cargo fmt

# Command to Install Dependencies
i:
	pip install -r build/requirements.txt

# Start the Docs Server
docs:
	mmkdocs serve --dev-addr 0.0.0.0:9000

# Search for todos in the codebase
todo:
	grep --color=auto --exclude-dir=target --exclude-dir=site --exclude-dir=.git -rw TODO .

# The commands are always forced
.PHONY: test docs cl pre-commit fmt

#
# =========================================
#

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

# Testing a Module
testmodule:
	cargo run -- run module -path=test/datestmodule

# Count Line Stats with cloc
stats:
	cloc . --csv --out=stats.csv --exclude-dir=target,site,docs
