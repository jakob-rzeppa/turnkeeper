mod websocket;
mod handler;
mod error;
mod util;
mod db;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::routing::post;
use dotenv::dotenv;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use crate::db::create_pool;
use crate::handler::get_routes;
use crate::websocket::websocket_handler;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is not set");

    let db = create_pool(&database_url).await.unwrap();

    let state = AppState { db };

    // Initialize the Axum router
    let app = Router::new()
        .merge(get_routes())
        .route("/ws", get(websocket_handler))
        .with_state(state);

    // Specify the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create listener on address
    let listener = TcpListener::bind(addr).await.expect("create tcp listener failed");

    // Start the Axum server
    axum::serve(listener, app).await.expect("launch server failed");

    println!("Server running at {}", addr);
}

async fn root_handler() -> &'static str {
    "Hello, world!"
}
