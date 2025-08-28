# ScoutQuest Rust SDK

[![Crates.io](https://img.shields.io/crates/v/scoutquest-rust.svg)](https://crates.io/crates/scoutquest-rust)
[![Documentation](https://docs.rs/scoutquest-rust/badge.svg)](https://docs.rs/scoutquest-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for ScoutQuest Service Discovery - Universal service discovery for microservices architectures.

## Features

- 🔍 **Service Discovery**: Find and connect to services dynamically
- 📝 **Service Registration**: Register your services with health checks
- ⚖️ **Load Balancing**: Multiple strategies (Random, Round-Robin, Healthy-Only)
- 🌐 **HTTP Client**: Built-in HTTP client with automatic service discovery
- 🏷️ **Metadata & Tags**: Rich service metadata and tag-based filtering
- 💓 **Health Monitoring**: Automatic heartbeat and health check support
- 🔄 **Retry Logic**: Configurable retry mechanism for resilience
- 📊 **Tracing Support**: Built-in logging and tracing integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
scoutquest-rust = "1.0.0"
```

## Quick Start

```rust
use scoutquest_rust::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = ServiceDiscoveryClient::new("http://localhost:8080")?;
    
    // Register a service
    let options = ServiceRegistrationOptions::new()
        .with_tags(vec!["api".to_string(), "v1".to_string()])
        .with_metadata({
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("version".to_string(), "1.2.3".to_string());
            metadata
        });
    
    client.register_service("user-service", "localhost", 3000, Some(options)).await?;
    
    // Discover services
    let instances = client.discover_service("payment-service", None).await?;
    
    // Make HTTP calls to discovered services
    let response: serde_json::Value = client.get("payment-service", "/api/balance").await?;
    
    // Graceful shutdown
    client.deregister().await?;
    Ok(())
}
```

## Advanced Usage

### Load Balancing Strategies

```rust
use scoutquest_rust::*;

let client = ServiceDiscoveryClient::new("http://localhost:8080")?;

// Service discovery returns a ready-to-use service instance
let instance = client.discover_service("api-service", None).await?;

// Use the instance to make HTTP calls
let response = client.get(&instance, "/users", None).await?;
```

### Service Discovery with Filters

```rust
let discovery_options = ServiceDiscoveryOptions::new()
    .with_healthy_only(true)
    .with_tags(vec!["production".to_string(), "api".to_string()])
    .with_limit(5);

let instances = client.discover_service("user-service", Some(discovery_options)).await?;
```

### HTTP Client with Custom Configuration

```rust
// GET request
let users: serde_json::Value = client.get("user-service", "/api/users").await?;

// POST request
let new_user = serde_json::json!({
    "name": "John Doe",
    "email": "john@example.com"
});
let response: serde_json::Value = client.post("user-service", "/api/users", new_user).await?;

// Custom request with specific load balancing
let response: serde_json::Value = client.call_service(
    "user-service",
    "/api/health",
    reqwest::Method::GET,
    None,
    LoadBalancingStrategy::HealthyOnly
).await?;
```

### Health Checks

```rust
use scoutquest_rust::*;

let health_check = HealthCheck {
    url: "/health".to_string(),
    interval_seconds: 30,
    timeout_seconds: 5,
    method: "GET".to_string(),
    expected_status: 200,
    headers: None,
};

let options = ServiceRegistrationOptions::new()
    .with_health_check(health_check);

client.register_service("api-service", "localhost", 8080, Some(options)).await?;
```

## Configuration

Create a client with custom configuration:

```rust
use std::time::Duration;

let client = ServiceDiscoveryClient::with_config(
    "http://localhost:8080",
    Duration::from_secs(30),  // HTTP timeout
    3,                        // Retry attempts
    Duration::from_secs(1),   // Retry delay
)?;
```

## Examples

See the [`examples/`](examples/) directory for complete examples:

- **Basic Usage**: [`examples/basic_usage.rs`](examples/basic_usage.rs)
- **HTTP Client**: [`examples/http_client.rs`](examples/http_client.rs)  
- **Load Balancing**: [`examples/load_balancing.rs`](examples/load_balancing.rs)

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example http_client
cargo run --example load_balancing
```

## Error Handling

The SDK provides comprehensive error types:

```rust
use scoutquest_rust::ScoutQuestError;

match client.discover_service("unknown-service", None).await {
    Ok(instances) => println!("Found {} instances", instances.len()),
    Err(ScoutQuestError::ServiceNotFound { service_name }) => {
        println!("Service {} not found", service_name);
    }
    Err(ScoutQuestError::NetworkError(e)) => {
        println!("Network error: {}", e);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests  
cargo test --test integration_tests

# All tests with output
cargo test -- --nocapture
```

## Benchmarks

Run performance benchmarks:

```bash
cargo bench
```

This will benchmark load balancing strategies and other performance-critical operations.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Documentation

- [API Documentation](https://docs.rs/scoutquest-rust)
- [ScoutQuest Homepage](https://scoutquest.dev)
- [Examples](examples/)

## Support

- 📧 Email: team@scoutquest.dev
- 🐛 Issues: [GitHub Issues](https://github.com/scoutquest/scoutquest/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/scoutquest/scoutquest/discussions)
