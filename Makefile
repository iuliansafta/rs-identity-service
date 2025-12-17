.PHONY: help test test-verbose test-api test-models test-migration test-jwt watch build run clean fmt lint

help:
	@echo "Available commands:"
	@echo "  make test           - Run all tests in workspace"
	@echo "  make test-verbose   - Run tests with output"
	@echo "  make test-api       - Run API package tests only"
	@echo "  make test-models    - Run models package tests only"
	@echo "  make test-migration - Run migration package tests only"
	@echo "  make test-jwt       - Run JWT service tests specifically"
	@echo "  make watch          - Watch and auto-run tests on changes"
	@echo "  make build          - Build all packages"
	@echo "  make run            - Run the application"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make fmt            - Format code"
	@echo "  make lint           - Run clippy linter"

test:
	cargo test --workspace

test-verbose:
	cargo test --workspace -- --nocapture

test-api:
	cargo test -p identity-service-api

test-models:
	cargo test -p models

test-migration:
	cargo test -p migration

test-jwt:
	cargo test --workspace jwt_service

watch:
	cargo watch -x "test --workspace"

build:
	cargo build --workspace

build-release:
	cargo build --workspace --release

run:
	cargo run

clean:
	cargo clean

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace -- -D warnings

lint-fix:
	cargo clippy --workspace --fix

# Check everything before commit
check: fmt-check lint test
	@echo "All checks passed!"
