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
  /** Current status of the service instance */
  status: InstanceStatus;
  /** Custom metadata key-value pairs */
  metadata: Record<string, string>;
  /** Tags associated with this service instance */
  tags: string[];
  /** Health check configuration */
  health_check?: HealthCheck;
  /** Timestamp when the service was first registered */
  registered_at: string;
  /** Timestamp of the last heartbeat received */
  last_heartbeat: string;
  /** Timestamp when the status last changed */
  last_status_change: string;
}

/**
 * Represents a service with all its instances.
 */
export interface Service {
  /** Name of the service */
  name: string;
  /** List of service instances */
  instances: ServiceInstance[];
  /** Tags associated with the service */
  tags: string[];
  /** Timestamp when the service was created */
  created_at: string;
  /** Timestamp when the service was last updated */
  updated_at: string;
}

/**
 * Load balancing strategies for service discovery.
 */
export enum LoadBalancingStrategy {
  /** Distribute requests evenly across instances */
  RoundRobin = 'RoundRobin',
  /** Select instances randomly */
  Random = 'Random',
  /** Route to instance with fewest active connections */
  LeastConnections = 'LeastConnections',
  /** Random selection with instance weights */
  WeightedRandom = 'WeightedRandom',
  /** Only select healthy instances */
  HealthyOnly = 'HealthyOnly',
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
  secure?: boolean;
  /** Custom metadata */
  metadata?: Record<string, string>;
  /** Service tags */
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
  /** Load balancing strategy to use */
  strategy?: LoadBalancingStrategy;
}

/**
 * Request payload for updating instance status.
 */
export interface UpdateStatusRequest {
  /** New status for the instance */
  status: InstanceStatus;
}

/**
 * Registry statistics information.
 */
export interface RegistryStats {
  /** Total number of services registered */
  total_services: number;
  /** Total number of service instances */
  total_instances: number;
  /** Number of healthy instances */
  healthy_instances: number;
  /** Server start time (Unix timestamp) */
  start_time: number;
}

/**
 * Event types for service registry events.
 */
export enum EventType {
  ServiceRegistered = 'ServiceRegistered',
  ServiceDeregistered = 'ServiceDeregistered',
  InstanceStatusChanged = 'InstanceStatusChanged',
  HealthCheckFailed = 'HealthCheckFailed',
  HealthCheckPassed = 'HealthCheckPassed',
}

/**
 * Service registry event.
 */
export interface ServiceEvent {
  /** Type of event */
  event_type: EventType;
  /** Name of the service */
  service_name: string;
  /** Instance ID (if applicable) */
  instance_id?: string;
  /** Event timestamp */
  timestamp: string;
  /** Additional event details */
  details: any;
}

/**
 * Options for service registration.
 */
export interface ServiceRegistrationOptions {
  /** Whether the service uses HTTPS/TLS */
  secure?: boolean;
  /** Custom metadata */
  metadata?: Record<string, string>;
  /** Service tags */
  tags?: string[];
  /** Health check configuration */
  health_check?: HealthCheck;
  /** Enable automatic heartbeat (default: true) */
  enable_heartbeat?: boolean;
  /** Heartbeat interval in milliseconds (default: 30000) */
  heartbeat_interval?: number;
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
  /** Default load balancing strategy */
  default_strategy?: LoadBalancingStrategy;
  /** Additional HTTP headers to include in requests */
  headers?: Record<string, string>;
}

/**
 * Response from service discovery endpoint.
 */
export interface DiscoveryResponse {
  /** List of discovered service instances */
  instances: ServiceInstance[];
  /** Load balancing strategy used */
  strategy: LoadBalancingStrategy;
}
