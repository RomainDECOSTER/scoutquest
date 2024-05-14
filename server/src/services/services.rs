use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Up,
    Down,
    Registered,
}

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Up => write!(f, "Up"),
            ServiceStatus::Down => write!(f, "Down"),
            ServiceStatus::Registered => write!(f, "Registered"),
        }
    }
}

/// Service struct
///
/// Describe a service
///
/// # Example
///
/// ```
/// use services::Service;
///
/// let service = Service::new("service".to_string(), "0.0.0.0".to_string(), "hostname".to_string());
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub ip_addr: String,
    pub hostname: String,
    pub port: u16,
    pub status: ServiceStatus,
}

impl Service {
    pub fn new(name: String, ip_addr: String, hostname: String, port: u16) -> Self {
        Self {
            name,
            ip_addr,
            hostname,
            port,
            status: ServiceStatus::Registered,
            id: Uuid::new_v4(),
        }
    }
}

impl PartialEq for Service{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ip_addr == other.ip_addr && self.hostname == other.hostname && self.port == other.port
    }

}

/// Services struct
///
/// Describe a list of services, it is a wrapper around a `Vec<Service>`.
///
/// # Example
///
/// ```
/// use services::Services;
/// use services::Service;
///
/// let services = Services::new("service".to_string(), "service group".to_string(), vec![Service::new("service".to_string(), "0.0.0.0".to_string(), "hostname".to_string())]);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceGroup {
    pub name: String,
    pub services: Vec<Service>,
}

impl ServiceGroup {
    pub fn new(name: String, services: Vec<Service>) -> Self {
        Self {
            name,
            services,
        }
    }
}


/// ServiceState struct
///
/// Describe the state of services, it is a wrapper around a `Vec<ServiceGroup>`.
///
/// # Example
///
/// ```
/// use services::ServiceState;
/// use services::ServiceGroup;
///
/// let service_state = ServiceState::new(vec![ServiceGroup::new("service".to_string(), "service group".to_string(), vec![Service::new("service".to_string(), "0.0.0.0".to_string(), "hostname".to_string())])]);
/// ```
///
/// # Example
///
/// ```
/// use services::ServiceState;
///
/// let service_state = ServiceState::default();
///
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceState {
    pub service_groups: Vec<ServiceGroup>,
}

impl Default for ServiceState {
    fn default() -> Self {
        Self {
            service_groups: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_new() {
        let service = Service::new("service".to_string(), "0.0.0.0".to_string(), "hostname".to_string(), 8080);
        assert_eq!(service.name, "service");
        assert_eq!(service.ip_addr, "0.0.0.0");
        assert_eq!(service.hostname, "hostname");
        assert_eq!(service.port, 8080);
        assert_eq!(service.status, ServiceStatus::Registered)
    }

    #[test]
    fn test_service_group_new() {
        let service_group = ServiceGroup::new("service".to_string(), vec![Service::new("service".to_string(), "0.0.0.0".to_string(), "hostname".to_string(), 8080)]);
        assert_eq!(service_group.name, "service");
        assert_eq!(service_group.services.len(), 1);
    }

    #[test]
    fn test_service_state_default() {
        let service_state = ServiceState::default();
        assert_eq!(service_state.service_groups.len(), 0);
    }

}