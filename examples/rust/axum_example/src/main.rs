use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use scoutquest_rust::{
    ServiceDiscoveryClient, ServiceRegistrationOptions, HealthCheck
};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{info, error};
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    discovery_client: ServiceDiscoveryClient,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    description: String,
    completed: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .unwrap_or(4000);
    let discovery_url = std::env::var("DISCOVERY_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let discovery_client = ServiceDiscoveryClient::new(&discovery_url)?;

    let health_check = HealthCheck {
        url: format!("http://{}:{}/health", host, port),
        interval_seconds: 30,
        timeout_seconds: 10,
        method: "GET".to_string(),
        expected_status: 200,
        headers: None,
    };

    let mut metadata = HashMap::new();
    metadata.insert("language".to_string(), "rust".to_string());
    metadata.insert("framework".to_string(), "axum".to_string());
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("environment".to_string(),
                    std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()));
    metadata.insert("description".to_string(), "Task management service".to_string());

    let registration_options = ServiceRegistrationOptions::new()
        .with_metadata(metadata)
        .with_tags(vec![
            "api".to_string(),
            "tasks".to_string(),
            "microservice".to_string(),
            "productivity".to_string(),
            "backend".to_string(),
        ])
        .with_health_check(health_check)
        .with_secure(false);

    discovery_client
        .register_service("task-service", &host, port, Some(registration_options))
        .await?;

    let app_state = AppState { discovery_client };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/tasks", get(get_tasks).post(create_task))
        .route("/api/tasks/{id}", get(get_task))
        .route("/api/call-user-service", get(call_user_service))
        .route("/api/call-product-service", get(call_product_service))
        .route("/api/microservices-info", get(get_microservices_info))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🚀 Task service started on http://{}:{}", host, port);
    info!("📡 Connected to the service discovery: {}", discovery_url);
    info!("🔍 Dashboard available: {}/dashboard", discovery_url);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install a signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, deregistering...");
}

async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "UP",
        "timestamp": chrono::Utc::now(),
        "service": "task-service"
    }))
}

async fn get_tasks() -> Json<ApiResponse<Vec<Task>>> {
    let tasks = vec![
        Task {
            id: 1,
            title: "Implement the REST API".to_string(),
            description: "Create the endpoints for the task management".to_string(),
            completed: true,
            created_at: chrono::Utc::now() - chrono::Duration::days(2),
        },
        Task {
            id: 2,
            title: "Integrate Service Discovery".to_string(),
            description: "Configure the automatic registration of the service in the service discovery".to_string(),
            completed: true,
            created_at: chrono::Utc::now() - chrono::Duration::days(1),
        },
        Task {
            id: 3,
            title: "Unit tests".to_string(),
            description: "Write tests for all endpoints".to_string(),
            completed: false,
            created_at: chrono::Utc::now(),
        },
    ];

    Json(ApiResponse::success(tasks))
}

async fn create_task(Json(payload): Json<Value>) -> Json<ApiResponse<Task>> {
    let new_task = Task {
        id: chrono::Utc::now().timestamp() as u64 % 1000,
        title: payload["title"].as_str().unwrap_or("New task").to_string(),
        description: payload["description"].as_str().unwrap_or("").to_string(),
        completed: false,
        created_at: chrono::Utc::now(),
    };

    info!("New task created: {}", new_task.id);
    Json(ApiResponse::success(new_task))
}

async fn get_task(Path(id): Path<u64>) -> Json<ApiResponse<Option<Task>>> {
    if id <= 3 {
        let task = Task {
            id,
            title: format!("Task {}", id),
            description: format!("Description of the task {}", id),
            completed: id == 1,
            created_at: chrono::Utc::now() - chrono::Duration::days(id as i64),
        };
        Json(ApiResponse::success(Some(task)))
    } else {
        Json(ApiResponse::success(None))
    }
}

async fn call_user_service(State(state): State<AppState>) -> Json<ApiResponse<Value>> {
    match state.discovery_client.get::<Value>("user-service", "/api/users").await {
        Ok(users) => {
            let response = json!({
                "message": "Users retrieved from the user service",
                "users": users,
                "strategy": "random"
            });
            Json(ApiResponse::success(response))
        }
        Err(e) => {
            error!("Error calling the user service: {}", e);
            Json(ApiResponse::error(format!("User service not available: {}", e)))
        }
    }
}

async fn call_product_service(State(state): State<AppState>) -> Json<ApiResponse<Value>> {
    match state.discovery_client
        .call_service::<Value>(
            "product-service",
            "/api/products",
            reqwest::Method::GET,
            None,
        )
        .await
    {
        Ok(products) => {
            let response = json!({
                "message": "Products retrieved from the product service",
                "products": products,
            });
            Json(ApiResponse::success(response))
        }
        Err(e) => {
            error!("Error calling the product service: {}", e);
            Json(ApiResponse::error(format!("Product service not available: {}", e)))
        }
    }
}

async fn get_microservices_info(State(state): State<AppState>) -> Json<ApiResponse<Value>> {
    match state.discovery_client.get_services_by_tag("microservice").await {
        Ok(services) => {
            let mut service_stats = Vec::new();

            for service in &services {
                match state.discovery_client.discover_service(&service.name, None).await {
                    Ok(instance) => {
                        let is_healthy = instance.is_healthy();

                        service_stats.push(json!({
                            "name": service.name,
                            "instance": {
                                "id": instance.id,
                                "host": instance.host,
                                "port": instance.port,
                                "status": format!("{:?}", instance.status)
                            },
                            "healthy": is_healthy,
                            "tags": service.tags
                        }));
                    }
                    Err(e) => {
                        error!("Error discovering the service {}: {}", service.name, e);
                    }
                }
            }

            let total_services = services.len();
            let healthy_services = service_stats.iter()
                .filter(|s| s["healthy"].as_bool().unwrap_or(false))
                .count();

            let response = json!({
                "microservices_count": total_services,
                "healthy_services": healthy_services,
                "services": service_stats
            });

            Json(ApiResponse::success(response))
        }
        Err(e) => {
            error!("Error retrieving microservices: {}", e);
            Json(ApiResponse::error(format!("Error retrieving microservices: {}", e)))
        }
    }
}
