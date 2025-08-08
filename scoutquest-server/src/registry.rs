use chrono::Utc;
use dashmap::DashMap;
use rand::prelude::IndexedRandom;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::models::*;

pub struct ServiceRegistry {
    services: DashMap<String, Service>,
    instances: DashMap<String, ServiceInstance>,
    start_time: AtomicI64,
    round_robin_counters: DashMap<String, AtomicUsize>,
    event_sender: broadcast::Sender<ServiceEvent>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            services: DashMap::new(),
            instances: DashMap::new(),
            start_time: AtomicI64::new(Utc::now().timestamp()),
            round_robin_counters: DashMap::new(),
            event_sender,
        }
    }

    pub async fn register_instance(
        &self,
        request: RegisterServiceRequest,
    ) -> anyhow::Result<ServiceInstance> {
        let instance_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let instance = ServiceInstance {
            id: instance_id.clone(),
            service_name: request.service_name.clone(),
            host: request.host,
            port: request.port,
            secure: request.secure.unwrap_or(false),
            status: InstanceStatus::Up,
            metadata: request.metadata.unwrap_or_default(),
            tags: request.tags.unwrap_or_default(),
            health_check: request.health_check,
            registered_at: now,
            last_heartbeat: now,
            last_status_change: now,
        };

        self.instances.insert(instance_id.clone(), instance.clone());

        let service_existed = self.services.contains_key(&request.service_name);

        self.services
            .entry(request.service_name.clone())
            .and_modify(|service| {
                service.instances.push(instance.clone());
                service.updated_at = now;
            })
            .or_insert_with(|| Service {
                name: request.service_name.clone(),
                instances: vec![instance.clone()],
                tags: instance.tags.clone(),
                created_at: now,
                updated_at: now,
            });

        let event = ServiceEvent {
            event_type: if service_existed {
                EventType::InstanceRegistered
            } else {
                EventType::ServiceRegistered
            },
            service_name: request.service_name.clone(),
            instance_id: Some(instance_id.clone()),
            timestamp: now,
            details: serde_json::json!({
                "host": instance.host,
                "port": instance.port,
                "tags": instance.tags
            }),
        };

        let _ = self.event_sender.send(event);

        tracing::info!(
            "Instance registered: {} for service {}",
            instance_id,
            request.service_name
        );
        Ok(instance)
    }

    pub async fn deregister_instance(&self, instance_id: &str) -> bool {
        if let Some((_, instance)) = self.instances.remove(instance_id) {
            let mut service_removed = false;

            if let Some(mut service) = self.services.get_mut(&instance.service_name) {
                service.instances.retain(|i| i.id != instance_id);
                service.updated_at = Utc::now();

                if service.instances.is_empty() {
                    drop(service);
                    self.services.remove(&instance.service_name);
                    service_removed = true;
                }
            }

            let event = ServiceEvent {
                event_type: if service_removed {
                    EventType::ServiceDeregistered
                } else {
                    EventType::InstanceDeregistered
                },
                service_name: instance.service_name.clone(),
                instance_id: Some(instance_id.to_string()),
                timestamp: Utc::now(),
                details: serde_json::json!({
                    "host": instance.host,
                    "port": instance.port
                }),
            };

            let _ = self.event_sender.send(event);

            tracing::info!("Instance deregistered: {}", instance_id);
            true
        } else {
            false
        }
    }

    pub async fn update_heartbeat(&self, instance_id: &str) -> bool {
        if let Some(mut instance) = self.instances.get_mut(instance_id) {
            let previous_status = instance.status.clone();
            instance.last_heartbeat = Utc::now();

            if !matches!(instance.status, InstanceStatus::Up) {
                instance.status = InstanceStatus::Up;
                instance.last_status_change = Utc::now();

                let event = ServiceEvent {
                    event_type: EventType::HealthCheckRecovered,
                    service_name: instance.service_name.clone(),
                    instance_id: Some(instance_id.to_string()),
                    timestamp: Utc::now(),
                    details: serde_json::json!({
                        "previous_status": format!("{:?}", previous_status),
                        "new_status": "Up"
                    }),
                };

                let _ = self.event_sender.send(event);
            }

            true
        } else {
            false
        }
    }

    pub async fn get_service_instances(
        &self,
        service_name: &str,
        query: &DiscoveryQuery,
    ) -> Vec<ServiceInstance> {
        let mut instances = self
            .services
            .get(service_name)
            .map(|service| service.instances.clone())
            .unwrap_or_default();

        if query.healthy_only.unwrap_or(true) {
            instances.retain(|i| matches!(i.status, InstanceStatus::Up));
        }

        if let Some(required_tags) = &query.tags {
            let tags: Vec<&str> = required_tags.split(',').collect();
            instances.retain(|i| tags.iter().all(|tag| i.tags.contains(&tag.to_string())));
        }

        if let Some(limit) = query.limit {
            instances.truncate(limit);
        }

        instances
    }

    pub async fn load_balance_service(
        &self,
        service_name: &str,
        strategy: LoadBalancingStrategy,
    ) -> Option<ServiceInstance> {
        let query = DiscoveryQuery {
            healthy_only: Some(true),
            tags: None,
            limit: None,
            strategy: Some(strategy.clone()),
        };

        let instances = self.get_service_instances(service_name, &query).await;

        if instances.is_empty() {
            return None;
        }

        match strategy {
            LoadBalancingStrategy::Random => {
                let mut rng = rand::rng();
                instances.choose(&mut rng).cloned()
            }
            LoadBalancingStrategy::RoundRobin => {
                let counter = self
                    .round_robin_counters
                    .entry(service_name.to_string())
                    .or_insert_with(|| AtomicUsize::new(0));

                let index = counter.fetch_add(1, Ordering::Relaxed) % instances.len();
                instances.get(index).cloned()
            }
            LoadBalancingStrategy::LeastConnections => instances.first().cloned(),
            LoadBalancingStrategy::WeightedRandom => {
                let mut rng = rand::rng();
                instances.choose(&mut rng).cloned()
            }
            LoadBalancingStrategy::HealthyOnly => instances.first().cloned(),
        }
    }

    pub async fn get_all_services(&self) -> Vec<Service> {
        self.services
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub async fn get_services_by_tag(&self, tag: &str) -> Vec<Service> {
        self.services
            .iter()
            .filter(|entry| entry.value().tags.contains(&tag.to_string()))
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub async fn update_instance_status(&self, instance_id: &str, status: InstanceStatus) -> bool {
        if let Some(mut instance) = self.instances.get_mut(instance_id) {
            let previous_status = instance.status.clone();
            instance.status = status.clone();
            instance.last_status_change = Utc::now();

            let event = ServiceEvent {
                event_type: EventType::InstanceStatusChanged,
                service_name: instance.service_name.clone(),
                instance_id: Some(instance_id.to_string()),
                timestamp: Utc::now(),
                details: serde_json::json!({
                    "previous_status": format!("{:?}", previous_status),
                    "new_status": format!("{:?}", status)
                }),
            };

            let _ = self.event_sender.send(event);

            tracing::info!("Status updated for instance {}: {:?}", instance_id, status);
            true
        } else {
            false
        }
    }

    pub async fn get_stats(&self) -> RegistryStats {
        let total_services = self.services.len();
        let total_instances = self.instances.len();
        let healthy_instances = self
            .instances
            .iter()
            .filter(|entry| matches!(entry.value().status, InstanceStatus::Up))
            .count();

        RegistryStats {
            total_services,
            total_instances,
            healthy_instances,
            start_time: self.start_time.load(Ordering::Relaxed),
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<ServiceEvent> {
        self.event_sender.subscribe()
    }

    pub fn get_all_instances(&self) -> Vec<ServiceInstance> {
        self.instances
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}
