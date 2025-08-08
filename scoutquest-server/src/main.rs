use axum::{
    extract::State,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use clap::Parser;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod health_checker;
mod models;
mod registry;

use health_checker::HealthChecker;
use registry::ServiceRegistry;

/// SquoutQuest server configuration
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub health_check: HealthCheckConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String, // json or pretty
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HealthCheckConfig {
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub max_failures: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SecurityConfig {
    pub enable_auth: bool,
    pub api_key: Option<String>,
    pub rate_limit_per_minute: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                enable_cors: true,
                cors_origins: vec!["*".to_string()],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
            },
            health_check: HealthCheckConfig {
                interval_seconds: 30,
                timeout_seconds: 10,
                max_failures: 3,
            },
            security: SecurityConfig {
                enable_auth: false,
                api_key: None,
                rate_limit_per_minute: 1000,
            },
        }
    }
}

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "scoutquest-server")]
#[command(about = "SquoutQuest - Universal Service Discovery for microservices")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Listen port (overrides configuration)
    #[arg(short, long)]
    port: Option<u16>,

    /// Listen address (overrides configuration)
    #[arg(long)]
    host: Option<String>,

    /// Log level (overrides configuration)
    #[arg(long)]
    log_level: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub registry: Arc<ServiceRegistry>,
    pub health_checker: Arc<HealthChecker>,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = load_config(&args)?;

    setup_logging(&config.logging)?;

    tracing::info!(
        "🔍 Starting SquoutQuest Server v{}",
        env!("CARGO_PKG_VERSION")
    );

    let registry = Arc::new(ServiceRegistry::new());
    let health_checker = Arc::new(HealthChecker::new(registry.clone(), &config.health_check));

    health_checker.start_monitoring().await?;

    let app_state = AppState {
        registry,
        health_checker,
        config: config.clone(),
    };

    let cors = if config.server.enable_cors {
        if config.server.cors_origins.contains(&"*".to_string()) {
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any)
        } else {
            let origins: Result<Vec<_>, _> = config
                .server
                .cors_origins
                .iter()
                .map(|origin| origin.parse::<axum::http::HeaderValue>())
                .collect();

            CorsLayer::new()
                .allow_origin(origins?)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ])
        }
    } else {
        CorsLayer::new()
    };

    let app = Router::new()
        .nest("/api/v1", api_routes())
        .route("/health", get(health_endpoint))
        .route("/metrics", get(metrics_endpoint))
        .route("/dashboard", get(dashboard))
        .route("/info", get(info_endpoint))
        .route("/ws", get(websocket_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);

    let host = args.host.as_deref().unwrap_or(&config.server.host);
    let port = args.port.unwrap_or(config.server.port);
    let addr = SocketAddr::from((host.parse::<std::net::IpAddr>()?, port));

    tracing::info!("🚀 SquoutQuest Server started on http://{}", addr);
    tracing::info!("📊 Dashboard available at http://{}/dashboard", addr);
    tracing::info!("🔍 API documentation at http://{}/api/v1", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn load_config(args: &Args) -> anyhow::Result<AppConfig> {
    let mut config_builder =
        Config::builder().add_source(config::Config::try_from(&AppConfig::default())?);

    if std::path::Path::new(&args.config).exists() {
        config_builder =
            config_builder.add_source(File::with_name(&args.config.replace(".toml", "")));
        tracing::info!("📄 Configuration loaded from {}", args.config);
    } else {
        tracing::info!(
            "📄 Configuration file {} not found, using default values",
            args.config
        );
    }

    config_builder = config_builder.add_source(
        Environment::with_prefix("SCOUTQUEST")
            .separator("_")
            .try_parsing(true),
    );

    let mut config: AppConfig = config_builder.build()?.try_deserialize()?;

    if let Some(port) = args.port {
        config.server.port = port;
    }
    if let Some(host) = &args.host {
        config.server.host = host.clone();
    }
    if let Some(log_level) = &args.log_level {
        config.logging.level = log_level.clone();
    }

    Ok(config)
}

fn setup_logging(config: &LoggingConfig) -> anyhow::Result<()> {
    let level = config
        .level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    let registry = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(level.into()));

    match config.format.as_str() {
        "json" => {
            registry
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        "pretty" | _ => {
            registry.with(tracing_subscriber::fmt::layer()).init();
        }
    }

    Ok(())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/services",
            get(api::list_services).post(api::register_service),
        )
        .route(
            "/services/{name}",
            get(api::get_service).delete(api::delete_service),
        )
        .route("/services/{name}/instances", get(api::get_instances))
        .route(
            "/services/{name}/instances/{id}",
            delete(api::deregister_instance),
        )
        .route(
            "/services/{name}/instances/{id}/heartbeat",
            post(api::heartbeat),
        )
        .route(
            "/services/{name}/instances/{id}/status",
            put(api::update_status),
        )
        .route("/discovery/{name}", get(api::discover_service))
        .route(
            "/discovery/{name}/load-balance",
            get(api::load_balance_service),
        )
        .route("/services/{name}/tags", get(api::get_service_tags))
        .route("/tags/{tag}/services", get(api::get_services_by_tag))
        .route("/events", get(api::get_events))
        .route("/services/{name}/watch", get(api::watch_service))
}

async fn health_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    let stats = state.registry.get_stats().await;
    Json(serde_json::json!({
        "status": "UP",
        "services": stats.total_services,
        "instances": stats.total_instances,
        "healthy_instances": stats.healthy_instances,
        "timestamp": chrono::Utc::now()
    }))
}

