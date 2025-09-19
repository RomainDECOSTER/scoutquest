# ScoutQuest Rust SDK - Advanced Guide

This guide covers advanced usage patterns and best practices for the ScoutQuest Rust SDK.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Advanced Configuration](#advanced-configuration)
- [Error Handling Strategies](#error-handling-strategies)
- [Performance Optimization](#performance-optimization)
- [Production Deployment](#production-deployment)
- [Monitoring and Observability](#monitoring-and-observability)
- [Migration Guide](#migration-guide)

## Architecture Overview

The ScoutQuest Rust SDK is built around several core components:

### ServiceDiscoveryClient

The main client that orchestrates all operations:
- **Service Registration**: Manages the registration lifecycle with automatic heartbeats
- **Service Discovery**: Queries the discovery server for available services
- **Load Balancing**: Selects instances using various strategies
- **HTTP Client**: Built-in HTTP client with service discovery integration

### Load Balancer

Implements multiple load balancing strategies:
- **Random**: Good for general-purpose load distribution
- **Round Robin**: Ensures even distribution across instances
- **Healthy Only**: Filters out unhealthy instances
- **Least Connections**: Selects the instance with fewest active connections (planned)
- **Weighted Random**: Allows custom weighting of instances (planned)

### Error Management

Comprehensive error handling with specific error types for different failure scenarios.

## Advanced Configuration

### Custom HTTP Client Configuration

```rust
use scoutquest_rust::*;
use std::time::Duration;

// Create with custom timeouts and retry logic
let client = ServiceDiscoveryClient::with_config(
    "http://discovery.example.com:8080",
    Duration::from_secs(10),    // HTTP timeout
    5,                          // Retry attempts
    Duration::from_millis(500), // Retry delay
)?;
```

### Health Check Configuration

```rust
use scoutquest_rust::*;
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token123".to_string());

let health_check = HealthCheck {
    url: "/api/health".to_string(),
    interval_seconds: 15,       // Check every 15 seconds
    timeout_seconds: 3,         // 3 second timeout
    method: "GET".to_string(),
    expected_status: 200,
    headers: Some(headers),
};

let options = ServiceRegistrationOptions::new()
    .with_health_check(health_check);
```

### Service Metadata and Tags

```rust
use scoutquest_rust::*;
use std::collections::HashMap;

let mut metadata = HashMap::new();
metadata.insert("version".to_string(), "2.1.3".to_string());
metadata.insert("region".to_string(), "us-west-2".to_string());
metadata.insert("datacenter".to_string(), "dc1".to_string());
metadata.insert("instance_type".to_string(), "m5.large".to_string());

let options = ServiceRegistrationOptions::new()
    .with_metadata(metadata)
    .with_tags(vec![
        "production".to_string(),
        "api".to_string(),
        "v2".to_string(),
        "critical".to_string(),
    ])
    .with_secure(true); // Enable HTTPS
```

## Error Handling Strategies

### Comprehensive Error Handling

```rust
use scoutquest_rust::*;

async fn robust_service_call() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = ServiceDiscoveryClient::new("http://localhost:8080")?;

    match client.get::<serde_json::Value>("user-service", "/api/users").await {
        Ok(response) => Ok(response),
        Err(ScoutQuestError::ServiceNotFound { service_name }) => {
            eprintln!("Service {} not found in discovery", service_name);
            // Fallback to default endpoint
            let fallback_response = serde_json::json!({ "users": [] });
            Ok(fallback_response)
        }
        Err(ScoutQuestError::NoHealthyInstances { service_name }) => {
            eprintln!("No healthy instances for {}", service_name);
            // Could trigger circuit breaker or alerting
            Err("Service temporarily unavailable".into())
        }
        Err(ScoutQuestError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
            // Could retry with exponential backoff
            Err(e.into())
        }
        Err(ScoutQuestError::Timeout) => {
            eprintln!("Request timed out");
            // Could use cached response or circuit breaker
            Err("Request timeout".into())
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            Err(e.into())
        }
    }
}
```

### Circuit Breaker Pattern

```rust
use scoutquest_rust::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct CircuitBreaker {
    failure_count: Arc<AtomicU32>,
    last_failure: Arc<std::sync::Mutex<Option<Instant>>>,
    threshold: u32,
    timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_count: Arc::new(AtomicU32::new(0)),
            last_failure: Arc::new(std::sync::Mutex::new(None)),
            threshold,
            timeout,
        }
    }

    pub fn is_open(&self) -> bool {
        let count = self.failure_count.load(Ordering::Relaxed);
        if count >= self.threshold {
            let last_failure = self.last_failure.lock().unwrap();
            if let Some(last) = *last_failure {
                return last.elapsed() < self.timeout;
            }
        }
        false
    }

    pub fn record_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        *self.last_failure.lock().unwrap() = Some(Instant::now());
    }
}
```

## Performance Optimization

### Connection Pooling

The SDK uses `reqwest` which automatically handles connection pooling. For high-throughput applications, consider:

```rust
use scoutquest_rust::*;
use std::time::Duration;

// Configure for high-throughput scenarios
let client = ServiceDiscoveryClient::with_config(
    "http://localhost:8080",
    Duration::from_secs(5),     // Shorter timeout for faster failover
    2,                          // Fewer retries for lower latency
    Duration::from_millis(100), // Shorter retry delay
)?;
```

### Caching Strategies

```rust
use scoutquest_rust::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub struct ServiceCache {
    cache: Arc<RwLock<HashMap<String, (Vec<ServiceInstance>, Instant)>>>,
    ttl: Duration,
}

impl ServiceCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub async fn get_or_discover(
        &self,
        client: &ServiceDiscoveryClient,
        service_name: &str,
    ) -> Result<Vec<ServiceInstance>, ScoutQuestError> {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some((instances, timestamp)) = cache.get(service_name) {
                if timestamp.elapsed() < self.ttl {
                    return Ok(instances.clone());
                }
            }
        }

        // Cache miss or expired, fetch from discovery
        let instances = client.discover_service(service_name, None).await?;

        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(service_name.to_string(), (instances.clone(), Instant::now()));
        }

        Ok(instances)
    }
}
```

## Production Deployment

### Environment Configuration

```rust
use scoutquest_rust::*;
use std::env;
use std::time::Duration;

pub fn create_production_client() -> Result<ServiceDiscoveryClient, ScoutQuestError> {
    let discovery_url = env::var("SCOUTQUEST_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let timeout_secs: u64 = env::var("SCOUTQUEST_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);

    let retry_attempts: usize = env::var("SCOUTQUEST_RETRIES")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap_or(3);

    ServiceDiscoveryClient::with_config(
        &discovery_url,
        Duration::from_secs(timeout_secs),
        retry_attempts,
        Duration::from_secs(1),
    )
}
```

### Graceful Shutdown

```rust
use scoutquest_rust::*;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ServiceDiscoveryClient::new("http://localhost:8080")?;

    // Register service
    client.register_service("my-service", "localhost", 8080, None).await?;

    // Set up graceful shutdown
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal");
        }
        _ = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())? => {
            println!("Received SIGTERM");
        }
    }

    // Graceful cleanup
    println!("Deregistering service...");
    client.deregister().await?;
    println!("Service deregistered successfully");

    Ok(())
}
```

## Monitoring and Observability

### Tracing Integration

The SDK uses the `tracing` crate for structured logging:

```rust
use scoutquest_rust::*;
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let client = ServiceDiscoveryClient::new("http://localhost:8080")?;

    // All SDK operations will emit structured logs
    let instance = client.register_service("my-service", "localhost", 8080, None).await?;
    info!(service_id = %instance.id, "Service registered successfully");

    Ok(())
}
```

### Metrics Collection

```rust
use scoutquest_rust::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct Metrics {
    pub discovery_calls: Arc<AtomicU64>,
    pub registration_calls: Arc<AtomicU64>,
    pub http_calls: Arc<AtomicU64>,
    pub errors: Arc<AtomicU64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            discovery_calls: Arc::new(AtomicU64::new(0)),
            registration_calls: Arc::new(AtomicU64::new(0)),
            http_calls: Arc::new(AtomicU64::new(0)),
            errors: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_discovery_call(&self) {
        self.discovery_calls.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_registration_call(&self) {
        self.registration_calls.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_http_call(&self) {
        self.http_calls.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}
```

## Migration Guide

### From Version 0.x to 1.0

1. **Import Changes**: Update your imports to use the new module structure
2. **Error Handling**: Update error handling to use the new `ScoutQuestError` enum
3. **Configuration**: Use the new `with_config` method for custom configuration
4. **Load Balancing**: Update to use the new `LoadBalancingStrategy` enum

### Breaking Changes

- `ServiceDiscoveryClient::new()` now returns a `Result`
- Error types have been consolidated into `ScoutQuestError`
- Load balancing strategies are now an enum instead of strings
- Health check configuration has been restructured

### Migration Example

```rust
// Old (0.x)
let client = ServiceDiscoveryClient::new("http://localhost:8080");
let instances = client.discover("service-name").await?;

// New (1.0)
let client = ServiceDiscoveryClient::new("http://localhost:8080")?;
let instances = client.discover_service("service-name", None).await?;
```

## Best Practices

1. **Always handle errors gracefully** - Use proper error handling patterns
2. **Use structured logging** - Leverage the built-in tracing integration
3. **Implement health checks** - Configure appropriate health check endpoints
4. **Cache discovery results** - Implement caching for high-traffic scenarios
5. **Monitor metrics** - Track key metrics for operational visibility
6. **Graceful shutdown** - Always deregister services on shutdown
7. **Environment configuration** - Use environment variables for configuration
8. **Circuit breaker pattern** - Implement circuit breakers for resilience

## Troubleshooting

### Common Issues

1. **Connection Refused**: Ensure the ScoutQuest server is running and accessible
2. **Service Not Found**: Check service name spelling and registration status
3. **Health Check Failures**: Verify health check endpoint is accessible
4. **Load Balancing Issues**: Ensure instances are healthy and properly registered

### Debug Logging

Enable debug logging to troubleshoot issues:

```rust
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

This will show detailed information about all SDK operations.
