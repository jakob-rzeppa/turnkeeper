pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod util;

use crate::infrastructure::{app_state::AppState, http::get_routes};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

/// Builds the Axum application router with all routes configured.
pub fn build_app(state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(get_routes(state.clone()))
        //.merge(get_websocket_routes(state.clone()))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors_layer))
}
