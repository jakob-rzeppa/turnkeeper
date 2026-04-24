//! # Turnkeeper Backend Server
//!
//! Binary entry-point that boots the Axum server.
//! All shared logic lives in the library crate (`lib.rs`).

use std::net::SocketAddr;
use tokio::net::TcpListener;
use turnkeeper_backend::infrastructure::auth::AuthManager;
use turnkeeper_backend::infrastructure::persistence::db::create_pool;
use turnkeeper_backend::infrastructure::persistence::repositories::RepositoryManager;
// use turnkeeper_backend::infrastructure::websocket::game_session_manager::GameSessionManager;
// use turnkeeper_backend::infrastructure::websocket::session_manager::GameSessionManager;
// use turnkeeper_backend::infrastructure::websocket::ws_session_manager::WsSessionManager;
use turnkeeper_backend::{AppState, build_app};

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
/// - `.env` file is not found (in dev mode)
/// - `DATABASE_URL` is not set
/// - Database pool creation fails
/// - Server binding fails
#[tokio::main]
async fn main() {
    println!("[STARTUP] Application starting...");

    dotenv::dotenv().expect("Failed to load .env file");
    println!("[STARTUP] .env file loaded");

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://var/db/turnkeeper.db".to_string());

    println!("[STARTUP] Connecting to database: {}", database_url);
    let db = create_pool(&database_url)
        .await
        .expect(format!("Failed to create database pool: {}", database_url).as_str());
    println!("[STARTUP] Database connected");

    // Create Managers
    println!("[STARTUP] Initializing managers...");
    let repository_manager = RepositoryManager::new(db.clone());
    let auth_manager = AuthManager::new();
    // let ws_session_manager = WsSessionManager::new();
    // let game_session_manager = GameSessionManager::new(repository_manager.game());

    let state = AppState {
        repository_manager,
        auth_manager,
        // //game_session_manager: GameSessionManager::new(),
        // ws_session_manager,
        // game_session_manager,
    };

    let app = build_app(state);

    // Specify the address to bind to (0.0.0.0 to listen on all interfaces)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // Create listener on address
    println!("[STARTUP] Binding to address: {}", addr);
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to create TCP listener: {}", addr).as_str());

    // Start the Axum server
    println!("[STARTUP] Server running at {}", addr);
    axum::serve(listener, app)
        .await
        .expect("Failed to launch server");
}
