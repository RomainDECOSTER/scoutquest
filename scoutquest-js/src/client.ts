import axios, { AxiosInstance, AxiosResponse, AxiosError } from 'axios';
import WebSocket from 'ws';
import { EventEmitter } from 'events';
import {
  ServiceInstance,
  Service,
  RegisterServiceRequest,
  DiscoveryQuery,
  UpdateStatusRequest,
  RegistryStats,
  ServiceEvent,
  ClientConfig,
  ServiceRegistrationOptions,
  DiscoveryResponse,
  InstanceStatus,
} from './types';
import { ScoutQuestError } from './errors';

/**
 * ScoutQuest Service Discovery Client for Node.js
 * 
 * Provides service registration, discovery, load balancing, and HTTP client functionality
 * for microservices using the ScoutQuest service discovery system.
 * 
 * @example
 * ```typescript
 * import { ScoutQuestClient } from '@scoutquest/sdk';
 * 
 * const client = new ScoutQuestClient('http://localhost:8080');
 * 
 * // Register a service
 * await client.registerService('my-service', 'localhost', 3000, {
 *   tags: ['api', 'v1'],
 *   metadata: { version: '1.0.0' }
 * });
 * 
 * // Discover services
 * const instances = await client.discoverService('other-service');
 * 
 * // Make HTTP calls to discovered services
 * const response = await client.get('other-service', '/api/users');
 * ```
 */
export class ScoutQuestClient extends EventEmitter {
  private readonly httpClient: AxiosInstance;
  private readonly discoveryUrl: string;
  private registeredInstance?: ServiceInstance;
  private heartbeatInterval?: any;
  private websocket?: WebSocket;
  private readonly config: Required<ClientConfig>;

  /**
   * Creates a new ScoutQuest client instance.
   * 
   * @param discoveryUrl - Base URL of the ScoutQuest discovery server
   * @param config - Optional client configuration
   */
  constructor(discoveryUrl: string, config: ClientConfig = {}) {
    super();
    
    this.discoveryUrl = discoveryUrl.replace(/\/+$/, ''); // Remove trailing slashes
    
    // Set default configuration
    this.config = {
      timeout: config.timeout ?? 30000,
      retry_attempts: config.retry_attempts ?? 3,
      retry_delay: config.retry_delay ?? 1000,
      headers: config.headers ?? {},
    };

    // Create HTTP client
    this.httpClient = axios.create({
      baseURL: this.discoveryUrl,
      timeout: this.config.timeout,
      headers: {
        'Content-Type': 'application/json',
        'User-Agent': '@scoutquest/sdk-js',
        ...this.config.headers,
      },
    });

    // Add response interceptor for error handling
    this.httpClient.interceptors.response.use(
      (response: AxiosResponse) => response,
      (error: AxiosError) => {
        throw this.handleHttpError(error);
      }
    );
  }

  /**
   * Registers a service instance with the discovery server.
   * 
   * @param serviceName - Name of the service
   * @param host - Host address
   * @param port - Port number
   * @param options - Registration options
   * @returns Promise resolving to the registered service instance
   */
  async registerService(
    serviceName: string,
    host: string,
    port: number,
    options: ServiceRegistrationOptions = {}
  ): Promise<ServiceInstance> {
    const request: RegisterServiceRequest = {
      service_name: serviceName,
      host,
      port,
      secure: options.secure ?? false,
      metadata: options.metadata,
      tags: options.tags,
      health_check: options.health_check,
    };

    try {
      const response = await this.retryRequest(() =>
        this.httpClient.post<ServiceInstance>('/api/services', request)
      );

      this.registeredInstance = response.data;

      // Start automatic heartbeat if enabled
      if (options.enable_heartbeat !== false) {
        this.startHeartbeat(options.heartbeat_interval ?? 30000);
      }

      this.emit('serviceRegistered', this.registeredInstance);
      return this.registeredInstance!;
    } catch (error) {
      if (error instanceof ScoutQuestError) {
        throw error;
      }
      throw ScoutQuestError.registrationFailed(
        error instanceof Error ? error.message : 'Unknown error',
        error
      );
    }
  }

