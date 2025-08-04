use crate::models::*;
use crate::error::{ScoutQuestError, Result};
use crate::load_balancer::{LoadBalancer, LoadBalancingStrategy};
use reqwest::{Client as HttpClient, Method};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{interval, sleep};
use tracing::{info, warn, error, debug};
use url::Url;

#[derive(Clone)]
pub struct ServiceDiscoveryClient {
    discovery_url: String,
    http_client: HttpClient,
    registered_instance: Arc<RwLock<Option<ServiceInstance>>>,
    heartbeat_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    load_balancer: LoadBalancer,
    retry_attempts: usize,
    retry_delay: Duration,
}

impl ServiceDiscoveryClient {
    pub fn new(discovery_url: &str) -> Result<Self> {
        Self::with_config(discovery_url, Duration::from_secs(30), 3, Duration::from_secs(1))
    }

    pub fn with_config(
        discovery_url: &str,
        timeout: Duration,
        retry_attempts: usize,
        retry_delay: Duration,
    ) -> Result<Self> {
        let discovery_url = discovery_url.trim_end_matches('/').to_string();

        Url::parse(&discovery_url)?;

        let http_client = HttpClient::builder()
            .timeout(timeout)
            .build()
            .map_err(ScoutQuestError::NetworkError)?;

        Ok(Self {
            discovery_url,
            http_client,
            registered_instance: Arc::new(RwLock::new(None)),
            heartbeat_handle: Arc::new(Mutex::new(None)),
            load_balancer: LoadBalancer::new(),
            retry_attempts,
            retry_delay,
        })
    }

