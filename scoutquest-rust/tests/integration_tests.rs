#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use scoutquest_rust::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_service_registration() {
        let mock_server = MockServer::start().await;

        let mock_response = serde_json::json!({
            "id": "test-123",
            "service_name": "test-service",
            "host": "localhost",
            "port": 3000,
            "secure": false,
            "status": "Up",
            "metadata": {},
            "tags": [],
            "registered_at": "2024-01-01T00:00:00Z",
            "last_heartbeat": "2024-01-01T00:00:00Z",
            "last_status_change": "2024-01-01T00:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/api/v1/services"))
            .respond_with(ResponseTemplate::new(201).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        let result = client.register_service(
            "test-service",
            "localhost",
            3000,
            Some(ServiceRegistrationOptions::new())
        ).await;

        assert!(result.is_ok());
        let instance = result.unwrap();
        assert_eq!(instance.service_name, "test-service");
        assert_eq!(instance.host, "localhost");
        assert_eq!(instance.port, 3000);
    }

    #[tokio::test]
    async fn test_service_discovery() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/discovery/user-service"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "id": "user-123",
                    "service_name": "user-service",
                    "host": "localhost",
                    "port": 5000,
                    "secure": false,
                    "status": "Up",
                    "metadata": {},
                    "tags": [],
                    "registered_at": "2024-01-01T00:00:00Z",
                    "last_heartbeat": "2024-01-01T00:00:00Z",
                    "last_status_change": "2024-01-01T00:00:00Z"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        let result = client.discover_service("user-service", None).await;

        assert!(result.is_ok());
        let instances = result.unwrap();
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].service_name, "user-service");
    }

    #[tokio::test]
    async fn test_load_balancing() {
        let instances = vec![
            ServiceInstance {
                id: "1".to_string(),
                service_name: "test".to_string(),
                host: "host1".to_string(),
                port: 3000,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            },
            ServiceInstance {
                id: "2".to_string(),
                service_name: "test".to_string(),
                host: "host2".to_string(),
                port: 3001,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            },
        ];

        let load_balancer = LoadBalancer::new();

        // Test Random
        let result = load_balancer.select_instance(&instances, &LoadBalancingStrategy::Random);
        assert!(result.is_ok());

        // Test RoundRobin
        let result1 = load_balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin);
        let result2 = load_balancer.select_instance(&instances, &LoadBalancingStrategy::RoundRobin);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_ne!(result1.unwrap().id, result2.unwrap().id);
    }

    #[tokio::test]
    async fn test_healthy_only_strategy() {
        let instances = vec![
            ServiceInstance {
                id: "1".to_string(),
                service_name: "test".to_string(),
                host: "host1".to_string(),
                port: 3000,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            },
            ServiceInstance {
                id: "2".to_string(),
                service_name: "test".to_string(),
                host: "host2".to_string(),
                port: 3001,
                secure: false,
                status: InstanceStatus::Down,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            },
        ];

        let load_balancer = LoadBalancer::new();

        let result = load_balancer.select_instance(&instances, &LoadBalancingStrategy::HealthyOnly);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, "1");

        let unhealthy_instances = vec![
            ServiceInstance {
                id: "1".to_string(),
                service_name: "test".to_string(),
                host: "host1".to_string(),
                port: 3000,
                secure: false,
                status: InstanceStatus::Down,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            },
        ];

        let result = load_balancer.select_instance(&unhealthy_instances, &LoadBalancingStrategy::HealthyOnly);
        assert!(result.is_err());
    }
}