use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub secure: bool,
    pub status: InstanceStatus,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub health_check: Option<HealthCheck>,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub last_status_change: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceStatus {
    Up,
    Down,
    Starting,
    Stopping,
    OutOfService,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub url: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub method: String,
    pub expected_status: u16,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub instances: Vec<ServiceInstance>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    Random,
    LeastConnections,
    WeightedRandom,
    HealthyOnly,
}

#[derive(Debug, Deserialize)]
pub struct RegisterServiceRequest {
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub secure: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryQuery {
    pub healthy_only: Option<bool>,
    pub tags: Option<String>, // Comma-separated tags
    pub limit: Option<usize>,
    pub strategy: Option<LoadBalancingStrategy>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: InstanceStatus,
}

#[derive(Debug, Serialize)]
pub struct RegistryStats {
    pub total_services: usize,
    pub total_instances: usize,
    pub healthy_instances: usize,
    pub start_time: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceEvent {
    pub event_type: EventType,
    pub service_name: String,
    pub instance_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub enum EventType {
    ServiceRegistered,
    ServiceDeregistered,
    InstanceRegistered,
    InstanceDeregistered,
    InstanceStatusChanged,
    HealthCheckFailed,
    HealthCheckRecovered,
}