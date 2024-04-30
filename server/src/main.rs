use std::borrow::Cow;
use std::str::FromStr;
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::{IntoResponse};
use axum::Router;
use tower::{BoxError, ServiceBuilder};
use tracing::Level;
use tower_http::{
    trace::TraceLayer,
    services::ServeDir,
};
use tower_http::add_extension::AddExtensionLayer;

mod types;
mod config;
mod services;
mod app_state;
mod routes;
#[tokio::main]
async fn main() {
    // Load settings
    let settings = match config::Settings::new() {
        Ok(settings) => settings,
        Err(e) => panic!("Error loading settings: {}", e)
    };

    // initialize tracing
    let log_level = match Level::from_str(settings.logger.level.as_str()) {
        Ok(level) => level,
        Err(e) => panic!("Error loading log level: {}", e)
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    let assets_path = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Error loading assets path: {}", e)
    };
// initialize the router
    let app = Router::new().nest("/services", services::services_ui_routes()).nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    ).nest("/api", routes::routes())
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(app_state::State::default()))
                .into_inner(),
        );

    // initialize the listener
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.server.port)).await {
        Ok(listener) => listener,
        Err(e) => panic!("Error binding to port: {}", e)
    };
    tracing::info!("Server running on port {}", settings.server.port);
    axum::serve(listener, app).await.unwrap();

}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
