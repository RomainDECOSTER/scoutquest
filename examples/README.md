# SquoutQuest Examples

This directory contains examples demonstrating how to use SquoutQuest Service Discovery with different technologies and frameworks.

## Available Examples

### Rust Examples

- **Axum Service** (`rust/`) - A complete Axum web service that demonstrates:
  - Service registration with SquoutQuest
  - Health checks and monitoring
  - Service discovery and inter-service communication
  - Load balancing strategies
  - Graceful shutdown

## Quick Start

1. **Start the SquoutQuest server**:
   ```bash
   cd scoutquest-server
   cargo run
   ```

2. **Run an example**:
   ```bash
   # Rust Axum example
   cd examples/rust
   cargo run --example axum_service
   ```

3. **View the dashboard**:
   Open http://localhost:8080/dashboard in your browser

## Example Structure

```
examples/
├── rust/                    # Rust examples
│   ├── Cargo.toml          # Dependencies for Rust examples
│   ├── README.md           # Rust-specific instructions
│   └── axum_service.rs     # Complete Axum service example
└── README.md               # This file
```

## Contributing Examples

To add a new example:

1. Create a new directory for your technology (e.g., `python/`, `nodejs/`)
2. Include a `README.md` with setup and usage instructions
3. Add a reference to your example in this README
4. Ensure your example demonstrates key SquoutQuest features

## Features Demonstrated

Each example showcases:

- ✅ **Service Registration**: Automatic registration with SquoutQuest
- ✅ **Health Checks**: Configuring and monitoring service health
- ✅ **Service Discovery**: Finding and calling other services
- ✅ **Load Balancing**: Using different load balancing strategies
- ✅ **Metadata & Tags**: Adding service metadata and tags
- ✅ **Graceful Shutdown**: Proper cleanup on service termination 