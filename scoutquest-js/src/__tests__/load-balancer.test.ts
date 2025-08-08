import { LoadBalancer } from '../load-balancer';
import type { ServiceInstance } from '../types';
import { LoadBalancingStrategy, InstanceStatus } from '../types';
import { ScoutQuestError } from '../errors';

describe('LoadBalancer', () => {
  let loadBalancer: LoadBalancer;
  let mockInstances: ServiceInstance[];

  beforeEach(() => {
    loadBalancer = new LoadBalancer();
    mockInstances = [
      {
        id: 'instance-1',
        service_name: 'test-service',
        host: 'localhost',
        port: 3001,
        secure: false,
        status: InstanceStatus.Up,
        metadata: {},
        tags: [],
        registered_at: '2023-01-01T00:00:00Z',
        last_heartbeat: '2023-01-01T00:00:00Z',
        last_status_change: '2023-01-01T00:00:00Z',
      },
      {
        id: 'instance-2',
        service_name: 'test-service',
        host: 'localhost',
        port: 3002,
        secure: false,
        status: InstanceStatus.Up,
        metadata: {},
        tags: [],
        registered_at: '2023-01-01T00:00:00Z',
        last_heartbeat: '2023-01-01T00:00:00Z',
        last_status_change: '2023-01-01T00:00:00Z',
      },
      {
        id: 'instance-3',
        service_name: 'test-service',
        host: 'localhost',
        port: 3003,
        secure: false,
        status: InstanceStatus.Down,
        metadata: {},
        tags: [],
        registered_at: '2023-01-01T00:00:00Z',
        last_heartbeat: '2023-01-01T00:00:00Z',
        last_status_change: '2023-01-01T00:00:00Z',
      },
    ];
  });

  describe('select', () => {
    it('should throw error when no instances available', () => {
      expect(() => {
        loadBalancer.select([], LoadBalancingStrategy.RoundRobin);
      }).toThrow(ScoutQuestError);
    });

    it('should select using round robin strategy', () => {
      const instance1 = loadBalancer.select(
        mockInstances,
        LoadBalancingStrategy.RoundRobin
      );
      const instance2 = loadBalancer.select(
        mockInstances,
        LoadBalancingStrategy.RoundRobin
      );
      const instance3 = loadBalancer.select(
        mockInstances,
        LoadBalancingStrategy.RoundRobin
      );

      expect(instance1.id).toBe('instance-1');
      expect(instance2.id).toBe('instance-2');
      expect(instance3.id).toBe('instance-1'); // Back to first (skipping down instance)
    });

    it('should select using random strategy', () => {
      const instance = loadBalancer.select(
        mockInstances,
        LoadBalancingStrategy.Random
      );
      expect(['instance-1', 'instance-2'].includes(instance.id)).toBe(true);
    });

    it('should only select healthy instances with HealthyOnly strategy', () => {
      const instance = loadBalancer.select(
        mockInstances,
        LoadBalancingStrategy.HealthyOnly
      );
      expect(['instance-1', 'instance-2'].includes(instance.id)).toBe(true);
      expect(instance.status).toBe(InstanceStatus.Up);
    });

    it('should throw error when no healthy instances with HealthyOnly strategy', () => {
      const downInstances = mockInstances.map((i) => ({
        ...i,
        status: InstanceStatus.Down,
      }));
      expect(() => {
        loadBalancer.select(downInstances, LoadBalancingStrategy.HealthyOnly);
      }).toThrow(ScoutQuestError);
    });
  });

  describe('resetRoundRobin', () => {
    it('should reset round robin counter for specific service', () => {
      loadBalancer.select(mockInstances, LoadBalancingStrategy.RoundRobin);
      loadBalancer.select(mockInstances, LoadBalancingStrategy.RoundRobin);

      expect(loadBalancer.getRoundRobinCounter('test-service')).toBe(2);

      loadBalancer.resetRoundRobin('test-service');
      expect(loadBalancer.getRoundRobinCounter('test-service')).toBe(0);
    });

    it('should reset all round robin counters', () => {
      loadBalancer.select(mockInstances, LoadBalancingStrategy.RoundRobin);
      loadBalancer.resetAllRoundRobin();
      expect(loadBalancer.getRoundRobinCounter('test-service')).toBe(0);
    });
  });
});