  /**
   * Deregisters the currently registered service instance.
   */
  async deregisterService(): Promise<void> {
    if (!this.registeredInstance) {
      throw new ScoutQuestError(
        'No service instance is currently registered',
        'NO_REGISTERED_INSTANCE'
      );
    }

    try {
      await this.retryRequest(() =>
        this.httpClient.delete(
          `/api/services/${this.registeredInstance!.service_name}/instances/${this.registeredInstance!.id}`
        )
      );

      this.stopHeartbeat();
      this.emit('serviceDeregistered', this.registeredInstance);
      this.registeredInstance = undefined;
    } catch (error) {
      if (error instanceof ScoutQuestError) {
        throw error;
      }
      throw new ScoutQuestError(
        `Failed to deregister service: ${error instanceof Error ? error.message : 'Unknown error'}`,
        'DEREGISTRATION_FAILED'
      );
    }
  }

  /**
   * Discovers and selects a service instance. The server handles load balancing
   * and returns the best available instance.
   * 
   * @param serviceName - Name of the service to discover
   * @param query - Optional discovery parameters
   * @returns Promise resolving to a selected service instance
   */
  async discoverService(
    serviceName: string,
    query: DiscoveryQuery = {}
  ): Promise<ServiceInstance> {
    try {
      const response = await this.retryRequest(() =>
        this.httpClient.get<ServiceInstance>(`/api/discovery/${serviceName}`, {
          params: query,
        })
      );

      return response.data;
    } catch (error) {
      if (error instanceof ScoutQuestError && error.statusCode === 404) {
        throw ScoutQuestError.serviceNotFound(serviceName);
      }
      throw error;
    }
  }

  /**
   * Lists all instances of a service (for debugging/monitoring purposes).
   * 
   * @param serviceName - Name of the service
   * @param query - Optional discovery parameters
   * @returns Promise resolving to all service instances
   */
  async listServiceInstances(
    serviceName: string,
    query: DiscoveryQuery = {}
  ): Promise<ServiceInstance[]> {
    try {
      const response = await this.retryRequest(() =>
        this.httpClient.get<DiscoveryResponse>(`/api/services/${serviceName}/instances`, {
          params: query,
        })
      );

      return response.data.instances;
    } catch (error) {
      if (error instanceof ScoutQuestError && error.statusCode === 404) {
        throw ScoutQuestError.serviceNotFound(serviceName);
      }
      throw error;
    }
  }  /**
   * Gets a specific service by name.
   * 
   * @param serviceName - Name of the service
   * @returns Promise resolving to the service information
   */
  async getService(serviceName: string): Promise<Service> {
    try {
      const response = await this.retryRequest(() =>
        this.httpClient.get<Service>(`/api/services/${serviceName}`)
      );

      return response.data;
    } catch (error) {
      if (error instanceof ScoutQuestError && error.statusCode === 404) {
        throw ScoutQuestError.serviceNotFound(serviceName);
      }
      throw error;
    }
  }

  /**
   * Lists all registered services.
   * 
   * @returns Promise resolving to an array of all services
   */
  async listServices(): Promise<Service[]> {
    const response = await this.retryRequest(() =>
      this.httpClient.get<Service[]>('/api/services')
    );

    return response.data;
  }

  /**
   * Deletes a service and all its instances.
   * 
   * @param serviceName - Name of the service to delete
   */
  async deleteService(serviceName: string): Promise<void> {
    try {
      await this.retryRequest(() =>
        this.httpClient.delete(`/api/services/${serviceName}`)
      );
    } catch (error) {
      if (error instanceof ScoutQuestError && error.statusCode === 404) {
        throw ScoutQuestError.serviceNotFound(serviceName);
      }
      throw error;
    }
  }

