pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod util;

use crate::infrastructure::{app_state::AppState, http::get_routes, logger::logging_middleware, websocket::get_websocket_routes};
use axum::{Router, http::HeaderValue, middleware};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use axum::http::{Method, header::{AUTHORIZATION}};

/// Builds the Axum application router with all routes configured.
pub fn build_app(state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin(std::env::var("CORS_ORIGIN").expect("CORS_ORIGIN environment variable not set.").parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([AUTHORIZATION]);

    Router::new()
        .merge(get_routes(state.clone()))
        .merge(get_websocket_routes(state.clone()))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors_layer))
        .layer(middleware::from_fn(logging_middleware))
}
