# ScoutQuest SDK for Node.js

[![npm version](https://badge.fury.io/js/%40scoutquest%2Fsdk.svg)](https://www.npmjs.com/package/@scoutquest/sdk)
[![TypeScript](https://img.shields.io/badge/%3C%2F%3E-TypeScript-%230074c1.svg)](http://www.typescriptlang.org/)
[![Node.js](https://img.shields.io/badge/node-%3E%3D16.0.0-brightgreen.svg)](https://nodejs.org/)
[![ESLint](https://img.shields.io/badge/ESLint-9.x-4B32C3.svg)](https://eslint.org/)

A TypeScript/JavaScript SDK for the ScoutQuest Service Discovery platform. This SDK provides service registration, discovery, load balancing, and HTTP client functionality for Node.js microservices.

## Features

- ✅ **Service Registration**: Register your service instances with the discovery server
- ✅ **Service Discovery**: Find and connect to other services
- ✅ **Load Balancing**: Multiple strategies (Round Robin, Random, Healthy Only, etc.)
- ✅ **HTTP Client**: Built-in HTTP client with automatic service discovery
- ✅ **Health Checks**: Automatic heartbeat and health monitoring
- ✅ **Real-time Events**: WebSocket-based service event notifications
- ✅ **TypeScript**: Full TypeScript support with type definitions
- ✅ **Dual Module**: Supports both CommonJS (CJS) and ES Modules (ESM)
- ✅ **Retry Logic**: Automatic retry with exponential backoff
- ✅ **Error Handling**: Comprehensive error types and handling

## Installation

```bash
npm install @scoutquest/sdk
```

## Quick Start

```typescript
import { ScoutQuestClient } from '@scoutquest/sdk';

// Create client
const client = new ScoutQuestClient('http://localhost:8080');

// Register your service
await client.registerService('my-service', 'localhost', 3000, {
  tags: ['api', 'v1'],
  metadata: { version: '1.0.0' },
  health_check: {
    url: '/health',
    interval_seconds: 30,
    timeout_seconds: 5,
    method: 'GET',
    expected_status: 200
  }
});

// Discover other services
const instances = await client.discoverService('other-service');

// Make HTTP calls to discovered services
const response = await client.get('other-service', '/api/users');
console.log(response);

// Graceful shutdown
process.on('SIGINT', async () => {
  await client.shutdown();
  process.exit(0);
});
```

## API Reference

### ScoutQuestClient

The main client class for interacting with ScoutQuest.

#### Constructor

```typescript
new ScoutQuestClient(discoveryUrl: string, config?: ClientConfig)
```

**Parameters:**
- `discoveryUrl`: Base URL of the ScoutQuest discovery server
- `config`: Optional client configuration

#### Configuration Options

```typescript
interface ClientConfig {
  timeout?: number;                    // Request timeout (default: 30000ms)
  retry_attempts?: number;             // Retry attempts (default: 3)
  retry_delay?: number;                // Base retry delay (default: 1000ms)
  default_strategy?: LoadBalancingStrategy; // Default LB strategy
  headers?: Record<string, string>;    // Additional HTTP headers
}
```

### Service Registration

#### registerService()

```typescript
await client.registerService(
  serviceName: string,
  host: string,
  port: number,
  options?: ServiceRegistrationOptions
): Promise<ServiceInstance>
```

**Registration Options:**

```typescript
interface ServiceRegistrationOptions {
  secure?: boolean;                    // HTTPS/TLS (default: false)
  metadata?: Record<string, string>;   // Custom metadata
  tags?: string[];                     // Service tags
  health_check?: HealthCheck;          // Health check config
  enable_heartbeat?: boolean;          // Auto heartbeat (default: true)
  heartbeat_interval?: number;         // Heartbeat interval (default: 30000ms)
}
```

#### deregisterService()

```typescript
await client.deregisterService(): Promise<void>
```

### Service Discovery

#### discoverService()

```typescript
await client.discoverService(
  serviceName: string,
  query?: DiscoveryQuery
): Promise<ServiceInstance[]>
```

**Discovery Query:**

```typescript
interface DiscoveryQuery {
  healthy_only?: boolean;              // Only healthy instances
  tags?: string;                       // Comma-separated tags
  limit?: number;                      // Max instances to return
  strategy?: LoadBalancingStrategy;    // Load balancing strategy
}
```

#### getService()

```typescript
await client.getService(serviceName: string): Promise<Service>
```

#### listServices()

```typescript
await client.listServices(): Promise<Service[]>
```

### HTTP Client Methods

The client includes built-in HTTP methods that automatically discover services and apply load balancing:

```typescript
// GET request
await client.get<ResponseType>('service-name', '/api/path', strategy?);

// POST request
await client.post<ResponseType>('service-name', '/api/path', data?, strategy?);

// PUT request
await client.put<ResponseType>('service-name', '/api/path', data?, strategy?);

// DELETE request
await client.delete<ResponseType>('service-name', '/api/path', strategy?);
```

### Load Balancing Strategies

```typescript
enum LoadBalancingStrategy {
  RoundRobin = 'RoundRobin',           // Distribute evenly
  Random = 'Random',                   // Random selection
  LeastConnections = 'LeastConnections', // Fewest connections
  WeightedRandom = 'WeightedRandom',   // Weighted random
  HealthyOnly = 'HealthyOnly'          // Only healthy instances
}
```

### Real-time Events

Listen to service registry events in real-time:

```typescript
// Connect to event stream
client.connectEventStream();

// Listen to events
client.on('serviceEvent', (event) => {
  console.log('Service event:', event);
});

client.on('ServiceRegistered', (event) => {
  console.log('New service registered:', event.service_name);
});

client.on('InstanceStatusChanged', (event) => {
  console.log('Instance status changed:', event);
});
```

### Health Management

#### updateStatus()

```typescript
import { InstanceStatus } from '@scoutquest/sdk';

await client.updateStatus(InstanceStatus.Up);
```

#### sendHeartbeat()

```typescript
await client.sendHeartbeat();
```

### Utility Methods

#### getStats()

```typescript
const stats = await client.getStats();
console.log('Registry stats:', stats);
```

#### shutdown()

```typescript
await client.shutdown(); // Graceful cleanup
```

## Error Handling

The SDK provides comprehensive error handling with specific error types:

```typescript
import { ScoutQuestError, isScoutQuestError } from '@scoutquest/sdk';

try {
  await client.discoverService('nonexistent-service');
} catch (error) {
  if (isScoutQuestError(error)) {
    console.log('Error code:', error.code);
    console.log('Status code:', error.statusCode);
    console.log('Details:', error.details);
  }
}
```

**Error Codes:**
- `SERVICE_NOT_FOUND`: Service doesn't exist
- `INSTANCE_NOT_FOUND`: Instance doesn't exist
- `REGISTRATION_FAILED`: Service registration failed
- `NETWORK_ERROR`: Network connectivity issues
- `TIMEOUT_ERROR`: Request timeout
- `VALIDATION_ERROR`: Invalid request data
- `NO_INSTANCES_AVAILABLE`: No service instances found
- `NO_HEALTHY_INSTANCES`: No healthy instances available

## Advanced Usage

### Custom Load Balancer

```typescript
import { LoadBalancer, LoadBalancingStrategy } from '@scoutquest/sdk';

const loadBalancer = new LoadBalancer();
const instance = loadBalancer.select(instances, LoadBalancingStrategy.Random);
```

### Manual Instance Selection

```typescript
// Discover instances manually
const instances = await client.discoverService('my-service', {
  healthy_only: true,
  tags: 'production,api',
  limit: 5
});

// Use external HTTP client
const selectedInstance = instances[0];
const url = `http://${selectedInstance.host}:${selectedInstance.port}/api/users`;
const response = await fetch(url);
```

### Service Tags and Metadata

```typescript
await client.registerService('user-service', 'localhost', 3000, {
  tags: ['api', 'users', 'v2', 'production'],
  metadata: {
    version: '2.1.0',
    region: 'us-east-1',
    datacenter: 'dc1',
    capabilities: 'crud,search'
  }
});

// Discover by tags
const instances = await client.discoverService('user-service', {
  tags: 'production,v2'
});
```

## TypeScript Support

The SDK is written in TypeScript and provides full type definitions:

```typescript
import {
  ScoutQuestClient,
  ServiceInstance,
  Service,
  LoadBalancingStrategy,
  InstanceStatus,
  ServiceRegistrationOptions,
  ClientConfig,
  DiscoveryQuery,
  ScoutQuestError
} from '@scoutquest/sdk';

// All types are fully typed
const client: ScoutQuestClient = new ScoutQuestClient('http://localhost:8080');
const instance: ServiceInstance = await client.registerService('api', 'localhost', 3000);
```

## CommonJS vs ES Modules

The SDK supports both module systems:

**ES Modules (ESM):**
```javascript
import { ScoutQuestClient } from '@scoutquest/sdk';
```

**CommonJS (CJS):**
```javascript
const { ScoutQuestClient } = require('@scoutquest/sdk');
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [GitHub Wiki](https://github.com/RomainDECOSTER/scoutquest/wiki)
- **Issues**: [GitHub Issues](https://github.com/RomainDECOSTER/scoutquest/issues)
- **Discussions**: [GitHub Discussions](https://github.com/RomainDECOSTER/scoutquest/discussions)

## Related Projects

- [ScoutQuest Server](../scoutquest-server) - The main discovery server
- [ScoutQuest Rust SDK](../scoutquest-rust) - Rust SDK for ScoutQuest
