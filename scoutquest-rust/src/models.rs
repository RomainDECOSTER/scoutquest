use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceInstance {
    pub id: String,
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub secure: bool,
    pub status: InstanceStatus,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub last_status_change: DateTime<Utc>,
}

impl ServiceInstance {
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, InstanceStatus::Up)
    }

    pub fn get_url(&self, path: &str) -> String {
        let protocol = if self.secure { "https" } else { "http" };
        let clean_path = if path.starts_with('/') { path } else { &format!("/{}", path) };
        format!("{}://{}:{}{}", protocol, self.host, self.port, clean_path)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            url: String::new(),
            interval_seconds: 30,
            timeout_seconds: 10,
            method: "GET".to_string(),
            expected_status: 200,
            headers: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServiceRegistrationOptions {
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub health_check: Option<HealthCheck>,
    pub secure: bool,
}

impl ServiceRegistrationOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_health_check(mut self, health_check: HealthCheck) -> Self {
        self.health_check = Some(health_check);
        self
    }

    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServiceDiscoveryOptions {
    pub healthy_only: bool,
    pub tags: Option<Vec<String>>,
    pub limit: Option<usize>,
}

impl ServiceDiscoveryOptions {
    pub fn new() -> Self {
        Self {
            healthy_only: true,
            ..Default::default()
        }
    }

    pub fn with_healthy_only(mut self, healthy_only: bool) -> Self {
        self.healthy_only = healthy_only;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct RegisterServiceRequest {
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub secure: bool,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub instances: Vec<ServiceInstance>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}