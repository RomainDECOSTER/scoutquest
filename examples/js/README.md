# ScoutQuest JavaScript/TypeScript Examples

This directory contains examples demonstrating how to use the ScoutQuest JavaScript/TypeScript SDK for service discovery in Node.js applications.

## Examples Included

### 1. Express.js Web Service (`express-example.ts`)
A complete Express.js web service that demonstrates:
- Service registration with ScoutQuest
- Health check endpoints
- Service discovery capabilities
- Service-to-service communication
- REST API endpoints
- Graceful shutdown with deregistration

### 2. Client Demo (`client-demo.ts`)
A comprehensive demonstration of the ScoutQuest client featuring:
- Service discovery and listing
- Service communication (GET/POST requests)
- Service registration and deregistration
- Tag-based service searching
- Error handling examples

## Prerequisites

1. **ScoutQuest Server**: Ensure the ScoutQuest server is running
   ```bash
   cd ../../scoutquest-server
   cargo run
   ```

2. **Node.js**: Version 18+ recommended

## Installation

```bash
# Or using yarn
yarn install

# Or using pnpm
pnpm install
```

## Usage

### Running the Express.js Example

```bash
# Development mode with hot reload
pnpm dev

# Production mode
pnpm start:express

# Or directly with tsx
npx tsx src/express-example.ts
```

The service will:
- Register itself with ScoutQuest as `express-example-service`
- Start an HTTP server on port 3001 (configurable via `PORT` env var)
- Provide comprehensive API endpoints for testing

#### Available Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check endpoint |
| GET | `/` | Welcome message and API documentation |
| GET | `/api/tasks` | List all tasks (mock data) |
| POST | `/api/tasks` | Create a new task |
| GET | `/api/tasks/:id` | Get a specific task |
| GET | `/api/services` | List all discovered services |
| GET | `/api/services/:name` | Get instance of a specific service |
| GET | `/api/call-user-service` | Example: Call user service |
| GET | `/api/call-product-service` | Example: Call product service |
| GET | `/api/monitoring` | Service monitoring and health stats |

### Running the Client Demo

```bash
# Run the client demonstration
pnpm start:client

# Or directly with tsx
npx tsx src/client-demo.ts
```

The demo will demonstrate:
1. **Service Discovery**: Listing and discovering services
2. **Service Communication**: Making HTTP requests to services
3. **Service Registration**: Registering and deregistering a demo service
4. **Tag-based Search**: Finding services by tags
5. **Error Handling**: Proper error handling examples

## Configuration

### Environment Variables

- `SCOUTQUEST_URL`: URL of the ScoutQuest server (default: `http://localhost:8080`)
- `PORT`: Port for the Express service (default: `3001`)

### Examples

```bash
# Custom ScoutQuest server URL
SCOUTQUEST_URL=http://scoutquest.company.com:8080 pnpm start:express

# Custom port
PORT=4000 pnpm start:express
```

## Testing the Examples

### 1. Test Express Service Health

```bash
curl http://localhost:3001/health
```

### 2. List All Services

```bash
curl http://localhost:3001/api/services
```

### 3. Create a New Task

```bash
curl -X POST http://localhost:3001/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Learn ScoutQuest","description":"Master service discovery"}'
```

### 4. Get Service Monitoring

```bash
curl http://localhost:3001/api/monitoring
```

### 5. Call Another Service

```bash
# This will attempt to call a user service
curl http://localhost:3001/api/call-user-service
```

## Development

### Build TypeScript

```bash
pnpm build
```

### Clean Build Files

```bash
pnpm clean
```

## Architecture Overview

The examples demonstrate the simplified ScoutQuest architecture where:

1. **No Client-side Load Balancing**: The server handles load balancing and returns a ready-to-use service instance
2. **Service Discovery**: `discoverService()` returns a single `ServiceInstance`
3. **Simple Communication**: Direct HTTP calls using `get()`, `post()`, `put()`, `delete()` methods
4. **Automatic Registration**: Services register themselves on startup
5. **Graceful Shutdown**: Services deregister on shutdown

## Key Features Demonstrated

### Service Registration
```typescript
const registrationOptions: ServiceRegistrationOptions = {
  metadata: {
    version: '1.0.0',
    framework: 'express',
  },
  tags: ['api', 'web', 'example'],
};

await client.registerService(
  'my-service',
  'localhost',
  3001,
  registrationOptions
);
```

### Service Discovery
```typescript
// Get a ready-to-use service instance
const instance = await client.discoverService('user-service');
console.log(`Service URL: ${instance.host}:${instance.port}`);
```

### Service Communication
```typescript
// Simple HTTP calls
const users = await client.get('user-service', '/api/users');
const newUser = await client.post('user-service', '/api/users', userData);
```

### Error Handling
```typescript
try {
  const instance = await client.discoverService('service-name');
  const data = await client.get(instance, '/api/data');
} catch (error) {
  console.error('Service call failed:', error);
  // Handle gracefully
}
```

## Troubleshooting

### Common Issues

1. **ScoutQuest Server Not Running**
   ```
   Error: connect ECONNREFUSED ::1:8080
   ```
   **Solution**: Start the ScoutQuest server first

2. **Service Not Found**
   ```
   Error: Service 'user-service' not found
   ```
   **Solution**: Ensure the target service is registered with ScoutQuest

3. **Port Already in Use**
   ```
   Error: listen EADDRINUSE :::3001
   ```
   **Solution**: Use a different port with `PORT=3002 pnpm start:express`

### Debugging

Enable debug logging by setting the environment variable:
```bash
DEBUG=scoutquest* pnpm start:express
```

## Next Steps

After running these examples:

1. **Explore the Code**: Check the source files to understand the implementation
2. **Modify Examples**: Add your own endpoints and service calls
3. **Integration**: Integrate ScoutQuest into your existing Node.js applications
4. **Production**: Configure for production environments with proper error handling and monitoring

## Related Documentation

- [ScoutQuest JavaScript SDK Documentation](../../scoutquest-js/README.md)
- [ScoutQuest Server Documentation](../../scoutquest-server/README.md)
- [Main Project README](../../README.md)
