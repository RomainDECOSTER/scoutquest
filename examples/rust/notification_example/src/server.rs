//! # Notification server
//!
//! Service that automatically registers with ScoutQuest and provides a REST API
//! for notification management.

use crate::types::*;
use anyhow::Result;
use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use scoutquest_rust::ServiceDiscoveryClient;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

/// Shared server state
#[derive(Clone)]
struct AppState {
    notifications: Arc<RwLock<HashMap<Uuid, Notification>>>,
    scoutquest: Arc<ServiceDiscoveryClient>,
    service_name: String,
    port: u16,
}

impl AppState {
    fn new(scoutquest_url: &str, service_name: String, port: u16) -> Result<Self> {
        let scoutquest = Arc::new(ServiceDiscoveryClient::new(scoutquest_url)?);
        let notifications = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            notifications,
            scoutquest,
            service_name,
            port,
        })
    }
}

/// Starts the notification server
pub async fn start_server(
    port: u16,
    scoutquest_url: &str,
    service_name: Option<String>,
) -> Result<()> {
    let service_name = service_name.unwrap_or_else(|| "notification-service".to_string());
    let state = AppState::new(scoutquest_url, service_name.clone(), port)?;

    // Register the service with ScoutQuest
    register_service(&state).await?;

    // Configure routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/notifications", post(create_notification_handler))
        .route("/api/notifications", get(list_notifications_handler))
        .route("/api/notifications/{id}", get(get_notification_handler))
        .route("/api/notifications/{id}/send", post(send_notification_handler))
        .route("/api/notifications/{id}/cancel", post(cancel_notification_handler))
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("🚀 Notification server started on port {}", port);
    println!("📡 Service registered with ScoutQuest: {}", service_name);

    axum::serve(listener, app).await?;
    Ok(())
}

/// Register the service with ScoutQuest
async fn register_service(state: &AppState) -> Result<()> {
    let mut metadata = HashMap::new();
    metadata.insert("type".to_string(), "notification".to_string());
    metadata.insert("version".to_string(), "1.0.0".to_string());

    let tags = vec!["notification".to_string(), "api".to_string()];

    let health_check = scoutquest_rust::models::HealthCheck {
        url: format!("http://localhost:{}/health", state.port),
        interval_seconds: 30,
        timeout_seconds: 5,
        method: "GET".to_string(),
        expected_status: 200,
        headers: None,
    };

    let options = scoutquest_rust::models::ServiceRegistrationOptions::new()
        .with_metadata(metadata)
        .with_tags(tags)
        .with_health_check(health_check);

    state.scoutquest.register_service(
        &state.service_name,
        "localhost", // In production, use external IP
        state.port,
        Some(options)
    ).await?;

    println!("✅ Service successfully registered with ScoutQuest");
    Ok(())
}

/// Handler to check service health
async fn health_handler(state: axum::extract::State<AppState>) -> Json<ServiceHealth> {
    let notifications = state.notifications.read().unwrap();
    let pending_count = notifications
        .values()
        .filter(|n| matches!(n.status, NotificationStatus::Pending))
        .count() as u64;

    Json(ServiceHealth {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        pending_notifications: pending_count,
        processed_today: notifications.len() as u64, // Simplified for the example
    })
}

/// Handler to create a new notification
async fn create_notification_handler(
    state: axum::extract::State<AppState>,
    Json(request): Json<CreateNotificationRequest>,
) -> Result<Json<Notification>, StatusCode> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let notification = Notification {
        id,
        recipient: request.recipient,
        channel: request.channel,
        subject: request.subject,
        content: request.content,
        priority: request.priority.unwrap_or_default(),
        status: NotificationStatus::Pending,
        created_at: now,
        updated_at: now,
        scheduled_at: request.scheduled_at,
        metadata: request.metadata.unwrap_or_default(),
    };

    state.notifications.write().unwrap().insert(id, notification.clone());
    println!("📨 New notification created: {}", id);

    Ok(Json(notification))
}

/// Handler to get a specific notification
async fn get_notification_handler(
    state: axum::extract::State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Notification>, StatusCode> {
    let notifications = state.notifications.read().unwrap();
    match notifications.get(&id) {
        Some(notification) => Ok(Json(notification.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler to get all notifications
async fn list_notifications_handler(
    state: axum::extract::State<AppState>,
) -> Json<Vec<Notification>> {
    let notifications = state.notifications.read().unwrap();
    let mut notifications_list: Vec<Notification> = notifications.values().cloned().collect();
    notifications_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Json(notifications_list)
}

/// Handler to send a notification
async fn send_notification_handler(
    state: axum::extract::State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut notifications = state.notifications.write().unwrap();

    match notifications.get_mut(&id) {
        Some(notification) => {
            if notification.status == NotificationStatus::Pending {
                notification.status = NotificationStatus::Sent;
                notification.updated_at = Utc::now();
                println!("📤 Notification sent: {}", id);

                Ok(Json(ActionResponse {
                    success: true,
                    message: "Notification sent successfully".to_string(),
                }))
            } else {
                Ok(Json(ActionResponse {
                    success: false,
                    message: format!("Notification already in state: {:?}", notification.status),
                }))
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler to cancel a notification
async fn cancel_notification_handler(
    state: axum::extract::State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionResponse>, StatusCode> {
    let mut notifications = state.notifications.write().unwrap();

    match notifications.get_mut(&id) {
        Some(notification) => {
            if matches!(notification.status, NotificationStatus::Pending | NotificationStatus::Sent) {
                notification.status = NotificationStatus::Cancelled;
                notification.updated_at = Utc::now();
                println!("🚫 Notification cancelled: {}", id);

                Ok(Json(ActionResponse {
                    success: true,
                    message: "Notification cancelled successfully".to_string(),
                }))
            } else {
                Ok(Json(ActionResponse {
                    success: false,
                    message: format!("Cannot cancel, current status: {:?}", notification.status),
                }))
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
