//! # Turnkeeper Backend Server
//!
//! A turn-based game management system that follows clean architecture principles.
//!
//! ## Architecture
//!
//! The application is organized into three main layers:
//! - **Domain Layer**: Pure business logic with entities, value objects, and domain events
//! - **Application Layer**: Use cases implemented via Request and Event Handlers
//! - **Infrastructure Layer**: External concerns (HTTP, WebSockets, Database, Auth)

mod macros;
mod util;
mod domain;
mod application;
mod infrastructure;

use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use axum::routing::post;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tokio::sync::{RwLock};
use tower::ServiceBuilder;
use crate::infrastructure::websocket::{websocket_handler, ws_ticket};
use tower_http::cors::{Any, CorsLayer};
use crate::application::game::session::GameSession;
use crate::infrastructure::auth::AuthManager;
use crate::infrastructure::auth::middleware::gm_auth_middleware;
use crate::infrastructure::http::get_routes;
use crate::infrastructure::persistence::db::create_pool;
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::persistence::repositories::RepositoryManager;
use crate::infrastructure::websocket::gm_connection::WebSocketGmConnection;

/// Application state shared across all HTTP handlers and WebSocket connections.
///
/// This struct is cloned for each request/connection using [`Clone`], which is cheap
/// because all fields use `Arc` internally for shared ownership.
/// 
/// # Managers
/// 
/// The Managers contain the services wrapped in an Arc.
/// When handling a request / event the services are passed to the application layer - also in an Arc.
/// This way the service objects stay in memory since they are referenced in
/// the managers and won't get dropped / recreated each time a request is handled.
///
/// # Fields
///
/// * `repository_manager` - Provides access to data repositories (User, Game)
/// * `auth_manager` - Handles JWT generation and validation for GMs and Users
#[derive(Clone)]
pub struct AppState {
    /// Manager providing access to all data repositories
    pub repository_manager: RepositoryManager,
    /// Manager for JWT authentication and validation
    pub auth_manager: AuthManager,
    pub game_session: Arc<RwLock<Option<GameSession<WebSocketGmConnection, SqliteGameRepository>>>>,
}

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

    let state = AppState { repository_manager, auth_manager, game_session: Arc::new(RwLock::new(None)) };

    // ONLY FOR DEVELOPMENT - change later
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Initialize the Axum router
    let app = Router::new()
        .merge(get_routes(state.clone()))
        .route("/gm/ws/{id}", get(websocket_handler))
        .route("/gm/ws/ticket/{game_id}", post(ws_ticket).route_layer(middleware::from_fn_with_state(state.clone(), gm_auth_middleware)))
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