  /**
   * Updates the status of the registered service instance.
   * 
   * @param status - New status
   */
  async updateStatus(status: InstanceStatus): Promise<void> {
    if (!this.registeredInstance) {
      throw new ScoutQuestError(
        'No service instance is currently registered',
        'NO_REGISTERED_INSTANCE'
      );
    }

    const request: UpdateStatusRequest = { status };

    await this.retryRequest(() =>
      this.httpClient.put(
        `/api/services/${this.registeredInstance!.service_name}/instances/${this.registeredInstance!.id}/status`,
        request
      )
    );

    this.registeredInstance.status = status;
    this.emit('statusUpdated', status);
  }

  /**
   * Sends a heartbeat for the registered service instance.
   */
  async sendHeartbeat(): Promise<void> {
    if (!this.registeredInstance) {
      throw new ScoutQuestError(
        'No service instance is currently registered',
        'NO_REGISTERED_INSTANCE'
      );
    }

    await this.retryRequest(() =>
      this.httpClient.post(
        `/api/services/${this.registeredInstance!.service_name}/instances/${this.registeredInstance!.id}/heartbeat`
      )
    );

    this.emit('heartbeatSent');
  }

  /**
   * Gets registry statistics.
   * 
   * @returns Promise resolving to registry stats
   */
  async getStats(): Promise<RegistryStats> {
    const response = await this.retryRequest(() =>
      this.httpClient.get<RegistryStats>('/health')
    );

    return response.data;
  }

  /**
   * Makes a GET request to a discovered service.
   * 
   * @param serviceName - Name of the target service
   * @param path - API path
   * @returns Promise resolving to the response data
   */
  async get<T = any>(
    serviceName: string,
    path: string
  ): Promise<T> {
    const instance = await this.discoverService(serviceName, { healthy_only: true });
    const url = this.buildUrl(instance, path);

    const response = await this.retryRequest(() =>
      axios.get<T>(url, { timeout: this.config.timeout })
    );

    return response.data;
  }

  /**
   * Makes a POST request to a discovered service.
   * 
   * @param serviceName - Name of the target service
   * @param path - API path
   * @param data - Request body data
   * @returns Promise resolving to the response data
   */
  async post<T = any>(
    serviceName: string,
    path: string,
    data?: any
  ): Promise<T> {
    const instance = await this.discoverService(serviceName, { healthy_only: true });
    const url = this.buildUrl(instance, path);

    const response = await this.retryRequest(() =>
      axios.post<T>(url, data, { timeout: this.config.timeout })
    );

    return response.data;
  }  /**
   * Makes a PUT request to a discovered service.
   * 
   * @param serviceName - Name of the target service
   * @param path - API path
   * @param data - Request body data
   * @returns Promise resolving to the response data
   */
  async put<T = any>(
    serviceName: string,
    path: string,
    data?: any
  ): Promise<T> {
    const instance = await this.discoverService(serviceName, { healthy_only: true });
    const url = this.buildUrl(instance, path);

    const response = await this.retryRequest(() =>
      axios.put<T>(url, data, { timeout: this.config.timeout })
    );

    return response.data;
  }

  /**
   * Makes a DELETE request to a discovered service.
   * 
   * @param serviceName - Name of the target service
   * @param path - API path
   * @returns Promise resolving to the response data
   */
  async delete<T = any>(
    serviceName: string,
    path: string
  ): Promise<T> {
    const instance = await this.discoverService(serviceName, { healthy_only: true });
    const url = this.buildUrl(instance, path);

    const response = await this.retryRequest(() =>
      axios.delete<T>(url, { timeout: this.config.timeout })
    );

    return response.data;
  }

