# 🔍 SquoutQuest

**Universal Service Discovery for microservices architectures**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/scoutquest/scoutquest/workflows/CI/badge.svg)](https://github.com/scoutquest/scoutquest/actions)

SquoutQuest is a modern Service Discovery solution designed to simplify microservices management in polyglot environments.

## 🚀 Quick Start

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

- [Server Guide](scoutquest-server/README.md)
- [Rust SDK Guide](scoutquest-rust/README.md)

## 🤝 Contributing

Contributions are welcome! See CONTRIBUTING.md for more information.

## 📄 License

MIT - see LICENSE for more details.