async fn info_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    let stats = state.registry.get_stats().await;
    Json(serde_json::json!({
        "name": "SquoutQuest Server",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Universal Service Discovery for microservices",
        "uptime_seconds": chrono::Utc::now().timestamp() - stats.start_time,
        "services": stats.total_services,
        "instances": stats.total_instances,
        "healthy_instances": stats.healthy_instances,
        "config": {
            "server": {
                "host": state.config.server.host,
                "port": state.config.server.port,
                "cors_enabled": state.config.server.enable_cors
            },
            "health_check": {
                "interval_seconds": state.config.health_check.interval_seconds,
                "timeout_seconds": state.config.health_check.timeout_seconds,
                "max_failures": state.config.health_check.max_failures
            }
        }
    }))
}

async fn metrics_endpoint(State(state): State<AppState>) -> Json<serde_json::Value> {
    let stats = state.registry.get_stats().await;
    Json(serde_json::json!({
        "registry": {
            "services": stats.total_services,
            "instances": stats.total_instances,
            "healthy": stats.healthy_instances,
            "unhealthy": stats.total_instances - stats.healthy_instances
        },
        "system": {
            "uptime_seconds": chrono::Utc::now().timestamp() - stats.start_time,
            "memory_usage": "TODO",
            "cpu_usage": "TODO"
        }
    }))
}

async fn dashboard() -> axum::response::Html<&'static str> {
    axum::response::Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SquoutQuest Dashboard</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .container { max-width: 1400px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; color: white; margin-bottom: 30px; }
        .header h1 { margin: 0; font-size: 3em; font-weight: 300; }
        .header .subtitle { margin: 10px 0; opacity: 0.9; font-size: 1.2em; }
        .header .logo { font-size: 4em; margin-bottom: 10px; }

        .card {
            background: rgba(255, 255, 255, 0.95);
            padding: 25px;
            margin: 20px 0;
            border-radius: 15px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255,255,255,0.2);
        }

        .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .stat-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 10px;
            text-align: center;
            box-shadow: 0 4px 15px rgba(0,0,0,0.2);
        }
        .stat-number { font-size: 2em; font-weight: bold; margin: 10px 0; }
        .stat-label { font-size: 0.9em; opacity: 0.9; }

        .btn {
            background: #667eea;
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 8px;
            cursor: pointer;
            font-size: 0.9em;
            transition: all 0.3s ease;
        }
        .btn:hover { background: #5a6fd8; transform: translateY(-1px); }

        .service-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr)); gap: 20px; }
        .loading { text-align: center; padding: 40px; color: #666; animation: pulse 2s infinite; }

        @keyframes pulse {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🔍</div>
            <h1>SquoutQuest</h1>
            <p class="subtitle">Service Discovery Dashboard</p>
            <p>Microservices monitoring and management</p>
        </div>

        <div class="card">
            <button class="btn" onclick="loadData()">🔄 Refresh</button>
            <div class="stats-grid" id="statsGrid">
                <div class="loading">Loading...</div>
            </div>
        </div>

        <div class="card">
            <h2>Registered Services</h2>
            <div id="servicesContainer">
                <div class="loading">Loading services...</div>
            </div>
        </div>

        <div class="card">
            <h2>System Metrics</h2>
            <div id="metricsContainer">
                <div class="loading">Loading metrics...</div>
            </div>
        </div>
    </div>

    <script>
        async function loadData() {
            try {
                const healthResponse = await fetch('/health');
                const health = await healthResponse.json();

                document.getElementById('statsGrid').innerHTML = `
                    <div class="stat-card">
                        <div class="stat-number">${health.services}</div>
                        <div class="stat-label">Services</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">${health.instances}</div>
                        <div class="stat-label">Instances</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">${health.healthy_instances}</div>
                        <div class="stat-label">Healthy Instances</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">${((health.healthy_instances / health.instances) * 100 || 0).toFixed(1)}%</div>
                        <div class="stat-label">Health Rate</div>
                    </div>
                `;

                const servicesResponse = await fetch('/api/v1/services');
                const services = await servicesResponse.json();

                if (services.length === 0) {
                    document.getElementById('servicesContainer').innerHTML =
                        '<div style="text-align: center; padding: 40px; color: #666;">No registered services</div>';
                } else {
                    const servicesHtml = services.map(service => `
                        <div class="card">
                            <h3>${service.name}</h3>
                            <p>Instances: ${service.instances.length}</p>
                            <p>Tags: ${service.tags.join(', ')}</p>
                        </div>
                    `).join('');

                    document.getElementById('servicesContainer').innerHTML =
                        `<div class="service-grid">${servicesHtml}</div>`;
                }

                const metricsResponse = await fetch('/metrics');
                const metrics = await metricsResponse.json();

                document.getElementById('metricsContainer').innerHTML = `
                    <div>
                        <h4>Service Registry</h4>
                        <p>Services: ${metrics.registry.services}</p>
                        <p>Instances: ${metrics.registry.instances}</p>
                        <p>Healthy: ${metrics.registry.healthy}</p>
                        <p>Unhealthy: ${metrics.registry.unhealthy}</p>
                    </div>
                `;

            } catch (error) {
                console.error('Error loading data:', error);
            }
        }

        loadData();

        setInterval(loadData, 30000);
    </script>
</body>
</html>
    "#,
    )
}

async fn websocket_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "WebSocket endpoint for real-time updates",
        "status": "coming_soon"
    }))
}