    pub async fn register_service(
        &self,
        service_name: &str,
        host: &str,
        port: u16,
        options: Option<ServiceRegistrationOptions>,
    ) -> Result<ServiceInstance> {
        let options = options.unwrap_or_default();

        let request = RegisterServiceRequest {
            service_name: service_name.to_string(),
            host: host.to_string(),
            port,
            secure: options.secure,
            metadata: options.metadata,
            tags: options.tags,
            health_check: options.health_check,
        };

        let url = format!("{}/api/v1/services", self.discovery_url);

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let instance: ServiceInstance = response.json().await?;

            {
                let mut registered = self.registered_instance.write().await;
                *registered = Some(instance.clone());
            }

            self.start_heartbeat().await;

            info!("Service {} registered with ID: {}", service_name, instance.id);
            Ok(instance)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(ScoutQuestError::RegistrationFailed { status, message })
        }
    }

    pub async fn discover_service(
        &self,
        service_name: &str,
        options: Option<ServiceDiscoveryOptions>,
    ) -> Result<Vec<ServiceInstance>> {
        let options = options.unwrap_or_default();

        let mut url = Url::parse(&format!("{}/api/v1/discovery/{}", self.discovery_url, service_name))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("healthy_only", &options.healthy_only.to_string());

            if let Some(tags) = &options.tags {
                query_pairs.append_pair("tags", &tags.join(","));
            }

            if let Some(limit) = options.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
        }

        let response = self.http_client.get(url).send().await?;

        if response.status().is_success() {
            let instances: Vec<ServiceInstance> = response.json().await?;
            debug!("Discovered {} instances for service {}", instances.len(), service_name);
            Ok(instances)
        } else if response.status().as_u16() == 404 {
            Ok(Vec::new())
        } else {
            warn!("Discovery failed for {}: {}", service_name, response.status());
            Ok(Vec::new())
        }
    }

    pub async fn load_balance_service(
        &self,
        service_name: &str,
        strategy: LoadBalancingStrategy,
    ) -> Result<ServiceInstance> {
        let instances = self.discover_service(service_name, None).await?;

        if instances.is_empty() {
            return Err(ScoutQuestError::ServiceNotFound {
                service_name: service_name.to_string(),
            });
        }

        let selected = self.load_balancer.select_instance(&instances, &strategy)?;
        Ok(selected)
    }

    pub async fn get_services_by_tag(&self, tag: &str) -> Result<Vec<Service>> {
        let url = format!("{}/api/v1/tags/{}/services", self.discovery_url, tag);

        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let services: Vec<Service> = response.json().await?;
            Ok(services)
        } else {
            warn!("Tag search failed for {}: {}", tag, response.status());
            Ok(Vec::new())
        }
    }

    pub async fn call_service<T>(
        &self,
        service_name: &str,
        path: &str,
        method: Method,
        body: Option<Value>,
        strategy: LoadBalancingStrategy,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        for attempt in 1..=self.retry_attempts {
            match self.try_call_service(service_name, path, &method, &body, &strategy).await {
                Ok(response) => {
                    info!("Successful call to {}:{} (attempt {})", service_name, path, attempt);
                    return Ok(response);
                }
                Err(e) => {
                    warn!("Attempt {}/{} failed for {}:{}: {}",
                          attempt, self.retry_attempts, service_name, path, e);

                    if attempt == self.retry_attempts {
                        error!("Final failure calling {}:{} after {} attempts",
                               service_name, path, self.retry_attempts);
                        return Err(e);
                    }

                    sleep(self.retry_delay * attempt as u32).await;
                }
            }
        }

        unreachable!()
    }

    async fn try_call_service<T>(
        &self,
        service_name: &str,
        path: &str,
        method: &Method,
        body: &Option<Value>,
        strategy: &LoadBalancingStrategy,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let instance = self.load_balance_service(service_name, strategy.clone()).await?;
        let url = instance.get_url(path);

        let mut request_builder = self.http_client.request(method.clone(), &url);

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let response = request_builder.send().await?;

        if response.status().is_success() {
            let result: T = response.json().await?;
            Ok(result)
        } else {
            Err(ScoutQuestError::InternalError(format!(
                "HTTP error {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )))
        }
    }

    pub async fn get<T>(&self, service_name: &str, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_service(service_name, path, Method::GET, None, LoadBalancingStrategy::Random)
            .await
    }

    pub async fn post<T>(&self, service_name: &str, path: &str, body: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_service(service_name, path, Method::POST, Some(body), LoadBalancingStrategy::Random)
            .await
    }

    pub async fn put<T>(&self, service_name: &str, path: &str, body: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_service(service_name, path, Method::PUT, Some(body), LoadBalancingStrategy::Random)
            .await
    }

    pub async fn delete(&self, service_name: &str, path: &str) -> Result<()> {
        let _: Value = self.call_service(service_name, path, Method::DELETE, None, LoadBalancingStrategy::Random)
            .await?;
        Ok(())
    }

    pub async fn deregister(&self) -> Result<()> {
        let instance = {
            let registered = self.registered_instance.read().await;
            registered.clone()
        };

        if let Some(instance) = instance {
            self.stop_heartbeat().await;

            let url = format!(
                "{}/api/v1/services/{}/instances/{}",
                self.discovery_url, instance.service_name, instance.id
            );

            let response = self.http_client.delete(&url).send().await?;

            if response.status().is_success() {
                info!("Service {} deregistered", instance.service_name);
            } else {
                warn!("Deregistration failed: {}", response.status());
            }

            {
                let mut registered = self.registered_instance.write().await;
                *registered = None;
            }
        }

        Ok(())
    }

    async fn start_heartbeat(&self) {
        self.stop_heartbeat().await;

        let discovery_url = self.discovery_url.clone();
        let http_client = self.http_client.clone();
        let registered_instance = self.registered_instance.clone();

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                let instance = {
                    let registered = registered_instance.read().await;
                    registered.clone()
                };

                if let Some(instance) = instance {
                    let url = format!(
                        "{}/api/v1/services/{}/instances/{}/heartbeat",
                        discovery_url, instance.service_name, instance.id
                    );

                    match http_client.post(&url).send().await {
                        Ok(response) => {
                            if !response.status().is_success() {
                                warn!("Heartbeat failed: {}", response.status());
                            }
                        }
                        Err(e) => {
                            error!("Error during heartbeat: {}", e);
                        }
                    }
                } else {
                    break; // No registered instance, stop heartbeat
                }
            }
        });

        {
            let mut heartbeat_handle = self.heartbeat_handle.lock().await;
            *heartbeat_handle = Some(handle);
        }
    }

    async fn stop_heartbeat(&self) {
        let mut heartbeat_handle = self.heartbeat_handle.lock().await;
        if let Some(handle) = heartbeat_handle.take() {
            handle.abort();
        }
    }

    pub async fn get_registered_instance(&self) -> Option<ServiceInstance> {
        let registered = self.registered_instance.read().await;
        registered.clone()
    }

    pub fn get_discovery_url(&self) -> &str {
        &self.discovery_url
    }
}

impl Drop for ServiceDiscoveryClient {
    fn drop(&mut self) {
        if Arc::strong_count(&self.registered_instance) > 1 {
            warn!("ServiceDiscoveryClient dropped without calling deregister(). Call deregister() before dropping.");
        }
    }
}