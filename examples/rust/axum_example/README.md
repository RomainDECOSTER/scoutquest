# Axum Service Example with ScoutQuest

This is a simple example demonstrating how to create an Axum web service that integrates with ScoutQuest for service discovery.

## Features

- Basic Axum web server
- Service registration with ScoutQuest
- Health check endpoint
- Service discovery capabilities
- Structured logging with tracing

## Usage

### 1. Start the ScoutQuest server

First, make sure the ScoutQuest server is running:

```bash
cd ../../../scoutquest-server
cargo run
```

### 2. Run the Axum service

```bash
# From the axum_example directory
cargo run --bin axum_service
```

The service will:
- Register itself with ScoutQuest
- Start an HTTP server on port 3000
- Provide endpoints for testing service discovery

## Endpoints

- `GET /health` - Health check endpoint
- `GET /` - Welcome message
- `GET /services` - List all discovered services
- `GET /services/{name}` - Get instances of a specific service

## Testing

You can test the service using curl:

```bash
# Health check
curl http://localhost:3000/health

# Welcome message
curl http://localhost:3000/

# List all services
curl http://localhost:3000/services

# Get specific service instances
curl http://localhost:3000/services/axum-service
```
