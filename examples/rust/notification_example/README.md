# Notification Service Example with ScoutQuest

This example demonstrates how to create a notification service that uses ScoutQuest for service discovery. It is divided into two main parts:

1. **Exportable part**: Shared types and client SDK
2. **Server part**: Service that registers with ScoutQuest

## 🏗️ Structure

```
src/
├── lib.rs              # Library entry point (exportable)
├── types.rs            # Shared types (exportable)
├── client.rs           # Client SDK (exportable)
├── server.rs           # Server logic
└── bin/
    ├── notification_server.rs  # Server binary
    └── client_demo.rs          # Client usage demo
```

## 🚀 Usage

### 1. Start the ScoutQuest server

First, make sure the ScoutQuest server is running:

```bash
cd ../../scoutquest-server
cargo run
```

### 2. Start the notification service

```bash
# From the notification_example directory
cargo run --bin notification_server -- --port 3001
```

The server will:
- Automatically register with ScoutQuest
- Expose a REST API on port 3001
- Send regular heartbeats

### 3. Use the client

```bash
# Complete demonstration
cargo run --bin client_demo

# Check service health
cargo run --bin client_demo -- --action health

# Create a test notification
cargo run --bin client_demo -- --action create

# List notifications
cargo run --bin client_demo -- --action list
```

## 📚 Usage as a Library

To use this code in another project, add it as a dependency:

```toml
[dependencies]
notification-example = { path = "path/to/notification_example" }
```

Then use the client:

```rust
use notification_example::{
    NotificationClient,
    CreateNotificationRequest,
    Channel,
    Priority,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client
    let client = NotificationClient::new("http://localhost:8080", None)?;

    // Create a notification
    let request = CreateNotificationRequest {
        recipient: "user@example.com".to_string(),
        channel: Channel::Email,
        subject: Some("Test".to_string()),
        content: "Hello world!".to_string(),
        priority: Some(Priority::High),
        scheduled_at: None,
        metadata: None,
    };

    let notification = client.create_notification(request).await?;
    println!("Notification created: {}", notification.id);

    Ok(())
}
```

## 🔧 Service API

### Endpoints

- `GET /health` - Service health
- `POST /api/notifications` - Create a notification
- `GET /api/notifications` - List notifications (with filters)
- `GET /api/notifications/{id}` - Retrieve a notification
- `POST /api/notifications/{id}/send` - Send a notification
- `POST /api/notifications/{id}/cancel` - Cancel a notification

### Channel Types

- `Email` - Email notifications
- `Sms` - SMS messages
- `Push` - Mobile push notifications
- `Webhook` - HTTP calls
- `InApp` - In-app notifications

### Priority Levels

- `Low` - Low priority
- `Normal` - Normal priority (default)
- `High` - High priority
- `Critical` - Critical

## 🔍 Service Discovery

The client automatically uses ScoutQuest to:

1. **Discover** the service by name (`notification-service`)
2. **Select** a healthy instance (load balancing)
3. **Perform** HTTP calls to the selected instance

This enables high availability and automatic load balancing.

## 🧪 Testing

```bash
# Compile and check
cargo check

# Run tests
cargo test

# Build optimized version
cargo build --release
```

## 📦 Exportable Parts

The following elements can be used in other projects:

- **`types`**: All data types (structs, enums)
- **`client`**: SDK client for calling the service
- **`NotificationClient`**: Main client class
- **Convenience functions**: `create_client()`, etc.

The `server` module is not exported and remains internal to this service.

## 🌟 Features

- ✅ Automatic registration with ScoutQuest
- ✅ Automatic heartbeat to maintain registration
- ✅ Client-side service discovery
- ✅ Automatic load balancing
- ✅ Complete REST API
- ✅ Shared types with JSON serialization
- ✅ Robust error handling
- ✅ CLI with configurable options
- ✅ Documentation and examples
