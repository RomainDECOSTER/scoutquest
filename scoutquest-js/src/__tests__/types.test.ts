import type {
  ServiceInstance,
  Service,
  HealthCheck,
  RegisterServiceRequest,
  DiscoveryQuery,
  UpdateStatusRequest,
  RegistryStats,
  ServiceEvent,
  ServiceRegistrationOptions,
  ClientConfig,
  DiscoveryResponse,
} from '../types';
import {
  InstanceStatus,
  LoadBalancingStrategy,
  EventType,
} from '../types';

describe('Types', () => {
  describe('InstanceStatus enum', () => {
    it('should have all expected statuses', () => {
      expect(InstanceStatus.Up).toBe('Up');
      expect(InstanceStatus.Down).toBe('Down');
      expect(InstanceStatus.Starting).toBe('Starting');
      expect(InstanceStatus.Stopping).toBe('Stopping');
      expect(InstanceStatus.OutOfService).toBe('OutOfService');
      expect(InstanceStatus.Unknown).toBe('Unknown');
    });
  });

  describe('LoadBalancingStrategy enum', () => {
    it('should have all expected strategies', () => {
      expect(LoadBalancingStrategy.RoundRobin).toBe('RoundRobin');
      expect(LoadBalancingStrategy.Random).toBe('Random');
      expect(LoadBalancingStrategy.LeastConnections).toBe('LeastConnections');
      expect(LoadBalancingStrategy.WeightedRandom).toBe('WeightedRandom');
      expect(LoadBalancingStrategy.HealthyOnly).toBe('HealthyOnly');
    });
  });

  describe('EventType enum', () => {
    it('should have all expected event types', () => {
      expect(EventType.ServiceRegistered).toBe('ServiceRegistered');
      expect(EventType.ServiceDeregistered).toBe('ServiceDeregistered');
      expect(EventType.InstanceStatusChanged).toBe('InstanceStatusChanged');
      expect(EventType.HealthCheckFailed).toBe('HealthCheckFailed');
      expect(EventType.HealthCheckPassed).toBe('HealthCheckPassed');
    });
  });

  describe('Type checking', () => {
    it('should accept valid ServiceInstance', () => {
      const instance: ServiceInstance = {
        id: 'test-1',
        service_name: 'test-service',
        host: 'localhost',
        port: 3000,
        secure: false,
        status: InstanceStatus.Up,
        metadata: { version: '1.0.0' },
        tags: ['api'],
        registered_at: '2023-01-01T00:00:00Z',
        last_heartbeat: '2023-01-01T00:00:00Z',
        last_status_change: '2023-01-01T00:00:00Z',
      };

      expect(instance.id).toBe('test-1');
      expect(instance.service_name).toBe('test-service');
      expect(instance.status).toBe(InstanceStatus.Up);
    });

    it('should accept valid HealthCheck', () => {
      const healthCheck: HealthCheck = {
        url: '/health',
        interval_seconds: 30,
        timeout_seconds: 5,
        method: 'GET',
        expected_status: 200,
        headers: { 'X-Health': 'check' },
      };

      expect(healthCheck.url).toBe('/health');
      expect(healthCheck.method).toBe('GET');
    });

    it('should accept valid Service', () => {
      const service: Service = {
        name: 'test-service',
        instances: [],
        tags: ['api'],
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-01T00:00:00Z',
      };

      expect(service.name).toBe('test-service');
      expect(Array.isArray(service.instances)).toBe(true);
    });

    it('should accept valid RegisterServiceRequest', () => {
      const request: RegisterServiceRequest = {
        service_name: 'test-service',
        host: 'localhost',
        port: 3000,
        secure: false,
        metadata: { version: '1.0.0' },
        tags: ['api'],
      };

      expect(request.service_name).toBe('test-service');
      expect(request.port).toBe(3000);
    });

    it('should accept valid DiscoveryQuery', () => {
      const query: DiscoveryQuery = {
        healthy_only: true,
        tags: 'api,v1',
        limit: 5,
        strategy: LoadBalancingStrategy.RoundRobin,
      };

      expect(query.healthy_only).toBe(true);
      expect(query.strategy).toBe(LoadBalancingStrategy.RoundRobin);
    });

    it('should accept valid ClientConfig', () => {
      const config: ClientConfig = {
        timeout: 60000,
        retry_attempts: 5,
        retry_delay: 2000,
        default_strategy: LoadBalancingStrategy.Random,
        headers: { 'X-Custom': 'value' },
      };

      expect(config.timeout).toBe(60000);
      expect(config.default_strategy).toBe(LoadBalancingStrategy.Random);
    });

    it('should accept valid ServiceRegistrationOptions', () => {
      const options: ServiceRegistrationOptions = {
        secure: true,
        metadata: { version: '2.0.0' },
        tags: ['api', 'v2'],
        enable_heartbeat: true,
        heartbeat_interval: 30000,
        health_check: {
          url: '/health',
          interval_seconds: 30,
          timeout_seconds: 5,
          method: 'GET',
          expected_status: 200,
        },
      };

      expect(options.secure).toBe(true);
      expect(options.enable_heartbeat).toBe(true);
    });

    it('should accept valid RegistryStats', () => {
      const stats: RegistryStats = {
        total_services: 10,
        total_instances: 25,
        healthy_instances: 20,
        start_time: 1640995200,
      };

      expect(stats.total_services).toBe(10);
      expect(stats.healthy_instances).toBe(20);
    });

    it('should accept valid ServiceEvent', () => {
      const event: ServiceEvent = {
        event_type: EventType.ServiceRegistered,
        service_name: 'test-service',
        instance_id: 'test-1',
        timestamp: '2023-01-01T00:00:00Z',
        details: { message: 'Service registered successfully' },
      };

      expect(event.event_type).toBe(EventType.ServiceRegistered);
      expect(event.service_name).toBe('test-service');
    });
  });
});
