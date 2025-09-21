use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

/// ScoutQuest-specific configuration section
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ScoutQuestConfig {
    pub tls: Option<ScoutQuestTlsConfig>,
}

/// TLS configuration for ScoutQuest server
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ScoutQuestTlsConfig {
    /// Enable TLS/HTTPS support
    pub enabled: bool,
    /// Certificate directory (ScoutQuest manages certificates here)
    pub cert_dir: String,
    /// Auto-generate self-signed certificates if none exist
    pub auto_generate: bool,
    /// Verify peer certificates (for client authentication)
    pub verify_peer: bool,
    /// Optional: Custom certificate path (overrides auto-generation)
    pub cert_path: Option<String>,
    /// Optional: Custom private key path (overrides auto-generation)
    pub key_path: Option<String>,
    /// TLS minimum version (1.2, 1.3)
    pub min_version: Option<String>,
    /// TLS maximum version (1.2, 1.3)
    pub max_version: Option<String>,
    /// HTTPS redirect (redirect HTTP to HTTPS)
    pub redirect_http: Option<bool>,
    /// Port for HTTP redirect server
    pub http_port: Option<u16>,
}

impl Default for ScoutQuestTlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_dir: "/etc/certs".to_string(),
            auto_generate: true,
            verify_peer: true,
            cert_path: None,
            key_path: None,
            min_version: Some("1.2".to_string()),
            max_version: Some("1.3".to_string()),
            redirect_http: Some(false),
            http_port: Some(3001),
        }
    }
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
