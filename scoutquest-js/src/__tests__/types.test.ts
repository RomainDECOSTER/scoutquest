import type {
  ServiceInstance,
  Service,
  RegisterServiceRequest,
  DiscoveryQuery,
  ServiceEvent,
  ClientConfig,
} from '../types';
import {
  InstanceStatus,
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

  describe('EventType enum', () => {
    it('should have all expected event types', () => {
      expect(EventType.ServiceRegistered).toBe('ServiceRegistered');
      expect(EventType.ServiceDeregistered).toBe('ServiceDeregistered');
      expect(EventType.InstanceStatusChanged).toBe('InstanceStatusChanged');
      expect(EventType.HealthCheckFailed).toBe('HealthCheckFailed');
      expect(EventType.HealthCheckPassed).toBe('HealthCheckPassed');
    });
  });

  describe('ServiceInstance interface', () => {
    it('should be valid with all required fields', () => {
      const instance: ServiceInstance = {
        id: 'test-instance',
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

      expect(instance.id).toBe('test-instance');
      expect(instance.service_name).toBe('test-service');
      expect(instance.status).toBe(InstanceStatus.Up);
    });
  });

  describe('Service interface', () => {
    it('should be valid with all required fields', () => {
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
  });

  describe('RegisterServiceRequest interface', () => {
    it('should be valid with all required fields', () => {
      const request: RegisterServiceRequest = {
        service_name: 'test-service',
        host: 'localhost',
        port: 3000,
        secure: false,
        metadata: { version: '1.0.0' },
        tags: ['api'],
        health_check: { 
          url: '/health',
          interval_seconds: 30,
          timeout_seconds: 5,
          method: 'GET',
          expected_status: 200
        },
      };

      expect(request.service_name).toBe('test-service');
      expect(request.host).toBe('localhost');
      expect(request.port).toBe(3000);
    });
  });

  describe('DiscoveryQuery interface', () => {
    it('should be valid with empty query', () => {
      const query: DiscoveryQuery = {};
      expect(query).toEqual({});
    });

    it('should be valid with optional fields', () => {
      const query: DiscoveryQuery = {
        healthy_only: true,
        tags: 'api',
      };

      expect(query.healthy_only).toBe(true);
      expect(query.tags).toBe('api');
    });
  });

  describe('ClientConfig interface', () => {
    it('should be valid with empty config', () => {
      const config: ClientConfig = {};
      expect(config).toEqual({});
    });

    it('should be valid with all optional fields', () => {
      const config: ClientConfig = {
        timeout: 30000,
        retry_attempts: 3,
        retry_delay: 1000,
        headers: { 'X-API-Key': 'secret' },
      };

      expect(config.timeout).toBe(30000);
      expect(config.retry_attempts).toBe(3);
      expect(config.retry_delay).toBe(1000);
    });
  });

  describe('ServiceEvent interface', () => {
    it('should be valid event', () => {
      const event: ServiceEvent = {
        event_type: EventType.ServiceRegistered,
        service_name: 'test-service',
        instance_id: 'test-instance',
        timestamp: '2023-01-01T00:00:00Z',
        details: { version: '1.0.0' },
      };

      expect(event.event_type).toBe(EventType.ServiceRegistered);
      expect(event.service_name).toBe('test-service');
      expect(event.instance_id).toBe('test-instance');
    });
  });
});
