.PHONY: help build test clean docker run-server run-example

help: ## Display help
	@echo "ScoutQuest - Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Compile server and SDK
	@echo "🔨 Compiling server..."
	cd scoutquest-server && cargo build --release
	@echo "🔨 Compiling Rust SDK..."
	cd scoutquest-rust && cargo build --release
	@echo "🔨 Compiling JS SDK..."
	cd scoutquest-js && pnpm build

test: ## Run tests
	@echo "🧪 Server tests..."
	cd scoutquest-server && cargo test
	@echo "🧪 Rust SDK tests..."
	cd scoutquest-rust && RUST_MIN_STACK=8388608 cargo test
	@echo "🧪 JS SDK tests..."
	cd scoutquest-js && pnpm test

clean: ## Clean artifacts
	cd scoutquest-server && cargo clean
	cd scoutquest-rust && cargo clean
	cd examples/rust && cargo clean
	cd examples/rust/axum_example && cargo clean
	cd examples/rust/notification_example && cargo clean
	cd scoutquest-js && pnpm clean
	cd examples/js && pnpm clean

run-server: ## Start server
	cd scoutquest-server && cargo run

install: ## Install server
	cd scoutquest-server && cargo install --path .

format: ## Format code
	cd scoutquest-server && cargo fmt
	cd scoutquest-rust && cargo fmt

check: ## Check code
	cd scoutquest-server && cargo check
	cd scoutquest-rust && cargo check

docs: ## Generate documentation
	cd scoutquest-server && cargo doc --no-deps
	cd scoutquest-rust && cargo doc --no-deps