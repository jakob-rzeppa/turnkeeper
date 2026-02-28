//! # Turnkeeper Backend Server
//!
//! Binary entry-point that boots the Axum server.
//! All shared logic lives in the library crate (`lib.rs`).

use std::net::SocketAddr;
use dotenv::dotenv;
use tokio::net::TcpListener;
use turnkeeper_backend::infrastructure::auth::AuthManager;
use turnkeeper_backend::infrastructure::persistence::db::create_pool;
use turnkeeper_backend::infrastructure::persistence::repositories::RepositoryManager;
use turnkeeper_backend::infrastructure::websocket::session_manager::GameSessionManager;
use turnkeeper_backend::{build_app, AppState};

/// Main entry point for the Turnkeeper backend server.
///
/// # Environment Variables
///
/// * `DATABASE_URL` - Path to SQLite database file (required)
/// * `GM_JWT_SECRET` - Gm secret key for JWT signing (required)
/// * `USER_JWT_SECRET` - User secret key for JWT signing (required)
/// * `GM_PASSWORD` - Master password for GM authentication (required)
///
/// # Panics
///
/// The function will panic if:
/// - `.env` file is not found
/// - `DATABASE_URL` is not set
/// - Database pool creation fails
/// - Server binding fails
#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is not set");

    let db = create_pool(&database_url).await.expect("Failed to create database pool");

    // Create Managers
    let repository_manager = RepositoryManager::new(db.clone());
    let auth_manager = AuthManager::new();

    let state = AppState { repository_manager, auth_manager, game_session_manager: GameSessionManager::new() };

    let app = build_app(state);

    // Specify the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create listener on address
    let listener = TcpListener::bind(addr).await.expect("create tcp listener failed");

    // Start the Axum server
    axum::serve(listener, app).await.expect("launch server failed");

    println!("Server running at {}", addr);
}