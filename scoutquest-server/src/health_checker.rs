use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use reqwest::Client;
use std::time::Duration;

use crate::{registry::ServiceRegistry, models::InstanceStatus, HealthCheckConfig};

pub struct HealthChecker {
    registry: Arc<ServiceRegistry>,
    http_client: Client,
    config: HealthCheckConfig,
}

impl HealthChecker {
    pub fn new(registry: Arc<ServiceRegistry>, config: &HealthCheckConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            registry,
            http_client,
            config: config.clone(),
        }
    }

    pub async fn start_monitoring(&self) -> anyhow::Result<()> {
        let scheduler = JobScheduler::new().await?;

        let registry = self.registry.clone();
        let client = self.http_client.clone();
        let interval = self.config.interval_seconds;

        let health_job = Job::new_async(&format!("0/{} * * * * *", interval), move |_uuid, _l| {
            let registry = registry.clone();
            let client = client.clone();

            Box::pin(async move {
                Self::check_all_instances(registry, client).await;
            })
        })?;

        let registry_cleanup = self.registry.clone();
        let cleanup_job = Job::new_async("0 */5 * * * *", move |_uuid, _l| {
            let registry = registry_cleanup.clone();

            Box::pin(async move {
                Self::cleanup_stale_instances(registry).await;
            })
        })?;

        scheduler.add(health_job).await?;
        scheduler.add(cleanup_job).await?;
        scheduler.start().await?;

        tracing::info!("🏥 Health checker started (interval: {}s)", interval);
        Ok(())
    }

    async fn check_all_instances(registry: Arc<ServiceRegistry>, client: Client) {
        let instances: Vec<_> = registry.get_all_instances();

        for instance in instances {
            if let Some(health_check) = &instance.health_check {
                let is_healthy = Self::check_instance_health(&client, health_check).await;

                let new_status = if is_healthy {
                    InstanceStatus::Up
                } else {
                    InstanceStatus::Down
                };

                if !matches!((instance.status.clone(), &new_status), (InstanceStatus::Up, InstanceStatus::Up) | (InstanceStatus::Down, InstanceStatus::Down)) {
                    registry.update_instance_status(&instance.id, new_status).await;
                }
            }
        }
    }

    async fn cleanup_stale_instances(registry: Arc<ServiceRegistry>) {
        let now = chrono::Utc::now();
        let stale_threshold = chrono::Duration::minutes(5);

        let stale_instances: Vec<String> = registry.get_all_instances().iter()
            .filter(|entry| {
                now.signed_duration_since(entry.last_heartbeat) > stale_threshold
            })
            .map(|entry| entry.id.clone())
            .collect();

        for instance_id in stale_instances {
            tracing::warn!("Removing stale instance: {}", instance_id);
            registry.deregister_instance(&instance_id).await;
        }
    }

    async fn check_instance_health(client: &Client, health_check: &crate::models::HealthCheck) -> bool {
        let mut request = client.request(
            health_check.method.parse().unwrap_or(reqwest::Method::GET),
            &health_check.url
        )
            .timeout(Duration::from_secs(health_check.timeout_seconds));

        if let Some(headers) = &health_check.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        match request.send().await {
            Ok(response) => response.status().as_u16() == health_check.expected_status,
            Err(_) => false,
        }
    }
}