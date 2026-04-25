use std::net::SocketAddr;
use tokio::net::TcpListener;
use turnkeeper_backend::infrastructure::app_state::AppState;
use turnkeeper_backend::infrastructure::auth::AuthManager;
use turnkeeper_backend::infrastructure::persistence::db::create_pool;
use turnkeeper_backend::infrastructure::persistence::repositories::RepositoryManager;
// use turnkeeper_backend::infrastructure::websocket::game_session_manager::GameSessionManager;
// use turnkeeper_backend::infrastructure::websocket::session_manager::GameSessionManager;
// use turnkeeper_backend::infrastructure::websocket::ws_session_manager::WsSessionManager;
use turnkeeper_backend::build_app;

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

    println!("[STARTUP] Creating AppState...");
    let state = AppState::new(db.clone());
    println!("[STARTUP] AppState created");

    println!("[STARTUP] Building application router...");
    let app = build_app(state);
    println!("[STARTUP] Application router built");

    // Create listener on address
    println!("[STARTUP] Binding TCP listener...");
    // Specify the address to bind to (0.0.0.0 to listen on all interfaces)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to create TCP listener: {}", addr).as_str());
    println!("[STARTUP] TCP listener bound on {}", addr);

    // Start the Axum server
    println!("[STARTUP] Starting server...");
    axum::serve(listener, app)
        .await
        .expect("Failed to launch server");
    println!("[STARTUP] Server running at {}", addr);
}
