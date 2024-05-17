use askama_axum::Template;
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

pub fn services_routes() -> axum::Router {
    axum::Router::new().route("/", post(register)).route("/:uuid", put(update_service_status).delete(delete_service))
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