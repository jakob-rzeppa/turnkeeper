mod macros;
mod util;
mod domain;
mod application;
mod infrastructure;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use crate::infrastructure::websocket::websocket_handler;
use tower_http::cors::{Any, CorsLayer};
use crate::application::game::event_handlers::GameEventHandler;
use crate::infrastructure::auth::AuthManager;
use crate::infrastructure::http::get_routes;
use crate::infrastructure::persistence::db::create_pool;
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::persistence::repositories::RepositoryManager;

#[derive(Clone)]
pub struct AppState {
    pub repository_manager: RepositoryManager,
    pub auth_manager: AuthManager,
    pub games: Arc<Mutex<Vec<Mutex<GameEventHandler<SqliteGameRepository>>>>>,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is not set");

    let db = create_pool(&database_url).await.expect("Failed to create database pool");

    // Create Managers
    let repository_manager = RepositoryManager::new(db.clone());
    let auth_manager = AuthManager::new();
    
    let state = AppState { repository_manager, auth_manager, games: Arc::new(Mutex::new(Vec::new())) };

    // ONLY FOR DEVELOPMENT - change later
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Initialize the Axum router
    let app = Router::new()
        .merge(get_routes())
        .route("/gm/ws/{id}", get(websocket_handler))
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