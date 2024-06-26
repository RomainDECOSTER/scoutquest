use askama_axum::{IntoResponse, Template};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::routing::{get, post, put};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app_state::State;
use crate::services::services::{Service, ServiceGroup, ServiceStatus};
use crate::types::OkResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RouteService {
    name: String,
    ip_addr: String,
    hostname: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RouteQuery {
    status: ServiceStatus
}

#[derive(Serialize)]
struct ServiceResponse {
    uuid: Uuid
}

impl ServiceResponse {
    pub fn new(uuid: Uuid) -> Self{
        Self {
            uuid: uuid.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServiceSearchUrl{
    name: String,
}

#[derive(Serialize)]
struct ServiceUrlResponse {
    url: String
}

impl ServiceUrlResponse {
    pub fn new(serivce: Service) -> Self{
        Self {
            url: format!("http://{}:{}", serivce.ip_addr, serivce.port),
        }
    }
}

#[derive(Template)]
#[template(path = "services.html")]
struct ServicesTemplate {
    services: Vec<ServiceGroup>,
}

// Register a service
async fn register(Extension(state): Extension<State>, json_body: Json<RouteService>) ->Json<ServiceResponse> {
    let mut state = match state.write() {
        Ok(state) => state,
        Err(e) => panic!("Error getting state: {}", e)        
    };
    let service = Service::new(json_body.name.clone().replace(" ", "_").to_uppercase(), json_body.ip_addr.clone(), json_body.hostname.clone(), json_body.port.clone());
    match state.services_state.service_groups.iter().position(|x| x.name == service.name.clone()) {
        Some(index) => match state.services_state.service_groups[index].services.iter().position(|x| x.clone() == service) {
            Some(i) => Json(ServiceResponse::new(state.services_state.service_groups[index].services[i].id)),
            None => {
                state.services_state.service_groups[index].services.push(service.clone());
                Json(ServiceResponse::new(service.id))
            }
        },
        None => {
            let service_group = ServiceGroup::new(service.name.clone(), vec![service.clone()]);
            state.services_state.service_groups.push(service_group);
            Json(ServiceResponse::new(service.id))
        }
    }
}

async fn update_service_status(Extension(state): Extension<State>, Path(uuid): Path<Uuid>, query: Query<RouteQuery>) -> Json<OkResponse>{
    let mut app_state = match state.write() {
        Ok(state) => state,
        Err(e) => panic!("Error getting state: {}", e)
    };
    for service_group in app_state.services_state.service_groups.iter_mut(){
        for service in service_group.services.iter_mut(){
            if service.id == uuid{
                service.status = query.status.clone();
            }
        }
    }
    Json(OkResponse::new())
}

async fn delete_service(Extension(state): Extension<State>, Path(uuid): Path<Uuid>) -> Json<OkResponse>{
    let mut app_state = match state.write() {
        Ok(state) => state,
        Err(e) => panic!("Error getting state: {}", e)
    };
    for service_group in app_state.services_state.service_groups.iter_mut(){
        service_group.services.retain(|x| x.id != uuid);
    }
    Json(OkResponse::new())
}

async fn get_service_by_uuid(Extension(state): Extension<State>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let app_state = match state.read() {
        Ok(state) => state,
        Err(e) => panic!("Error getting state: {}", e)
    };
    for service_group in app_state.services_state.service_groups.iter(){
        for service in service_group.services.iter(){
            if service.id == uuid{
                return (StatusCode::OK, Json(ServiceResponse::new(service.id))).into_response();
            }
        }
    }
    (StatusCode::NOT_FOUND, Json(OkResponse::new())).into_response()
}

async fn get_service_url(Extension(state): Extension<State>, json_body: Json<ServiceSearchUrl>) -> impl IntoResponse {
    let app_state = match state.read() {
        Ok(state) => state,
        Err(e) => panic!("Error getting state: {}", e)
    };
    match app_state.services_state.service_groups.iter().position(|x| x.name == json_body.name.clone()) {
        Some(index) => {
            for service in app_state.services_state.service_groups[index].services.iter(){
                if service.status == ServiceStatus::Up{
                    return (StatusCode::OK, Json(ServiceUrlResponse::new(service.clone()))).into_response();
                }
            }
        },
        None => {
            return (StatusCode::NOT_FOUND, Json(OkResponse::new())).into_response();
        }
    }
    (StatusCode::NOT_FOUND, Json(OkResponse::new())).into_response()
}

pub fn services_routes() -> axum::Router {
    axum::Router::new().route("/", post(register)).route("/:uuid", put(update_service_status).delete(delete_service).get(get_service_by_uuid)).route("/url", post(get_service_url))
}

async fn services_ui(Extension(state): Extension<State>) -> ServicesTemplate {
    ServicesTemplate {
        services: match state.read() {
            Ok(state) => state.services_state.service_groups.clone(),
            Err(e) => panic!("Error getting state: {}", e)
        },
    }
}

pub fn services_ui_routes() -> axum::Router {
    axum::Router::new().route("/", get(services_ui))
}