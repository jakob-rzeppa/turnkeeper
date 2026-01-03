mod websocket;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::websocket::websocket_handler;

#[tokio::main]
async fn main() {
    // Initialize the Axum router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(websocket_handler));

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
