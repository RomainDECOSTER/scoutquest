/**
 * Represents the operational status of a service instance.
 */
export enum InstanceStatus {
  /** Service is running and ready to accept requests */
  Up = 'Up',
  /** Service is not responding or has failed */
  Down = 'Down',
  /** Service is in the process of starting up */
  Starting = 'Starting',
  /** Service is gracefully shutting down */
  Stopping = 'Stopping',
  /** Service is running but temporarily out of service */
  OutOfService = 'OutOfService',
  /** Service status is unknown or could not be determined */
  Unknown = 'Unknown',
}

/**
 * Event types for service discovery system events.
 */
export enum EventType {
  /** Service instance was registered */
  ServiceRegistered = 'ServiceRegistered',
  /** Service instance was deregistered */
  ServiceDeregistered = 'ServiceDeregistered',
  /** Service instance status changed */
  InstanceStatusChanged = 'InstanceStatusChanged',
  /** Health check failed for an instance */
  HealthCheckFailed = 'HealthCheckFailed',
  /** Health check passed for an instance */
  HealthCheckPassed = 'HealthCheckPassed',
}

/**
 * Health check configuration for a service instance.
 */
export interface HealthCheck {
  /** URL endpoint for health checks */
  url: string;
  /** Interval between health checks in seconds */
  interval_seconds: number;
  /** Request timeout in seconds */
  timeout_seconds: number;
  /** HTTP method to use for health checks */
  method: string;
  /** Expected HTTP status code for healthy responses */
  expected_status: number;
  /** Optional headers to include in health check requests */
  headers?: Record<string, string>;
}

/**
 * Represents a service instance in the ScoutQuest discovery system.
 */
export interface ServiceInstance {
  /** Unique identifier for this service instance */
  id: string;
  /** Name of the service this instance belongs to */
  service_name: string;
  /** Hostname or IP address where the service is running */
  host: string;
  /** Port number where the service is listening */
  port: number;
  /** Whether the service uses HTTPS/TLS */
  secure: boolean;
  /** Current operational status */
  status: InstanceStatus;
  /** Additional metadata about the service */
  metadata: Record<string, any>;
  /** Tags for service categorization and filtering */
  tags: string[];
  /** Timestamp when the instance was registered */
  registered_at: string;
  /** Timestamp of the last heartbeat received */
  last_heartbeat: string;
  /** Timestamp of the last status change */
  last_status_change: string;
}

/**
 * Represents a complete service with all its instances.
 */
export interface Service {
  /** Name of the service */
  name: string;
  /** List of all instances for this service */
  instances: ServiceInstance[];
  /** Service tags */
  tags: string[];
  /** Timestamp when the service was first created */
  created_at: string;
  /** Timestamp when the service was last updated */
  updated_at: string;
}

/**
 * Request payload for registering a service instance.
 */
export interface RegisterServiceRequest {
  /** Name of the service to register */
  service_name: string;
  /** Hostname or IP address */
  host: string;
  /** Port number */
  port: number;
  /** Whether the service uses HTTPS/TLS */
  secure: boolean;
  /** Additional service metadata */
  metadata?: Record<string, any>;
  /** Service tags for categorization */
  tags?: string[];
  /** Health check configuration */
  health_check?: HealthCheck;
}

/**
 * Query parameters for service discovery.
 */
export interface DiscoveryQuery {
  /** Only return healthy instances */
  healthy_only?: boolean;
  /** Filter by comma-separated tags */
  tags?: string;
  /** Maximum number of instances to return */
  limit?: number;
}

/**
 * Request payload for updating instance status.
 */
export interface UpdateStatusRequest {
  /** New status for the instance */
  status: InstanceStatus;
}

/**
 * Statistics about the service registry.
 */
export interface RegistryStats {
  /** Total number of services registered */
  total_services: number;
  /** Total number of service instances */
  total_instances: number;
  /** Number of healthy service instances */
  healthy_instances: number;
  /** Timestamp when the registry started */
  start_time: number;
}

/**
 * Event data structure for service discovery events.
 */
export interface ServiceEvent {
  /** Type of the event */
  event_type: EventType;
  /** Name of the service involved */
  service_name: string;
  /** ID of the instance involved (if applicable) */
  instance_id?: string;
  /** Timestamp when the event occurred */
  timestamp: string;
  /** Additional event details */
  details?: Record<string, any>;
}

/**
 * Options for service registration.
 */
export interface ServiceRegistrationOptions {
  /** Whether the service uses HTTPS/TLS */
  secure?: boolean;
  /** Additional service metadata */
  metadata?: Record<string, any>;
  /** Service tags for categorization */
  tags?: string[];
  /** Enable automatic heartbeat */
  enable_heartbeat?: boolean;
  /** Heartbeat interval in milliseconds */
  heartbeat_interval?: number;
  /** Health check configuration */
  health_check?: HealthCheck;
}

/**
 * Configuration options for the ScoutQuest client.
 */
export interface ClientConfig {
  /** Request timeout in milliseconds */
  timeout?: number;
  /** Number of retry attempts for failed requests */
  retry_attempts?: number;
  /** Base delay between retries in milliseconds */
  retry_delay?: number;
  /** Additional HTTP headers to include in requests */
  headers?: Record<string, string>;
}

/**
 * Response from service discovery endpoint.
 */
export interface DiscoveryResponse {
  /** List of discovered service instances */
  instances: ServiceInstance[];
}
