use crate::services::services_routes;

pub fn routes() -> axum::Router {
    axum::Router::new().nest("/services", services_routes())
}