.PHONY: help build test clean docker run-server run-example release-prepare release-publish build-all test-all test-setup docs-build docs-deploy

# Version management
VERSION ?= $(shell cat package.json | grep '"version"' | head -1 | awk -F: '{ print $$2 }' | sed 's/[",]//g' | tr -d '[[:space:]]')
DOCKER_IMAGE = scoutquest/server
GITHUB_REPO = RomainDECOSTER/scoutquest

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

test-setup: ## Run setup tests (unit tests only, no integration tests)
	@echo "🧪 Running setup tests (unit tests only)..."
	@echo "🧪 Testing Rust SDK (lib only)..."
	cd scoutquest-rust && cargo test --lib
	@echo "🧪 Testing JavaScript SDK..."
	cd scoutquest-js && pnpm test
	@echo "✅ Setup tests completed (integration tests skipped)"

# ============================================================================
# RELEASE MANAGEMENT
# ============================================================================

release-prepare: ## Prepare release with version bump
	@echo "🚀 Preparing release v$(VERSION)..."
	@echo "📝 Updating version in all components..."
	$(call update_version,$(VERSION))
	@echo "🔨 Building all components..."
	$(MAKE) build-all
	@echo "🧪 Running all tests..."
	$(MAKE) test-all
	@echo "📚 Building documentation..."
	$(MAKE) docs-build

verify-release: ## Verify release prerequisites
	@echo "🔍 Verifying release prerequisites..."
	@command -v cargo >/dev/null || { echo "❌ Rust/Cargo not found"; exit 1; }
	@command -v pnpm >/dev/null || { echo "❌ pnpm not found"; exit 1; }
	@command -v docker >/dev/null || { echo "❌ Docker not found"; exit 1; }
	@echo "✅ All prerequisites verified"

release-publish: ## Publish release artifacts
	@echo "🚀 Publishing release v$(VERSION)..."
	@echo "📦 Publishing JavaScript SDK..."
	cd scoutquest-js && pnpm publish --access public
	@echo "📦 Publishing Rust SDK..."
	cd scoutquest-rust && cargo publish
	@echo "🐳 Building and pushing Docker images..."
	$(MAKE) docker-build-release
	$(MAKE) docker-push-release
	@echo "📚 Deploying documentation..."
	$(MAKE) docs-deploy
	@echo "✅ Release v$(VERSION) published successfully!"

build-all: ## Build all components
	@echo "🔨 Building server..."
	cd scoutquest-server && cargo build --release
	@echo "🔨 Building Rust SDK..."
	cd scoutquest-rust && cargo build --release
	@echo "🔨 Building JavaScript SDK..."
	cd scoutquest-js && pnpm install && pnpm build
	@echo "🔨 Building examples..."
	cd examples/rust && cargo build --release
	cd examples/js && pnpm install && pnpm build

test-all: ## Run all tests
	@echo "🧪 Testing server..."
	cd scoutquest-server && cargo test
	@echo "🧪 Testing Rust SDK..."
	cd scoutquest-rust && RUST_MIN_STACK=8388608 cargo test
	@echo "🧪 Testing JavaScript SDK..."
	cd scoutquest-js && pnpm test
	@echo "🧪 Running integration tests..."
	cd examples/rust && cargo test --bins --tests --benches

docs-build: ## Build documentation website
	@echo "📚 Building documentation website..."
	@# Documentation is already built as static HTML
	rm -rf docs/dist
	mkdir -p docs/dist
	cp -r docs/assets docs/docs docs/index.html docs/README.md docs/dist/ 2>/dev/null || true
	@echo "📚 Documentation built in docs/dist/"

docs-deploy: ## Deploy documentation to GitHub Pages
	@echo "📚 Deploying documentation to GitHub Pages..."
	@# This would typically use gh-pages or similar
	@echo "📚 Documentation deployment configured for GitHub Actions"

docker-build-release: ## Build Docker image for release
	@echo "🐳 Building Docker image v$(VERSION)..."
	cd scoutquest-server && docker build -t $(DOCKER_IMAGE):$(VERSION) -t $(DOCKER_IMAGE):latest .

docker-push-release: ## Push Docker image to registry
	@echo "🐳 Pushing Docker image v$(VERSION)..."
	docker push $(DOCKER_IMAGE):$(VERSION)
	docker push $(DOCKER_IMAGE):latest

# ============================================================================
# VERSION MANAGEMENT HELPERS
# ============================================================================

define update_version
	@echo "📝 Updating version to $(1)..."
	@# Update package.json
	sed -i.bak 's/"version": "[^"]*"/"version": "$(1)"/' package.json && rm package.json.bak
	@# Update JS SDK package.json
	sed -i.bak 's/"version": "[^"]*"/"version": "$(1)"/' scoutquest-js/package.json && rm scoutquest-js/package.json.bak
	@# Update Rust SDK Cargo.toml (only package version in [package] section)
	sed -i.bak '/^\[package\]/,/^\[/ s/^version = "[^"]*"/version = "$(1)"/' scoutquest-rust/Cargo.toml && rm scoutquest-rust/Cargo.toml.bak
	@# Update Server Cargo.toml (only package version in [package] section)
	sed -i.bak '/^\[package\]/,/^\[/ s/^version = "[^"]*"/version = "$(1)"/' scoutquest-server/Cargo.toml && rm scoutquest-server/Cargo.toml.bak
	@echo "✅ Version updated to $(1) in all components"
endef

version: ## Show current version
	@echo "Current version: $(VERSION)"

version-bump-patch: ## Bump patch version (1.0.0 -> 1.0.1)
	$(eval NEW_VERSION := $(shell echo $(VERSION) | awk -F. '{$$3++; print $$1"."$$2"."$$3}'))
	$(call update_version,$(NEW_VERSION))
	@echo "Version bumped to $(NEW_VERSION)"

version-bump-minor: ## Bump minor version (1.0.0 -> 1.1.0)
	$(eval NEW_VERSION := $(shell echo $(VERSION) | awk -F. '{$$2++; $$3=0; print $$1"."$$2"."$$3}'))
	$(call update_version,$(NEW_VERSION))
	@echo "Version bumped to $(NEW_VERSION)"

version-bump-major: ## Bump major version (1.0.0 -> 2.0.0)
	$(eval NEW_VERSION := $(shell echo $(VERSION) | awk -F. '{$$1++; $$2=0; $$3=0; print $$1"."$$2"."$$3}'))
	$(call update_version,$(NEW_VERSION))
	@echo "Version bumped to $(NEW_VERSION)"
