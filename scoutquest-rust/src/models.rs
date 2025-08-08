use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Represents a service instance in the ScoutQuest discovery system.
/// 
/// A service instance contains all the information needed to connect to
/// and identify a specific instance of a service, including its network
/// location, health status, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceInstance {
    /// Unique identifier for this service instance
    pub id: String,
    /// Name of the service this instance belongs to
    pub service_name: String,
    /// Hostname or IP address where the service is running
    pub host: String,
    /// Port number where the service is listening
    pub port: u16,
    /// Whether the service uses HTTPS/TLS
    pub secure: bool,
    /// Current status of the service instance
    pub status: InstanceStatus,
    /// Custom metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Tags associated with this service instance
    pub tags: Vec<String>,
    /// Timestamp when the service was first registered
    pub registered_at: DateTime<Utc>,
    /// Timestamp of the last heartbeat received
    pub last_heartbeat: DateTime<Utc>,
    /// Timestamp when the status last changed
    pub last_status_change: DateTime<Utc>,
}

impl ServiceInstance {
    /// Returns true if the service instance is healthy and ready to serve requests.
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, InstanceStatus::Up)
    }

    /// Constructs the full URL for a given path on this service instance.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The API path to append to the base URL
    /// 
    /// # Returns
    /// 
    /// A complete URL string ready to be used for HTTP requests.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use scoutquest_rust::*;
    /// use std::collections::HashMap;
    /// use chrono::Utc;
    /// 
    /// let instance = ServiceInstance {
    ///     id: "test-1".to_string(),
    ///     service_name: "api".to_string(),
    ///     host: "localhost".to_string(),
    ///     port: 3000,
    ///     secure: false,
    ///     status: InstanceStatus::Up,
    ///     metadata: HashMap::new(),
    ///     tags: Vec::new(),
    ///     registered_at: Utc::now(),
    ///     last_heartbeat: Utc::now(),
    ///     last_status_change: Utc::now(),
    /// };
    /// 
    /// assert_eq!(instance.get_url("/users"), "http://localhost:3000/users");
    /// assert_eq!(instance.get_url("users"), "http://localhost:3000/users");
    /// ```
    pub fn get_url(&self, path: &str) -> String {
        let protocol = if self.secure { "https" } else { "http" };
        let clean_path = if path.starts_with('/') { path } else { &format!("/{}", path) };
        format!("{}://{}:{}{}", protocol, self.host, self.port, clean_path)
    }
}

/// Represents the operational status of a service instance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstanceStatus {
    /// Service is running and ready to accept requests
    Up,
    /// Service is not responding or has failed
    Down,
    /// Service is in the process of starting up
    Starting,
    /// Service is gracefully shutting down
    Stopping,
    /// Service is running but temporarily out of service
    OutOfService,
    /// Service status is unknown or could not be determined
    Unknown,
}

/// Configuration for health check endpoints.
/// 
/// Defines how the ScoutQuest server should check the health of a service instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// URL path for the health check endpoint
    pub url: String,
    /// How often to perform health checks (in seconds)
    pub interval_seconds: u64,
    /// Maximum time to wait for a health check response (in seconds)
    pub timeout_seconds: u64,
    /// HTTP method to use for health checks
    pub method: String,
    /// Expected HTTP status code for a healthy response
    pub expected_status: u16,
    /// Optional HTTP headers to send with health check requests
    pub headers: Option<HashMap<String, String>>,
}

/// Default implementation for HealthCheck
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

/// Optional configuration for service registration.
/// 
/// This struct allows you to specify additional metadata, tags, health checks,
/// and security settings when registering a service.
#[derive(Debug, Clone, Default)]
pub struct ServiceRegistrationOptions {
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
    pub health_check: Option<HealthCheck>,
    pub secure: bool,
}

/// Service registration options.
impl ServiceRegistrationOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set metadata for the service.
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Set tags for the service.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set health check configuration for the service.
    pub fn with_health_check(mut self, health_check: HealthCheck) -> Self {
        self.health_check = Some(health_check);
        self
    }

    /// Set whether the service uses HTTPS/TLS.
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

/// Service discovery options.
impl ServiceDiscoveryOptions {
    pub fn new() -> Self {
        Self {
            healthy_only: true,
            ..Default::default()
        }
    }

    /// Set whether to include only healthy instances in the discovery results.
    pub fn with_healthy_only(mut self, healthy_only: bool) -> Self {
        self.healthy_only = healthy_only;
        self
    }

    /// Set tags for the service.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Set the maximum number of instances to return.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use chrono::Utc;

    fn create_test_instance() -> ServiceInstance {
        ServiceInstance {
            id: "test-123".to_string(),
            service_name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 3000,
            secure: false,
            status: InstanceStatus::Up,
            metadata: HashMap::new(),
            tags: vec!["test".to_string()],
            registered_at: Utc::now(),
            last_heartbeat: Utc::now(),
            last_status_change: Utc::now(),
        }
    }

    #[test]
    fn test_service_instance_is_healthy() {
        let mut instance = create_test_instance();
        
        // Test healthy status
        instance.status = InstanceStatus::Up;
        assert!(instance.is_healthy());

        // Test unhealthy statuses
        instance.status = InstanceStatus::Down;
        assert!(!instance.is_healthy());

        instance.status = InstanceStatus::Starting;
        assert!(!instance.is_healthy());

        instance.status = InstanceStatus::Stopping;
        assert!(!instance.is_healthy());

        instance.status = InstanceStatus::OutOfService;
        assert!(!instance.is_healthy());

        instance.status = InstanceStatus::Unknown;
        assert!(!instance.is_healthy());
    }

    #[test]
    fn test_service_instance_get_url() {
        let instance = create_test_instance();

        assert_eq!(instance.get_url("/api/users"), "http://localhost:3000/api/users");
        assert_eq!(instance.get_url("api/users"), "http://localhost:3000/api/users");
        assert_eq!(instance.get_url("/"), "http://localhost:3000/");
        assert_eq!(instance.get_url(""), "http://localhost:3000/");
    }

    #[test]
    fn test_service_instance_get_url_secure() {
        let mut instance = create_test_instance();
        instance.secure = true;

        assert_eq!(instance.get_url("/api/users"), "https://localhost:3000/api/users");
    }

    #[test]
    fn test_health_check_default() {
        let health_check = HealthCheck::default();
        
        assert_eq!(health_check.url, "");
        assert_eq!(health_check.interval_seconds, 30);
        assert_eq!(health_check.timeout_seconds, 10);
        assert_eq!(health_check.method, "GET");
        assert_eq!(health_check.expected_status, 200);
        assert!(health_check.headers.is_none());
    }

    #[test]
    fn test_service_registration_options_builder() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());

        let options = ServiceRegistrationOptions::new()
            .with_metadata(metadata.clone())
            .with_tags(vec!["api".to_string(), "v1".to_string()])
            .with_secure(true);

        assert_eq!(options.metadata, metadata);
        assert_eq!(options.tags, vec!["api", "v1"]);
        assert!(options.secure);
    }

    #[test]
    fn test_service_discovery_options_builder() {
        let options = ServiceDiscoveryOptions::new()
            .with_healthy_only(false)
            .with_tags(vec!["production".to_string()])
            .with_limit(10);

        assert!(!options.healthy_only);
        assert_eq!(options.tags, Some(vec!["production".to_string()]));
        assert_eq!(options.limit, Some(10));
    }

    #[test]
    fn test_service_discovery_options_default() {
        let options = ServiceDiscoveryOptions::new();
        assert!(options.healthy_only);
        assert!(options.tags.is_none());
        assert!(options.limit.is_none());
    }
}