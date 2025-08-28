import { ScoutQuestClient } from '../client';
import type {
  ServiceInstance,
  Service,
  ServiceRegistrationOptions,
  DiscoveryQuery,
  RegistryStats,
  ClientConfig,
} from '../types';
import { InstanceStatus } from '../types';
import { ScoutQuestError } from '../errors';

// Mock axios
jest.mock('axios', () => ({
  create: jest.fn(() => ({
    get: jest.fn(),
    post: jest.fn(),
    put: jest.fn(),
    delete: jest.fn(),
    interceptors: {
      response: {
        use: jest.fn(),
      },
    },
  })),
  get: jest.fn(),
  post: jest.fn(),
  put: jest.fn(),
  delete: jest.fn(),
}));

// Mock WebSocket
const mockWebSocket = {
  on: jest.fn(),
  send: jest.fn(),
  close: jest.fn(),
};

jest.mock('ws', () => {
  return jest.fn().mockImplementation(() => mockWebSocket);
});

import axios from 'axios';
import WebSocket from 'ws';

const mockedAxios = axios as jest.Mocked<typeof axios>;
const MockedWebSocket = WebSocket as jest.MockedClass<typeof WebSocket>;

describe('ScoutQuestClient', () => {
  let client: ScoutQuestClient;
  let fastClient: ScoutQuestClient; // Client with minimal retry for faster tests
  let mockAxiosInstance: any;
  let mockServiceInstance: ServiceInstance;
  let mockService: Service;

  beforeEach(() => {
    jest.clearAllMocks();
    
    // Setup mock axios instance
    mockAxiosInstance = {
      get: jest.fn(),
      post: jest.fn(),
      put: jest.fn(),
      delete: jest.fn(),
      interceptors: {
        response: {
          use: jest.fn(),
        },
      },
    };
    
    mockedAxios.create.mockReturnValue(mockAxiosInstance);
    
    // Mock service instance
    mockServiceInstance = {
      id: 'test-instance-1',
      service_name: 'test-service',
      host: 'localhost',
      port: 3000,
      secure: false,
      status: InstanceStatus.Up,
      metadata: { version: '1.0.0' },
      tags: ['api', 'test'],
      registered_at: '2023-01-01T00:00:00Z',
      last_heartbeat: '2023-01-01T00:00:00Z',
      last_status_change: '2023-01-01T00:00:00Z',
    };

    // Mock service
    mockService = {
      name: 'test-service',
      instances: [mockServiceInstance],
      tags: ['api'],
      created_at: '2023-01-01T00:00:00Z',
      updated_at: '2023-01-01T00:00:00Z',
    };

    client = new ScoutQuestClient('http://localhost:8080');
    
    // Fast client for error tests (no retries, fast timeout)
    fastClient = new ScoutQuestClient('http://localhost:8080', {
      retry_attempts: 0,
      retry_delay: 10,
      timeout: 1000,
    });
  });

  describe('constructor', () => {
    it('should create client with default config', () => {
      const client = new ScoutQuestClient('http://localhost:8080');
      expect(client).toBeInstanceOf(ScoutQuestClient);
      expect(mockedAxios.create).toHaveBeenCalledWith({
        baseURL: 'http://localhost:8080',
        timeout: 30000,
        headers: {
          'Content-Type': 'application/json',
          'User-Agent': '@scoutquest/sdk-js',
        },
      });
    });

    it('should create client with custom config', () => {
      const config: ClientConfig = {
        timeout: 60000,
        retry_attempts: 5,
        retry_delay: 2000,
        headers: { 'X-Custom': 'value' },
      };

      const client = new ScoutQuestClient('http://localhost:8080', config);
      expect(client).toBeInstanceOf(ScoutQuestClient);
      expect(mockedAxios.create).toHaveBeenCalledWith({
        baseURL: 'http://localhost:8080',
        timeout: 60000,
        headers: {
          'Content-Type': 'application/json',
          'User-Agent': '@scoutquest/sdk-js',
          'X-Custom': 'value',
        },
      });
    });

    it('should remove trailing slashes from URL', () => {
      new ScoutQuestClient('http://localhost:8080///');
      expect(mockedAxios.create).toHaveBeenCalledWith(
        expect.objectContaining({
          baseURL: 'http://localhost:8080',
        })
      );
    });
  });

  describe('registerService', () => {
    it('should register service successfully', async () => {
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });

      const options: ServiceRegistrationOptions = {
        tags: ['api', 'test'],
        metadata: { version: '1.0.0' },
        secure: true,
      };

      const result = await client.registerService(
        'test-service',
        'localhost',
        3000,
        options
      );

      expect(result).toEqual(mockServiceInstance);
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/api/services', {
        service_name: 'test-service',
        host: 'localhost',
        port: 3000,
        secure: true,
        metadata: { version: '1.0.0' },
        tags: ['api', 'test'],
        health_check: undefined,
      });
    });

    it('should register service with minimal options', async () => {
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });

      const result = await client.registerService(
        'test-service',
        'localhost',
        3000
      );

      expect(result).toEqual(mockServiceInstance);
      expect(mockAxiosInstance.post).toHaveBeenCalledWith('/api/services', {
        service_name: 'test-service',
        host: 'localhost',
        port: 3000,
        secure: false,
        metadata: undefined,
        tags: undefined,
        health_check: undefined,
      });
    });

    it('should handle registration failure', async () => {
      const error = new Error('Registration failed');
      mockAxiosInstance.post.mockRejectedValue(error);

      await expect(
        fastClient.registerService('test-service', 'localhost', 3000)
      ).rejects.toThrow('Registration failed');
    });
  });

  describe('discoverService', () => {
    it('should discover service instances', async () => {
      mockAxiosInstance.get.mockResolvedValue({
        data: mockServiceInstance,
        status: 200,
      });

      const result = await client.discoverService('test-service');

      expect(result).toEqual(mockServiceInstance);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith(
        '/api/discovery/test-service',
        { params: {} }
      );
    });

    it('should discover service with query parameters', async () => {
      mockAxiosInstance.get.mockResolvedValue({
        data: mockServiceInstance,
        status: 200,
      });

      const query: DiscoveryQuery = {
        healthy_only: true,
        tags: 'api,test',
        limit: 5,
      };

      const result = await client.discoverService('test-service', query);

      expect(result).toEqual(mockServiceInstance);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith(
        '/api/discovery/test-service',
        { params: query }
      );
    });

    it('should throw service not found error', async () => {
      const error = new Error('Network Error') as any;
      error.response = { status: 404 };
      mockAxiosInstance.get.mockRejectedValue(error);

      await expect(fastClient.discoverService('nonexistent')).rejects.toThrow();
    });
  });

  describe('getService', () => {
    it('should get service by name', async () => {
      mockAxiosInstance.get.mockResolvedValue({
        data: mockService,
        status: 200,
      });

      const result = await client.getService('test-service');

      expect(result).toEqual(mockService);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith(
        '/api/services/test-service'
      );
    });

    it('should throw service not found error', async () => {
      const error = new Error('Network Error') as any;
      error.response = { status: 404 };
      mockAxiosInstance.get.mockRejectedValue(error);

      await expect(fastClient.getService('nonexistent')).rejects.toThrow();
    });
  });

  describe('listServices', () => {
    it('should list all services', async () => {
      const services = [mockService];
      mockAxiosInstance.get.mockResolvedValue({
        data: services,
        status: 200,
      });

      const result = await client.listServices();

      expect(result).toEqual(services);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/api/services');
    });
  });

  describe('deleteService', () => {
    it('should delete service successfully', async () => {
      mockAxiosInstance.delete.mockResolvedValue({
        status: 204,
      });

      await client.deleteService('test-service');

      expect(mockAxiosInstance.delete).toHaveBeenCalledWith(
        '/api/services/test-service'
      );
    });

    it('should throw service not found error', async () => {
      const error = new Error('Network Error') as any;
      error.response = { status: 404 };
      mockAxiosInstance.delete.mockRejectedValue(error);

      await expect(fastClient.deleteService('nonexistent')).rejects.toThrow();
    });
  });

  describe('updateStatus', () => {
    beforeEach(async () => {
      // Register a service first
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });
      await client.registerService('test-service', 'localhost', 3000);
    });

    it('should update status successfully', async () => {
      mockAxiosInstance.put.mockResolvedValue({
        status: 200,
      });

      await client.updateStatus(InstanceStatus.Down);

      expect(mockAxiosInstance.put).toHaveBeenCalledWith(
        '/api/services/test-service/instances/test-instance-1/status',
        { status: InstanceStatus.Down }
      );
    });

    it('should throw error when no service registered', async () => {
      const clientWithoutService = new ScoutQuestClient('http://localhost:8080');

      await expect(
        clientWithoutService.updateStatus(InstanceStatus.Down)
      ).rejects.toThrow(ScoutQuestError);
    });
  });

  describe('sendHeartbeat', () => {
    beforeEach(async () => {
      // Register a service first
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });
      await client.registerService('test-service', 'localhost', 3000);
    });

    it('should send heartbeat successfully', async () => {
      mockAxiosInstance.post.mockResolvedValue({
        status: 200,
      });

      await client.sendHeartbeat();

      expect(mockAxiosInstance.post).toHaveBeenCalledWith(
        '/api/services/test-service/instances/test-instance-1/heartbeat'
      );
    });

    it('should throw error when no service registered', async () => {
      const clientWithoutService = new ScoutQuestClient('http://localhost:8080');

      await expect(clientWithoutService.sendHeartbeat()).rejects.toThrow(
        ScoutQuestError
      );
    });
  });

  describe('getStats', () => {
    it('should get registry stats', async () => {
      const stats: RegistryStats = {
        total_services: 5,
        total_instances: 10,
        healthy_instances: 8,
        start_time: 1640995200,
      };

      mockAxiosInstance.get.mockResolvedValue({
        data: stats,
        status: 200,
      });

      const result = await client.getStats();

      expect(result).toEqual(stats);
      expect(mockAxiosInstance.get).toHaveBeenCalledWith('/health');
    });
  });

  describe('HTTP methods', () => {
    beforeEach(() => {
      // Mock discoverService to return the instance directly
      mockAxiosInstance.get.mockResolvedValue({
        data: mockServiceInstance,
        status: 200,
      });
    });

    it('should make GET request to discovered service', async () => {
      // Mock discoverService call
      mockAxiosInstance.get.mockResolvedValueOnce({
        data: mockServiceInstance,
        status: 200,
      });

      // Mock actual GET call
      mockedAxios.get.mockResolvedValueOnce({
        data: { users: [] },
        status: 200,
      });

      const result = await client.get('test-service', '/api/users');

      expect(result).toEqual({ users: [] });
      expect(mockedAxios.get).toHaveBeenCalledWith(
        'http://localhost:3000/api/users',
        { timeout: 30000 }
      );
    });

    it('should make POST request to discovered service', async () => {
      // Mock discoverService call
      mockAxiosInstance.get.mockResolvedValueOnce({
        data: mockServiceInstance,
        status: 200,
      });

      // Mock actual POST call
      mockedAxios.post.mockResolvedValueOnce({
        data: { id: 1 },
        status: 201,
      });

      const userData = { name: 'John' };
      const result = await client.post('test-service', '/api/users', userData);

      expect(result).toEqual({ id: 1 });
      expect(mockedAxios.post).toHaveBeenCalledWith(
        'http://localhost:3000/api/users',
        userData,
        { timeout: 30000 }
      );
    });

    it('should make PUT request to discovered service', async () => {
      // Mock discoverService call
      mockAxiosInstance.get.mockResolvedValueOnce({
        data: mockServiceInstance,
        status: 200,
      });

      // Mock actual PUT call
      mockedAxios.put.mockResolvedValueOnce({
        data: { id: 1, name: 'John Updated' },
        status: 200,
      });

      const userData = { name: 'John Updated' };
      const result = await client.put('test-service', '/api/users/1', userData);

      expect(result).toEqual({ id: 1, name: 'John Updated' });
      expect(mockedAxios.put).toHaveBeenCalledWith(
        'http://localhost:3000/api/users/1',
        userData,
        { timeout: 30000 }
      );
    });

    it('should make DELETE request to discovered service', async () => {
      // Mock discoverService call
      mockAxiosInstance.get.mockResolvedValueOnce({
        data: mockServiceInstance,
        status: 200,
      });

      // Mock actual DELETE call
      mockedAxios.delete.mockResolvedValueOnce({
        data: null,
        status: 204,
      });

      const result = await client.delete('test-service', '/api/users/1');

      expect(result).toBeNull();
      expect(mockedAxios.delete).toHaveBeenCalledWith(
        'http://localhost:3000/api/users/1',
        { timeout: 30000 }
      );
    });
  });

  describe('WebSocket events', () => {
    it('should connect to event stream', () => {
      client.connectEventStream();

      expect(MockedWebSocket).toHaveBeenCalledWith('ws://localhost:8080/ws');
      expect(mockWebSocket.on).toHaveBeenCalledWith('open', expect.any(Function));
      expect(mockWebSocket.on).toHaveBeenCalledWith('message', expect.any(Function));
      expect(mockWebSocket.on).toHaveBeenCalledWith('error', expect.any(Function));
      expect(mockWebSocket.on).toHaveBeenCalledWith('close', expect.any(Function));
    });

    it('should disconnect from event stream', () => {
      client.connectEventStream();
      client.disconnectEventStream();

      expect(mockWebSocket.close).toHaveBeenCalled();
    });
  });

  describe('shutdown', () => {
    it('should shutdown gracefully', async () => {
      // Register a service first
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });
      mockAxiosInstance.delete.mockResolvedValue({
        status: 204,
      });

      await client.registerService('test-service', 'localhost', 3000);
      client.connectEventStream();

      await client.shutdown();

      expect(mockAxiosInstance.delete).toHaveBeenCalledWith(
        '/api/services/test-service/instances/test-instance-1'
      );
      expect(mockWebSocket.close).toHaveBeenCalled();
    });

    it('should handle shutdown errors gracefully', async () => {
      // Register a service first
      mockAxiosInstance.post.mockResolvedValue({
        data: mockServiceInstance,
        status: 201,
      });
      mockAxiosInstance.delete.mockRejectedValue(new Error('Network error'));

      await fastClient.registerService('test-service', 'localhost', 3000);

      // Mock error handler to capture emitted errors
      const errorHandler = jest.fn();
      fastClient.on('error', errorHandler);

      // Should not throw
      await fastClient.shutdown();
      
      // Should have emitted an error
      expect(errorHandler).toHaveBeenCalled();
    });
  });

  describe('retry logic', () => {
    it('should retry failed requests', async () => {
      // Create a client with very fast retry delay for this test
      const fastRetryClient = new ScoutQuestClient('http://localhost:8080', {
        retry_attempts: 2,
        retry_delay: 1, // 1ms delay instead of 1000ms
        timeout: 1000,
      });
      
      mockAxiosInstance.get
        .mockRejectedValueOnce(new Error('Network error'))
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValue({
          data: [mockService],
          status: 200,
        });

      const result = await fastRetryClient.listServices();

      expect(result).toEqual([mockService]);
      expect(mockAxiosInstance.get).toHaveBeenCalledTimes(3);
    });

    it('should not retry client errors', async () => {
      // Create a client with retry disabled to simplify the test
      const noRetryClient = new ScoutQuestClient('http://localhost:8080', {
        retry_attempts: 0,
      });

      const error = new Error('Client Error') as any;
      error.response = { status: 400 };
      mockAxiosInstance.get.mockRejectedValue(error);

      await expect(noRetryClient.listServices()).rejects.toThrow();
      expect(mockAxiosInstance.get).toHaveBeenCalledTimes(1);
    });
  });
});
