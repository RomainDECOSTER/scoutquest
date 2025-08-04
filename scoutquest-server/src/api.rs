use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};

use crate::{AppState, models::*};

pub async fn list_services(State(state): State<AppState>) -> Json<Vec<Service>> {
    let services = state.registry.get_all_services().await;
    Json(services)
}

pub async fn register_service(
    State(state): State<AppState>,
    Json(request): Json<RegisterServiceRequest>,
) -> Result<(StatusCode, Json<ServiceInstance>), StatusCode> {
    match state.registry.register_instance(request).await {
        Ok(instance) => Ok((StatusCode::CREATED, Json(instance))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_service(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Service>, StatusCode> {
    let services = state.registry.get_all_services().await;

    match services.into_iter().find(|s| s.name == name) {
        Some(service) => Ok(Json(service)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_service(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> StatusCode {

    let instances: Vec<_> = state.registry.get_all_instances().iter()
        .filter(|entry| entry.service_name == name)
        .map(|entry| entry.id.clone())
        .collect();

    for instance_id in instances {
        state.registry.deregister_instance(&instance_id).await;
    }

    StatusCode::NO_CONTENT
}

pub async fn get_instances(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<DiscoveryQuery>,
) -> Json<Vec<ServiceInstance>> {
    let instances = state.registry.get_service_instances(&name, &query).await;
    Json(instances)
}

pub async fn discover_service(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<DiscoveryQuery>,
) -> Json<Vec<ServiceInstance>> {
    let instances = state.registry.get_service_instances(&name, &query).await;
    Json(instances)
}

pub async fn load_balance_service(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<DiscoveryQuery>,
) -> Result<Json<ServiceInstance>, StatusCode> {
    let strategy = query.strategy.unwrap_or(LoadBalancingStrategy::Random);

    match state.registry.load_balance_service(&name, strategy).await {
        Some(instance) => Ok(Json(instance)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn deregister_instance(
    State(state): State<AppState>,
    Path((_, id)): Path<(String, String)>,
) -> StatusCode {
    if state.registry.deregister_instance(&id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn heartbeat(
    State(state): State<AppState>,
    Path((_, id)): Path<(String, String)>,
) -> StatusCode {
    if state.registry.update_heartbeat(&id).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn update_status(
    State(state): State<AppState>,
    Path((_, id)): Path<(String, String)>,
    Json(request): Json<UpdateStatusRequest>,
) -> StatusCode {
    if state.registry.update_instance_status(&id, request.status).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_service_tags(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let services = state.registry.get_all_services().await;

    match services.into_iter().find(|s| s.name == name) {
        Some(service) => Ok(Json(service.tags)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_services_by_tag(
    State(state): State<AppState>,
    Path(tag): Path<String>,
) -> Json<Vec<Service>> {
    let services = state.registry.get_services_by_tag(&tag).await;
    Json(services)
}

pub async fn get_events(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Real-time events available via WebSocket at /ws"
    }))
}

pub async fn watch_service(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": format!("Service {} monitoring available via WebSocket", name),
        "websocket_url": "/ws"
    }))
}