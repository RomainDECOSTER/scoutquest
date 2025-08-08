use crate::models::ServiceInstance;
use crate::error::{ScoutQuestError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Strategies for load balancing across service instances.
/// 
/// Different strategies can be used depending on your requirements:
/// - Random: Good for general purpose load distribution
/// - RoundRobin: Ensures even distribution across all instances
/// - LeastConnections: Chooses instance with fewest active connections (TODO)
/// - WeightedRandom: Allows weighting instances differently (TODO)
/// - HealthyOnly: Only selects from healthy instances, fails if none available
#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    /// Select a random instance from the available pool
    Random,
    /// Cycle through instances in order
    RoundRobin,
    /// Select instance with least active connections (not yet implemented)
    LeastConnections,
    /// Select instance based on weighted random distribution (not yet implemented)
    WeightedRandom,
    /// Only select from healthy instances, error if none available
    HealthyOnly,
}

/// Load balancer for selecting service instances.
/// 
/// The LoadBalancer implements various strategies for distributing requests
/// across multiple instances of a service. It maintains state for round-robin
/// selection and can filter instances based on health status.
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    round_robin_counter: Arc<AtomicUsize>,
}

impl LoadBalancer {
    /// Creates a new LoadBalancer instance.
    pub fn new() -> Self {
        Self {
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Selects a service instance using the specified strategy.
    /// 
    /// # Arguments
    /// 
    /// * `instances` - Slice of available service instances
    /// * `strategy` - The load balancing strategy to use
    /// 
    /// # Returns
    /// 
    /// Returns a selected ServiceInstance or an error if no suitable instance is available.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use scoutquest_rust::*;
    /// use std::collections::HashMap;
    /// use chrono::Utc;
    /// 
    /// let instances = vec![
    ///     ServiceInstance {
    ///         id: "1".to_string(),
    ///         service_name: "test".to_string(),
    ///         host: "host1".to_string(),
    ///         port: 3000,
    ///         secure: false,
    ///         status: InstanceStatus::Up,
    ///         metadata: HashMap::new(),
    ///         tags: Vec::new(),
    ///         registered_at: Utc::now(),
    ///         last_heartbeat: Utc::now(),
    ///         last_status_change: Utc::now(),
    ///     }
    /// ];
    /// 
    /// let balancer = LoadBalancer::new();
    /// let selected = balancer.select_instance(&instances, &LoadBalancingStrategy::Random).unwrap();
    /// assert_eq!(selected.id, "1");
    /// ```
    pub fn select_instance(
        &self,
        instances: &[ServiceInstance],
        strategy: &LoadBalancingStrategy,
    ) -> Result<ServiceInstance> {
        if instances.is_empty() {
            return Err(ScoutQuestError::InternalError(
                "Aucune instance disponible".to_string(),
            ));
        }

        let healthy_instances: Vec<ServiceInstance> = instances
            .iter()
            .filter(|instance| instance.is_healthy())
            .cloned()
            .collect();

        let target_instances = if healthy_instances.is_empty() {
            instances
        } else {
            &healthy_instances
        };

        match strategy {
            LoadBalancingStrategy::Random => {
                let index = fastrand::usize(0..target_instances.len());
                Ok(target_instances[index].clone())
            }
            LoadBalancingStrategy::RoundRobin => {
                let index = self.round_robin_counter.fetch_add(1, Ordering::Relaxed) % target_instances.len();
                Ok(target_instances[index].clone())
            }
            LoadBalancingStrategy::LeastConnections => {
                Ok(target_instances[0].clone())
            }
            LoadBalancingStrategy::WeightedRandom => {
                let index = fastrand::usize(0..target_instances.len());
                Ok(target_instances[index].clone())
            }
            LoadBalancingStrategy::HealthyOnly => {
                if healthy_instances.is_empty() {
                    return Err(ScoutQuestError::NoHealthyInstances {
                        service_name: instances[0].service_name.clone(),
                    });
                }
                Ok(healthy_instances[0].clone())
            }
        }
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ServiceInstance, InstanceStatus};
    use std::collections::HashMap;
    use chrono::Utc;

    fn create_test_instances() -> Vec<ServiceInstance> {
        vec![
            ServiceInstance {
                id: "instance-1".to_string(),
                service_name: "test-service".to_string(),
                host: "host1".to_string(),
                port: 3000,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: vec!["web".to_string()],
                registered_at: Utc::now(),
                last_heartbeat: Utc::now(),
                last_status_change: Utc::now(),
            },
            ServiceInstance {
                id: "instance-2".to_string(),
                service_name: "test-service".to_string(),
                host: "host2".to_string(),
                port: 3001,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: vec!["web".to_string()],
                registered_at: Utc::now(),
                last_heartbeat: Utc::now(),
                last_status_change: Utc::now(),
            },
            ServiceInstance {
                id: "instance-3".to_string(),
                service_name: "test-service".to_string(),
                host: "host3".to_string(),
                port: 3002,
                secure: false,
                status: InstanceStatus::Down,
                metadata: HashMap::new(),
                tags: vec!["web".to_string()],
                registered_at: Utc::now(),
                last_heartbeat: Utc::now(),
                last_status_change: Utc::now(),
            },
        ]
    }

    #[test]
    fn test_load_balancer_new() {
        let balancer = LoadBalancer::new();
        assert_eq!(balancer.round_robin_counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_random_strategy() {
        let balancer = LoadBalancer::new();
        let instances = create_test_instances();

        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::Random);
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        assert!(instances.iter().any(|i| i.id == selected.id));
    }

    #[test]
    fn test_round_robin_strategy() {
        let balancer = LoadBalancer::new();
        let instances = create_test_instances();

        let first = balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin).unwrap();
        let second = balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin).unwrap();
        let third = balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin).unwrap();
        let fourth = balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin).unwrap();

