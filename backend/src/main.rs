mod error;
mod entity;
mod macros;
mod util;
mod domain;
mod application;
mod infrastructure;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use dotenv::dotenv;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use crate::infrastructure::db::create_pool;
use crate::infrastructure::websocket::websocket_handler;
use tower_http::cors::{Any, CorsLayer};
use crate::infrastructure::handler::get_routes;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is not set");

    let db = create_pool(&database_url).await.expect("Failed to create database pool");

    let state = AppState { db };

    // ONLY FOR DEVELOPMENT - change later
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Initialize the Axum router
    let app = Router::new()
        .merge(get_routes())
        .route("/ws", get(websocket_handler))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors_layer));

    // Specify the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create listener on address
    let listener = TcpListener::bind(addr).await.expect("create tcp listener failed");

    // Start the Axum server
    axum::serve(listener, app).await.expect("launch server failed");

    println!("Server running at {}", addr);
}