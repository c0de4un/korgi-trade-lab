# Korgi.Trade-Lab Makefile — Cargo workspace targets from repository root.

.PHONY: help init build build-lob build-cabinet run-lob-dev run-cabinet-dev \
	test-lob test-cabinet test-all lint fmt infra-up infra-down

COMPOSE := docker compose -f compose.local.yml

help: ## Show this help message
	@echo "Korgi.Trade-Lab Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make <target>"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

init: ## Fetch workspace dependencies
	@echo "Fetching Cargo workspace dependencies..."
	cargo fetch
	@echo "Initialization complete."

# ================= BUILD TARGETS =================

build: ## Build all workspace members
	cargo build --workspace

build-lob: ## Build LOB server and CLI (release)
	cargo build -p lob-server -p lob-cli --release

build-cabinet: ## Build cabinet server and CLI (release)
	cargo build -p cabinet-server -p cabinet-cli --release

# ================= RUN TARGETS =================

run-lob-dev: ## Run LOB server in development mode (default :8080)
	cargo run -p lob-server

run-cabinet-dev: ## Run cabinet server in development mode (default :8081)
	cargo run -p cabinet-server

# ================= TEST TARGETS =================

test-lob: ## Run LOB-related package tests
	cargo test -p lob-server -p lob-cli -p lob-core -p sim-engine -p importers -p market-types -p bot-protocol -p storage -p fix-codec

test-cabinet: ## Run cabinet-related package tests
	cargo test -p cabinet-server -p cabinet-cli -p storage -p market-types -p bot-protocol

test-all: ## Run all workspace tests
	cargo test --workspace

# ================= LINT & FORMAT =================

lint: ## Run clippy with warnings denied
	cargo clippy --workspace -- -D warnings

fmt: ## Format all Rust sources
	cargo fmt --all

# ================= INFRASTRUCTURE =================

infra-up: ## Start PostgreSQL and Redis (local compose)
	$(COMPOSE) up -d

infra-down: ## Stop local infrastructure
	$(COMPOSE) down
