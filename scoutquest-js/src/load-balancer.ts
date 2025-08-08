import { ServiceInstance, LoadBalancingStrategy } from './types';
import { ScoutQuestError } from './errors';

/**
 * Load balancer for selecting service instances based on different strategies.
 */
export class LoadBalancer {
  private roundRobinCounters: Map<string, number> = new Map();

  /**
   * Selects a service instance using the specified load balancing strategy.
   * 
   * @param instances - Available service instances
   * @param strategy - Load balancing strategy to use
   * @returns Selected service instance
   * @throws ScoutQuestError if no instances are available
   */
  select(
    instances: ServiceInstance[],
    strategy: LoadBalancingStrategy = LoadBalancingStrategy.RoundRobin
  ): ServiceInstance {
    if (!instances || instances.length === 0) {
      throw new ScoutQuestError(
        'No service instances available',
        'NO_INSTANCES_AVAILABLE'
      );
    }

    // Filter healthy instances if strategy is HealthyOnly or as fallback
    const healthyInstances = instances.filter(instance => 
      instance.status === 'Up'
    );

    // If HealthyOnly strategy and no healthy instances, throw error
    if (strategy === LoadBalancingStrategy.HealthyOnly && healthyInstances.length === 0) {
      throw new ScoutQuestError(
        'No healthy service instances available',
        'NO_HEALTHY_INSTANCES'
      );
    }

    // Use healthy instances if available, otherwise fall back to all instances
    const availableInstances = healthyInstances.length > 0 ? healthyInstances : instances;

    switch (strategy) {
      case LoadBalancingStrategy.RoundRobin:
        return this.selectRoundRobin(availableInstances);
      
      case LoadBalancingStrategy.Random:
      case LoadBalancingStrategy.WeightedRandom:
        return this.selectRandom(availableInstances);
      
      case LoadBalancingStrategy.LeastConnections:
        // For now, fall back to random since we don't track connections
        return this.selectRandom(availableInstances);
      
      case LoadBalancingStrategy.HealthyOnly:
        return this.selectRandom(availableInstances);
      
      default:
        return this.selectRoundRobin(availableInstances);
    }
  }

  /**
   * Selects an instance using round-robin strategy.
   */
  private selectRoundRobin(instances: ServiceInstance[]): ServiceInstance {
    if (instances.length === 1) {
      return instances[0];
    }

    // Create a key based on service name for round-robin counter
    const serviceName = instances[0].service_name;
    const currentCount = this.roundRobinCounters.get(serviceName) || 0;
    const selectedIndex = currentCount % instances.length;
    
    this.roundRobinCounters.set(serviceName, currentCount + 1);
    
    return instances[selectedIndex];
  }

  /**
   * Selects an instance using random strategy.
   */
  private selectRandom(instances: ServiceInstance[]): ServiceInstance {
    const randomIndex = Math.floor(Math.random() * instances.length);
    return instances[randomIndex];
  }

  /**
   * Resets round-robin counters for a specific service.
   * 
   * @param serviceName - Name of the service to reset
   */
  resetRoundRobin(serviceName: string): void {
    this.roundRobinCounters.delete(serviceName);
  }

  /**
   * Resets all round-robin counters.
   */
  resetAllRoundRobin(): void {
    this.roundRobinCounters.clear();
  }

  /**
   * Gets the current round-robin counter for a service.
   * 
   * @param serviceName - Name of the service
   * @returns Current counter value
   */
  getRoundRobinCounter(serviceName: string): number {
    return this.roundRobinCounters.get(serviceName) || 0;
  }
}
