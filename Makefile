# MCP Guard - Development Makefile
# Run 'make help' for available commands

.PHONY: help build test clean run dev frontend-* backend-* dev-*

# Default target
help:
	@echo "MCP Guard Development Commands"
	@echo ""
	@echo "Unified Development:"
	@echo "  make dev            Start all services (backend + frontend)"
	@echo "  make dev-stop       Stop all services gracefully"
	@echo "  make dev-restart    Restart all services"
	@echo "  make dev-status     Show status of all services"
	@echo "  make dev-logs       Tail logs from all services"
	@echo ""
	@echo "Backend (Rust):"
	@echo "  make build          Build the backend in release mode"
	@echo "  make build-dev      Build the backend in debug mode"
	@echo "  make test           Run backend tests"
	@echo "  make run            Run the backend server"
	@echo "  make check          Run cargo check"
	@echo "  make clippy         Run clippy linter"
	@echo "  make fmt            Format Rust code"
	@echo "  make bench          Run benchmarks"
	@echo ""
	@echo "Frontend (Angular):"
	@echo "  make frontend       Start frontend dev server"
	@echo "  make frontend-dev   Start frontend with browser open"
	@echo "  make frontend-stop  Stop frontend dev server"
	@echo "  make frontend-build Build frontend for production"
	@echo "  make frontend-clean Clean frontend build artifacts"
	@echo ""
	@echo "Full Stack:"
	@echo "  make build-all      Build both backend and frontend"
	@echo "  make clean-all      Clean all build artifacts"
	@echo ""

# =============================================================================
# Backend Commands
# =============================================================================

build:
	cargo build --release

build-dev:
	cargo build

test:
	cargo test

run:
	cargo run -- run

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

bench:
	cargo bench

# =============================================================================
# Frontend Commands
# =============================================================================

frontend:
	@./scripts/frontend.sh start

frontend-dev:
	@./scripts/frontend.sh dev

frontend-stop:
	@./scripts/frontend.sh stop

frontend-restart:
	@./scripts/frontend.sh restart

frontend-build:
	@./scripts/frontend.sh build

frontend-preview:
	@./scripts/frontend.sh preview

frontend-clean:
	@./scripts/frontend.sh clean

frontend-install:
	@./scripts/frontend.sh install

frontend-status:
	@./scripts/frontend.sh status

# =============================================================================
# Unified Development Commands (use scripts/dev.sh)
# =============================================================================

dev:
	@./scripts/dev.sh start

dev-stop:
	@./scripts/dev.sh stop

dev-restart:
	@./scripts/dev.sh restart

dev-status:
	@./scripts/dev.sh status

dev-logs:
	@./scripts/dev.sh logs

# =============================================================================
# Full Stack Build Commands
# =============================================================================

build-all: build frontend-build
	@echo "Both backend and frontend built successfully"

clean-all: clean frontend-clean
	@echo "All build artifacts cleaned"


clean:
	cargo clean

# =============================================================================
# Utility Commands
# =============================================================================

install-deps:
	@echo "Installing Rust dependencies..."
	cargo fetch
	@echo "Installing frontend dependencies..."
	cd landing && npm install

update-deps:
	@echo "Updating Rust dependencies..."
	cargo update
	@echo "Updating frontend dependencies..."
	cd landing && npm update

# Quick development setup
setup: install-deps
	@echo "Development environment ready!"
	@echo "Run 'make frontend-dev' to start the landing page"
	@echo "Run 'make run' to start the backend"