  /**
   * Connects to the event stream via WebSocket to receive real-time service events.
   * 
   * @param serviceName - Optional service name to watch specific service events
   */
  connectEventStream(serviceName?: string): void {
    const wsUrl = this.discoveryUrl.replace(/^http/, 'ws') + '/ws';
    
    this.websocket = new WebSocket(wsUrl);
    
    this.websocket.on('open', () => {
      this.emit('connected');
      
      // Subscribe to specific service if provided
      if (serviceName) {
        this.websocket?.send(JSON.stringify({
          type: 'subscribe',
          service: serviceName,
        }));
      }
    });
    
    this.websocket.on('message', (data: any) => {
      try {
        const event: ServiceEvent = JSON.parse(data.toString());
        this.emit('serviceEvent', event);
        this.emit(event.event_type, event);
      } catch (error) {
        this.emit('error', new ScoutQuestError(
          'Failed to parse WebSocket message',
          'WEBSOCKET_PARSE_ERROR',
          undefined,
          error
        ));
      }
    });
    
    this.websocket.on('error', (error: any) => {
      this.emit('error', ScoutQuestError.network(
        'WebSocket connection error',
        error
      ));
    });
    
    this.websocket.on('close', () => {
      this.emit('disconnected');
    });
  }

  /**
   * Disconnects from the event stream.
   */
  disconnectEventStream(): void {
    if (this.websocket) {
      this.websocket.close();
      this.websocket = undefined;
    }
  }

  /**
   * Gracefully shuts down the client, deregistering services and cleaning up resources.
   */
  async shutdown(): Promise<void> {
    try {
      if (this.registeredInstance) {
        await this.deregisterService();
      }
    } catch (error) {
      // Log error but don't throw during shutdown
      this.emit('error', error);
    }

    this.stopHeartbeat();
    this.disconnectEventStream();
    this.removeAllListeners();
  }

  /**
   * Builds a complete URL for a service instance and path.
   */
  private buildUrl(instance: ServiceInstance, path: string): string {
    const protocol = instance.secure ? 'https' : 'http';
    const cleanPath = path.startsWith('/') ? path : `/${path}`;
    return `${protocol}://${instance.host}:${instance.port}${cleanPath}`;
  }

  /**
   * Starts automatic heartbeat for the registered service.
   */
  private startHeartbeat(interval: number): void {
    this.stopHeartbeat();
    
    this.heartbeatInterval = setInterval(async () => {
      try {
        await this.sendHeartbeat();
      } catch (error) {
        this.emit('heartbeatError', error);
      }
    }, interval);
  }

  /**
   * Stops automatic heartbeat.
   */
  private stopHeartbeat(): void {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = undefined;
    }
  }

  /**
   * Handles HTTP errors and converts them to ScoutQuestError.
   */
  private handleHttpError(error: AxiosError): ScoutQuestError {
    if (error.code === 'ECONNABORTED') {
      return ScoutQuestError.timeout('Request timeout');
    }

    if (error.response) {
      const message = (error.response.data as any)?.message || error.message;
      return ScoutQuestError.fromHttpResponse(
        error.response.status,
        message,
        error.response.data
      );
    }

    if (error.request) {
      return ScoutQuestError.network(
        'Network error: Unable to reach the server',
        error
      );
    }

    return new ScoutQuestError(
      error.message || 'Unknown HTTP error',
      'HTTP_ERROR'
    );
  }

  /**
   * Retries a request with exponential backoff.
   */
  private async retryRequest<T>(
    requestFn: () => Promise<AxiosResponse<T>>
  ): Promise<AxiosResponse<T>> {
    let lastError: any;

    for (let attempt = 0; attempt <= this.config.retry_attempts; attempt++) {
      try {
        return await requestFn();
      } catch (error) {
        lastError = error;

        // Don't retry on certain error types
        if (error instanceof ScoutQuestError) {
          if (error.statusCode && error.statusCode >= 400 && error.statusCode < 500) {
            throw error; // Client errors shouldn't be retried
          }
        }

        // If this was the last attempt, throw the error
        if (attempt === this.config.retry_attempts) {
          throw error;
        }

        // Wait before retrying with exponential backoff
        const delay = this.config.retry_delay * Math.pow(2, attempt);
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }

    throw lastError;
  }
}
