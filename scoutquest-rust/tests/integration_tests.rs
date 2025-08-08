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

    #[tokio::test]
    async fn test_service_registration_with_metadata() {
        let mock_server = MockServer::start().await;

        let mock_response = serde_json::json!({
            "id": "test-with-metadata",
            "service_name": "test-service",
            "host": "localhost",
            "port": 3000,
            "secure": false,
            "status": "Up",
            "metadata": {
                "version": "1.0.0",
                "environment": "test"
            },
            "tags": ["api", "v1"],
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

        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("environment".to_string(), "test".to_string());

        let options = ServiceRegistrationOptions::new()
            .with_metadata(metadata.clone())
            .with_tags(vec!["api".to_string(), "v1".to_string()]);

        let result = client.register_service("test-service", "localhost", 3000, Some(options)).await;

        assert!(result.is_ok());
        let instance = result.unwrap();
        assert_eq!(instance.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(instance.metadata.get("environment"), Some(&"test".to_string()));
        assert!(instance.tags.contains(&"api".to_string()));
        assert!(instance.tags.contains(&"v1".to_string()));
    }

    #[tokio::test]
    async fn test_service_discovery_with_options() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/discovery/filtered-service"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "id": "healthy-1",
                    "service_name": "filtered-service",
                    "host": "localhost",
                    "port": 5000,
                    "secure": false,
                    "status": "Up",
                    "metadata": {},
                    "tags": ["production", "api"],
                    "registered_at": "2024-01-01T00:00:00Z",
                    "last_heartbeat": "2024-01-01T00:00:00Z",
                    "last_status_change": "2024-01-01T00:00:00Z"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        let options = ServiceDiscoveryOptions::new()
            .with_healthy_only(true)
            .with_tags(vec!["production".to_string()])
            .with_limit(5);

        let result = client.discover_service("filtered-service", Some(options)).await;

        assert!(result.is_ok());
        let instances = result.unwrap();
        assert_eq!(instances.len(), 1);
        assert!(instances[0].tags.contains(&"production".to_string()));
    }

    #[tokio::test]
    async fn test_client_configuration() {
        use std::time::Duration;

        let client = ServiceDiscoveryClient::with_config(
            "http://localhost:8080",
            Duration::from_secs(10),
            2,
            Duration::from_millis(500),
        );

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.get_discovery_url(), "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_error_handling_invalid_url() {
        let result = ServiceDiscoveryClient::new("not-a-valid-url");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_error_handling_registration_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/services"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        let result = client.register_service("test-service", "localhost", 3000, None).await;

        assert!(result.is_err());
        if let Err(ScoutQuestError::RegistrationFailed { status, message }) = result {
            assert_eq!(status, 500);
            assert_eq!(message, "Internal Server Error");
        } else {
            panic!("Expected RegistrationFailed error");
        }
    }

    #[tokio::test]
    async fn test_get_services_by_tag() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/tags/api/services"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "name": "user-service",
                    "instances": [],
                    "tags": ["api", "users"],
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z"
                }
            ])))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        let result = client.get_services_by_tag("api").await;

        assert!(result.is_ok());
        let services = result.unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "user-service");
        assert!(services[0].tags.contains(&"api".to_string()));
    }

    #[tokio::test]
    async fn test_deregistration() {
        let mock_server = MockServer::start().await;

        // Mock registration
        let mock_register_response = serde_json::json!({
            "id": "test-deregister",
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
            .respond_with(ResponseTemplate::new(201).set_body_json(mock_register_response))
            .mount(&mock_server)
            .await;

        // Mock deregistration
        Mock::given(method("DELETE"))
            .and(path("/api/v1/services/test-service/instances/test-deregister"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = ServiceDiscoveryClient::new(&mock_server.uri()).unwrap();

        // Register first
        let _instance = client.register_service("test-service", "localhost", 3000, None).await.unwrap();

        // Then deregister
        let result = client.deregister().await;
        assert!(result.is_ok());

        // Check that no instance is registered
        let registered = client.get_registered_instance().await;
        assert!(registered.is_none());
    }
}