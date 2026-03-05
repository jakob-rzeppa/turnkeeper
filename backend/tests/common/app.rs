use std::net::SocketAddr;
use tokio::net::TcpListener;
use turnkeeper_backend::{build_app, AppState};
use turnkeeper_backend::infrastructure::auth::AuthManager;
use turnkeeper_backend::infrastructure::persistence::db::create_pool;
use turnkeeper_backend::infrastructure::persistence::repositories::RepositoryManager;
use turnkeeper_backend::infrastructure::websocket::session_manager::GameSessionManager;

pub const TEST_GM_PASSWORD: &str = "test-password";

/// Boots the full Axum application on a random OS-assigned port and returns its address.
pub async fn spawn_app() -> SocketAddr {
    // Set environment variables required by the application.
    // These mirror the defaults used in unit tests (cfg!(test) branches).
    // SAFETY: We set these before any LazyLock is initialized, and all tests
    // use identical values, so there is no data race.
    unsafe {
        std::env::set_var("GM_PASSWORD", TEST_GM_PASSWORD);
        std::env::set_var("GM_JWT_SECRET", "gm test secret");
        std::env::set_var("USER_JWT_SECRET", "user test secret");
    }

    let pool = create_pool("sqlite::memory:").await.expect("Failed to create test pool");
    let state = AppState {
        repository_manager: RepositoryManager::new(pool),
        auth_manager: AuthManager::new(),
        game_session_manager: GameSessionManager::new(),
    };

    let app = build_app(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}