.PHONY: help build test run clean fmt clippy pre-commit

.DEFAULT_GOAL := help

help:
	@echo "MD Parser - Available Commands:"
	@echo "==============================="
	@echo "build      - Build the project (debug version)"
	@echo "run        - Run the parser (uses test.md file)"
	@echo "test       - Run tests"
	@echo "clean      - Clean build artifacts (target directory)"
	@echo "fmt        - Format code (cargo fmt)"
	@echo "clippy     - Run linter checks (cargo clippy)"
	@echo "pre-commit - Run checks: format + clippy + test"
	@echo "help       - Show this message"

build:
	@echo "Building MD Parser..."
	cargo build

test:
	@echo "Running tests..."
	cargo test

run: build
	@echo "Running MD Parser..."
	@if [ -f "test.md" ]; then \
		cargo run parse test.md; \
	else \
		echo "Error: test.md file not found"; \
		exit 1; \
	fi

clean:
	@echo "Cleaning build artifacts..."
	cargo clean


fmt:
	@echo "Formatting code..."
	cargo fmt

clippy:
	@echo "Running clippy lints..."
	cargo clippy

pre-commit: fmt clippy test
	@echo "Pre-commit checks passed!"
