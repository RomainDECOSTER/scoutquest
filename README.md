# 🔍 SquoutQuest

**Universal Service Discovery for microservices architectures**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Latest Release](https://img.shields.io/github/v/release/RomainDECOSTER/scoutquest?label=version)](https://github.com/RomainDECOSTER/scoutquest/releases/latest)
[![Build Status](https://github.com/RomainDECOSTER/scoutquest/workflows/CI/badge.svg)](https://github.com/RomainDECOSTER/scoutquest/actions)
[![Release](https://github.com/RomainDECOSTER/scoutquest/workflows/Release/badge.svg)](https://github.com/RomainDECOSTER/scoutquest/actions)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://romaindecoster.github.io/scoutquest/)
[![npm version](https://badge.fury.io/js/@scoutquest%2Fsdk.svg)](https://badge.fury.io/js/@scoutquest%2Fsdk)
[![Crates.io](https://img.shields.io/crates/v/scoutquest-rust.svg)](https://crates.io/crates/scoutquest-rust)
[![Docker Hub](https://img.shields.io/docker/v/scoutquest/server?label=docker)](https://hub.docker.com/r/scoutquest/server)

SquoutQuest is a modern Service Discovery solution designed to simplify microservices management in polyglot environments.

## 🚀 Quick Start

### Prerequisites

- **Node.js 22+** (we recommend using [nvm](https://github.com/nvm-sh/nvm))
- **Rust 1.70+**
- **pnpm 10+**

### Installation

```bash
# Install and use Node.js 22 with nvm
nvm install 22
nvm use

# Or if you have .nvmrc support:
nvm use

# Install dependencies
pnpm install

# Run development setup
./scripts/setup-dev.sh
```

### 1. Start the server

```bash
cd scoutquest-server
cargo run
```

### 2. Use the Rust SDK

```bash
cd scoutquest-rust
cargo run --example axum_service
```

### 3. Dashboard

Open http://localhost:8080/dashboard in your browser.

## 📦 Project Structure

```
scoutquest/
├── scoutquest-server/     # Main server (Rust)
│   └── config/           # Server configuration files
├── scoutquest-rust/       # Rust SDK
├── examples/              # Usage examples
└── docs/                  # Documentation
```

## 🔧 Configuration

See files in `scoutquest-server/config/` for configuration options.

## 📚 Documentation

📖 **[Complete Documentation](https://romaindecoster.github.io/scoutquest/)** - Official documentation website

### API References

- [Server Guide](scoutquest-server/README.md)
- [Rust SDK Guide](scoutquest-rust/README.md)

## 🤝 Contributing

Contributions are welcome! See CONTRIBUTING.md for more information.

## 📄 License

MIT - see LICENSE for more details.
