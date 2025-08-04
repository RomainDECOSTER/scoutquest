# SquoutQuest Rust SDK

Rust SDK to interact with SquoutQuest Service Discovery.

## Installation

```toml
[dependencies]
scoutquest-rust = "1.0.0"
```

## Usage

```rust
use scoutquest_rust::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ServiceDiscoveryClient::new("http://localhost:8080")?;
    
    // Register a service
    client.register_service("my-service", "localhost", 3000, None).await?;
    
    // Discover services
    let instances = client.discover_service("other-service", None).await?;
    
    Ok(())
}
```

## Examples

See the `examples/` folder for complete examples.
