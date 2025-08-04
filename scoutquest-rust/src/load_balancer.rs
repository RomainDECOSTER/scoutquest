use crate::models::ServiceInstance;
use crate::error::{ScoutQuestError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    Random,
    RoundRobin,
    LeastConnections,
    WeightedRandom,
    HealthyOnly,
}

#[derive(Debug, Clone)]
pub struct LoadBalancer {
    round_robin_counter: Arc<AtomicUsize>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

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