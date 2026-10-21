# Korgi.Trade-Lab Makefile
# Separates build, test, and run processes for independent modules.

.PHONY: help init build-lob build-cabinet run-lob-dev run-cabinet-dev test-lob test-cabinet test-all lint fmt

help: ## Show this help message
	@echo "Korgi.Trade-Lab Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make <target>"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

init: ## Initialize project dependencies and tools
	@echo "Initializing Rust toolchains and dependencies..."
	cd lob/backend && cargo fetch
	cd cabinet/backend && cargo fetch
	@echo "Initialization complete."

# ================= BUILD TARGETS =================

build-lob: ## Build LOB Engine backend
	@echo "Building LOB Engine..."
	cd lob/backend && cargo build --release

build-cabinet: ## Build Trader Cabinet backend
	@echo "Building Trader Cabinet..."
	cd cabinet/backend && cargo build --release

# ================= RUN TARGETS =================

run-lob-dev: ## Run LOB Engine in development mode
	@echo "Running LOB Engine (dev)..."
	cd lob/backend && cargo run

run-cabinet-dev: ## Run Trader Cabinet in development mode
	@echo "Running Trader Cabinet (dev)..."
	cd cabinet/backend && cargo run

# ================= TEST TARGETS =================

test-lob: ## Run tests for LOB Engine
	@echo "Testing LOB Engine..."
	cd lob/backend && cargo test

test-cabinet: ## Run tests for Trader Cabinet
	@echo "Testing Trader Cabinet..."
	cd cabinet/backend && cargo test

test-all: test-lob test-cabinet ## Run tests for all modules

# ================= LINT & FORMAT =================

lint: ## Run linters (clippy) on all modules
	@echo "Linting LOB Engine..."
	cd lob/backend && cargo clippy -- -D warnings
	@echo "Linting Trader Cabinet..."
	cd cabinet/backend && cargo clippy -- -D warnings

fmt: ## Format code in all modules
	@echo "Formatting LOB Engine..."
	cd lob/backend && cargo fmt
	@echo "Formatting Trader Cabinet..."
	cd cabinet/backend && cargo fmt