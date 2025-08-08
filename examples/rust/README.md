# ScoutQuest Rust Examples

This directory contains examples demonstrating how to use the ScoutQuest Rust SDK.

## Examples

### 1. Axum Service Example

A simple Axum web service that integrates with ScoutQuest for service discovery.

```bash
cd axum_example
cargo run --bin axum_service
```

### 2. Notification Service Example

A complete notification service example with client SDK and server implementation, demonstrating service discovery, load balancing, and a REST API.

```bash
cd notification_example
cargo run --bin notification_server
```

## Prerequisites

1. **Start the ScoutQuest server**:
   ```bash
   cd ../../scoutquest-server
   cargo run
   ```

2. **Build the ScoutQuest Rust SDK**:
   ```bash
   cd ../../scoutquest-rust
   cargo build
   ```

## Running Examples

Each example is a separate Rust project with its own dependencies and README.

### Quick Start

1. **Start ScoutQuest server**:
   ```bash
   cd ../../scoutquest-server
   cargo run
   ```

2. **Run any example**:
   ```bash
   # Simple Axum service
   cd axum_example
   cargo run --bin axum_service
   
   # Or the complete notification service
   cd notification_example
   cargo run --bin notification_server
   ```

## Example Details

### Axum Example
- **Location**: `axum_example/`
- **Purpose**: Simple demonstration of ScoutQuest integration
- **Features**: Basic service registration, health checks

### Notification Example
- **Location**: `notification_example/`
- **Purpose**: Complete service with client SDK
- **Features**: Service discovery, load balancing, REST API, exportable client library

Each example includes its own README with detailed usage instructions.

```bash
# Service configuration
HOST=localhost
PORT=4000
ENVIRONMENT=development

# Service Discovery configuration
DISCOVERY_URL=http://localhost:8080
```

### 3. Test the Service

Once running, the service will be available at `http://localhost:4000` with these endpoints:

- **Health Check**: `GET /health`
- **Get Tasks**: `GET /api/tasks`
- **Create Task**: `POST /api/tasks`
- **Get Task**: `GET /api/tasks/{id}`
- **Call User Service**: `GET /api/call-user-service`
- **Call Product Service**: `GET /api/call-product-service`
- **Microservices Info**: `GET /api/microservices-info`

## Features Demonstrated

1. **Service Registration**: Automatically registers with SquoutQuest
2. **Health Checks**: Configures health check endpoints
3. **Service Discovery**: Discovers and calls other services
4. **Load Balancing**: Uses different load balancing strategies
5. **Metadata & Tags**: Adds service metadata and tags
6. **Graceful Shutdown**: Properly deregisters on shutdown

## Example API Calls

```bash
# Get all tasks
curl http://localhost:4000/api/tasks

# Create a new task
curl -X POST http://localhost:4000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "New Task", "description": "Task description"}'

# Get microservices information
curl http://localhost:4000/api/microservices-info
```

## Dashboard

View the SquoutQuest dashboard at: http://localhost:8080/dashboard

You should see your service registered with:
- Name: `task-service`
- Tags: `api`, `tasks`, `microservice`, `productivity`, `backend`
- Health check: `http://localhost:4000/health` 