        // Should cycle through healthy instances (first two)
        assert_ne!(first.id, second.id);
        assert_eq!(first.id, third.id); // Should wrap around
        assert_eq!(second.id, fourth.id);
    }

    #[test]
    fn test_healthy_only_strategy() {
        let balancer = LoadBalancer::new();
        let instances = create_test_instances();

        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::HealthyOnly);
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        assert!(selected.is_healthy());
        assert!(selected.id == "instance-1" || selected.id == "instance-2");
    }

    #[test]
    fn test_healthy_only_strategy_no_healthy_instances() {
        let balancer = LoadBalancer::new();
        let mut instances = create_test_instances();
        
        // Make all instances unhealthy
        for instance in &mut instances {
            instance.status = InstanceStatus::Down;
        }

        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::HealthyOnly);
        assert!(result.is_err());
        
        if let Err(ScoutQuestError::NoHealthyInstances { service_name }) = result {
            assert_eq!(service_name, "test-service");
        } else {
            panic!("Expected NoHealthyInstances error");
        }
    }

    #[test]
    fn test_empty_instances() {
        let balancer = LoadBalancer::new();
        let instances = vec![];

        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::Random);
        assert!(result.is_err());
        
        if let Err(ScoutQuestError::InternalError(msg)) = result {
            assert!(msg.contains("Aucune instance disponible"));
        } else {
            panic!("Expected InternalError");
        }
    }

    #[test]
    fn test_least_connections_fallback() {
        let balancer = LoadBalancer::new();
        let instances = create_test_instances();

        // LeastConnections currently falls back to first instance
        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::LeastConnections);
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        assert_eq!(selected.id, "instance-1");
    }

    #[test]
    fn test_weighted_random_fallback() {
        let balancer = LoadBalancer::new();
        let instances = create_test_instances();

        // WeightedRandom currently falls back to random selection
        let result = balancer.select_instance(&instances, &LoadBalancingStrategy::WeightedRandom);
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        assert!(instances.iter().any(|i| i.id == selected.id));
    }
}