# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ScoutQuest is a universal service discovery platform for microservices. It is a monorepo with three main packages:
- **`scoutquest-server/`** — Rust/Axum HTTP server (the discovery backend)
- **`scoutquest-rust/`** — Rust client SDK (crate for Rust microservices)
- **`scoutquest-js/`** — TypeScript/JavaScript client SDK (npm package `@scoutquest/sdk`)

## Commands

### Monorepo (root)
```bash
make build-all       # Build server + all SDKs
make test-all        # Run all tests (server, Rust SDK, JS SDK, examples)
make test-setup      # Unit tests only, skips integration tests (faster)
make format          # cargo fmt across all Rust crates
make run-server      # Start the discovery server
```

### JavaScript SDK (`scoutquest-js/`)
```bash
pnpm build           # Compile TypeScript → CJS + ESM (via tsup)
pnpm test            # Run Jest test suite
pnpm test -- client.test.ts --verbose   # Run a single test file
pnpm lint            # ESLint (flat config)
pnpm lint:fix        # Auto-fix ESLint issues
pnpm format          # Prettier
pnpm typecheck       # tsc --noEmit
pnpm quality         # typecheck + lint + format:check + test (full CI check)
```

### Rust (server or SDKs)
```bash
cargo build --release
cargo test                                 # All tests
cargo test health_check -- --nocapture    # Single test by name
cargo test --lib                           # Unit tests only
cargo fmt && cargo clippy                  # Format + lint
```

## Architecture

### Server (`scoutquest-server/src/`)
- `registry.rs` — In-memory service registry backed by `DashMap` (concurrent hashmap)
- `health_checker.rs` — Scheduled health checks (tokio-cron-scheduler) that HTTP-ping each registered service
- `api.rs` — Axum route handlers (register, discover, heartbeat, deregister, etc.)
- `middleware/ip_restriction.rs` — IP allowlist middleware
- `tls/` — Auto-generates self-signed certs or uses provided ones
- Configuration is TOML-driven (`config/default.toml`, `development.toml`, `production.toml`, `kubernetes.toml`)

### JavaScript SDK (`scoutquest-js/src/`)
- `client.ts` — `ScoutQuestClient` class; wraps axios for HTTP and `ws` for WebSocket event streaming
- `types.ts` — TypeScript interfaces shared between client and server contracts
- `errors.ts` — Typed error classes with error codes (e.g. `SERVICE_NOT_FOUND`, `TIMEOUT_ERROR`)
- `index.ts` — Public exports and factory function
- Tests live in `src/__tests__/` and mock axios + ws entirely

Key SDK features: load balancing strategies (RoundRobin, Random, WeightedRandom, LeastConnections, HealthyOnly), automatic heartbeat, retry with exponential backoff, and real-time event streaming via WebSocket.

### Rust SDK (`scoutquest-rust/src/`)
- `client.rs` — `ServiceDiscoveryClient` using reqwest
- `models.rs` — Shared data structures (mirrors the server's `models.rs`)
- `error.rs` — Typed error enum
- Integration tests in `tests/`, benchmarks in `benches/` (criterion)

### Release & Publishing
Releases are fully automated via semantic-release (`.releaserc.json`). A single commit to `main` with conventional commits triggers version bumps and publishes to npm, crates.io, Docker Hub, and GitHub Pages simultaneously. The Makefile `release-publish` target coordinates the cross-ecosystem publishing.
