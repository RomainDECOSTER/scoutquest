/**
 * @fileoverview ScoutQuest SDK for Node.js - Main entry point
 *
 * This SDK provides service discovery and registration capabilities
 * for Node.js microservices using the ScoutQuest platform.
 *
 * @author ScoutQuest Team
 * @version 1.0.0
 */

// Main client
export { ScoutQuestClient } from './client';

// Types and interfaces
export * from './types';

// Errors
export { ScoutQuestError, isScoutQuestError } from './errors';

// Import for internal use
import { ScoutQuestClient } from './client';

// Convenience exports
export {
  ServiceInstance,
  Service,
  InstanceStatus,
  ServiceRegistrationOptions,
  ClientConfig,
  DiscoveryQuery,
} from './types';

/**
 * SDK version
 */
export const VERSION = '1.0.0';

/**
 * Creates a new ScoutQuest client instance with default configuration.
 *
 * @param discoveryUrl - Base URL of the ScoutQuest discovery server
 * @param config - Optional client configuration
 * @returns New ScoutQuest client instance
 *
 * @example
 * ```typescript
 * import { createClient } from '@scoutquest/sdk';
 *
 * const client = createClient('http://localhost:8080');
 * await client.registerService('my-service', 'localhost', 3000);
 * ```
 */
export function createClient(
  discoveryUrl: string,
  config?: import('./types').ClientConfig
) {
  return new ScoutQuestClient(discoveryUrl, config);
}

/**
 * Default export - the main client class
 */
export default ScoutQuestClient;
