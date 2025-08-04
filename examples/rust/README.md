# SquoutQuest Rust Example

This example demonstrates how to use the SquoutQuest Rust SDK with an Axum web service.

## Prerequisites

1. **Start the SquoutQuest server**:
   ```bash
   cd scoutquest-server
   cargo run
   ```

2. **Build the ScoutQuest Rust SDK**:
   ```bash
   cd scoutquest-rust
   cargo build
   ```

## Running the Example

### 1. Start the example service

```bash
cd examples/rust
cargo run --example axum_service
```

### 2. Environment Variables (Optional)

You can customize the service behavior with these environment variables:

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