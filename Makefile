# Because i am to lazy to remember all those commands

# Standard run
all: t

# Run the Code
r:
	cargo run
	cargo fmt

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
	cargo install --locked cargo-nextest
	cargo install --locked cargo-bloat

# Start the Docs Server
docs:
	mkdocs serve --dev-addr 0.0.0.0:9000

# Search for todos in the codebase
todo:
	grep --color=auto --exclude-dir=target --exclude-dir=site --exclude-dir=.git -rw TODO .

# Command to run before committing Code
# Cleaning before to check if its really perfekt
pr:
	$(MAKE) i
	mkdocs build
	cargo clean
	cargo check
	$(MAKE) fulltest
	$(MAKE) b
	$(MAKE) s
	$(MAKE) todo

# INFO
# This is writen to be runned in a dockercontainer, this does probably
# not work everywhere!
fulltest:
	cargo install --locked cargo-nextest
	cargo test
	cargo nextest run --no-fail-fast

deprecated:
	grep --color=auto --exclude-dir=target --exclude-dir=site --exclude-dir=.git -rw DEPRECATED .

# The commands are always forced
.PHONY: test docs cl pre-commit fmt fulltest

#
# =========================================
#

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
