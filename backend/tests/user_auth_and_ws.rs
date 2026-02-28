//! Functional tests for User authentication and WebSocket connection flow.
//!
//! These tests spin up a real Axum server on a random port, make HTTP requests
//! for User register/login, and test the full WebSocket ticket + upgrade handshake.

use reqwest::StatusCode;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite;

use turnkeeper_backend::infrastructure::auth::AuthManager;
use turnkeeper_backend::infrastructure::persistence::db::create_pool;
use turnkeeper_backend::infrastructure::persistence::repositories::RepositoryManager;
use turnkeeper_backend::infrastructure::websocket::session_manager::GameSessionManager;
use turnkeeper_backend::{build_app, AppState};

/// Boots the full Axum application on a random OS-assigned port and returns its address.
async fn spawn_app() -> SocketAddr {
    unsafe {
        std::env::set_var("GM_PASSWORD", "test-password");
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

/// Helper: register a new user and return the response.
async fn user_register(addr: &SocketAddr, name: &str, password: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/user/register"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name, "password": password }))
        .send()
        .await
        .expect("Failed to send register request")
}

/// Helper: login as a user and return the response.
async fn user_login(addr: &SocketAddr, name: &str, password: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/user/login"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name, "password": password }))
        .send()
        .await
        .expect("Failed to send login request")
}

/// Helper: register a user and return the JWT token.
async fn get_user_token(addr: &SocketAddr, name: &str) -> String {
    let resp = user_register(addr, name, "test-pass").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = resp.json().await.unwrap();
    body["token"].as_str().unwrap().to_string()
}

/// Helper: GM login and return a token.
async fn get_gm_token(addr: &SocketAddr) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/gm/login"))
        .header("Content-Type", "application/json")
        .json(&json!({ "password": "test-password" }))
        .send()
        .await
        .expect("Failed to send GM login request");
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = resp.json().await.unwrap();
    body["token"].as_str().unwrap().to_string()
}

/// Helper: create a game (as GM) and return its id.
async fn create_game(addr: &SocketAddr, gm_token: &str, name: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/gm/games"))
        .header("Authorization", format!("Bearer {gm_token}"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = resp.json().await.unwrap();
    body["id"].as_str().unwrap().to_string()
}

// ─────────────────────────────────────────────────────────────
//  User Registration Tests
// ─────────────────────────────────────────────────────────────

mod user_registration {
    use super::*;

    #[tokio::test]
    async fn register_with_valid_credentials_returns_token() {
        let addr = spawn_app().await;

        let resp = user_register(&addr, "alice", "test-pass").await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = resp.json().await.unwrap();
        assert!(body["token"].is_string());
        assert!(!body["token"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn register_with_empty_name_returns_bad_request() {
        let addr = spawn_app().await;

        let resp = user_register(&addr, "", "test-pass").await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body: Value = resp.json().await.unwrap();
        assert!(body["error"].is_string());
    }

    #[tokio::test]
    async fn register_with_short_password_returns_bad_request() {
        let addr = spawn_app().await;

        let resp = user_register(&addr, "alice", "ab").await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body: Value = resp.json().await.unwrap();
        assert!(body["error"].is_string());
    }

    #[tokio::test]
    async fn register_duplicate_user_returns_conflict() {
        let addr = spawn_app().await;

        let resp1 = user_register(&addr, "alice", "test-pass").await;
        assert_eq!(resp1.status(), StatusCode::OK);

        let resp2 = user_register(&addr, "alice", "test-pass").await;
        assert_eq!(resp2.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn register_without_body_returns_bad_request() {
        let addr = spawn_app().await;

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{addr}/user/register"))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

// ─────────────────────────────────────────────────────────────
//  User Login Tests
// ─────────────────────────────────────────────────────────────

mod user_login_tests {
    use super::*;

    #[tokio::test]
    async fn login_with_correct_credentials_returns_token() {
        let addr = spawn_app().await;

        // Register first
        let reg_resp = user_register(&addr, "bob", "test-pass").await;
        assert_eq!(reg_resp.status(), StatusCode::OK);

        // Now login
        let resp = user_login(&addr, "bob", "test-pass").await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = resp.json().await.unwrap();
        assert!(body["token"].is_string());
        assert!(!body["token"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    async fn login_with_wrong_password_returns_unauthorized() {
        let addr = spawn_app().await;

        let reg_resp = user_register(&addr, "bob", "test-pass").await;
        assert_eq!(reg_resp.status(), StatusCode::OK);

        let resp = user_login(&addr, "bob", "wrong-password").await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body: Value = resp.json().await.unwrap();
        assert!(body["error"].is_string());
    }

    #[tokio::test]
    async fn login_with_nonexistent_user_returns_not_found() {
        let addr = spawn_app().await;

        let resp = user_login(&addr, "nobody", "test-pass").await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: Value = resp.json().await.unwrap();
        assert!(body["error"].is_string());
    }

    #[tokio::test]
    async fn login_without_body_returns_bad_request() {
        let addr = spawn_app().await;

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{addr}/user/login"))
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

// ─────────────────────────────────────────────────────────────
//  Protected Routes – User Authorization
// ─────────────────────────────────────────────────────────────

mod protected_routes {
    use super::*;

    #[tokio::test]
    async fn protected_route_without_token_returns_unauthorized() {
        let addr = spawn_app().await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://{addr}/user/games"))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn protected_route_with_invalid_token_returns_unauthorized() {
        let addr = spawn_app().await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://{addr}/user/games"))
            .header("Authorization", "Bearer invalid-token")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn protected_route_with_valid_token_returns_ok() {
        let addr = spawn_app().await;
        let token = get_user_token(&addr, "charlie").await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://{addr}/user/games"))
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = resp.json().await.unwrap();
        assert!(body["games"].is_array());
    }

    #[tokio::test]
    async fn gm_token_cannot_access_user_routes() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;

        let client = reqwest::Client::new();
        let resp = client
            .get(format!("http://{addr}/user/games"))
            .header("Authorization", format!("Bearer {gm_token}"))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

// ─────────────────────────────────────────────────────────────
//  WebSocket Ticket Endpoint
// ─────────────────────────────────────────────────────────────

mod ws_ticket_endpoint {
    use super::*;

    #[tokio::test]
    async fn ws_ticket_without_auth_returns_unauthorized() {
        let addr = spawn_app().await;

        let client = reqwest::Client::new();
        let resp = client
            .post(format!(
                "http://{addr}/user/ws/ticket/00000000-0000-0000-0000-000000000000"
            ))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn ws_ticket_for_existing_game_returns_url() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "Test Game User WS").await;
        let user_token = get_user_token(&addr, "dave").await;

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token}"))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = resp.json().await.unwrap();
        let url = body["url"].as_str().unwrap();
        assert!(url.starts_with("ws://"));
        assert!(url.contains(&game_id));
        assert!(url.contains("ticket="));
        assert!(url.contains("user_id="));
    }
}

// ─────────────────────────────────────────────────────────────
//  WebSocket Connection
// ─────────────────────────────────────────────────────────────

mod ws_connection {
    use super::*;

    #[tokio::test]
    async fn ws_connect_with_valid_ticket_succeeds() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "User WS Connect Game").await;
        let user_token = get_user_token(&addr, "eve").await;

        // 1. Obtain a ticket
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = resp.json().await.unwrap();
        let ws_url = body["url"].as_str().unwrap().to_string();

        // 2. Open the WebSocket using the ticket URL
        let (mut ws_stream, _) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .expect("WebSocket handshake should succeed");

        // 3. The server should immediately send the initial game state
        use futures_util::StreamExt;
        let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws_stream.next())
            .await
            .expect("Timed out waiting for initial game state")
            .expect("Stream ended unexpectedly")
            .expect("Failed to read message");

        match msg {
            tungstenite::Message::Text(text) => {
                let state: Value = serde_json::from_str(&text).expect("Invalid JSON game state");
                assert!(state["id"].is_string());
                assert!(state["name"].is_string());
                assert_eq!(state["name"].as_str().unwrap(), "User WS Connect Game");
            }
            other => panic!("Expected text message with game state, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn ws_connect_without_ticket_fails() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "No Ticket User Game").await;

        // Try connecting without a ticket query param
        let ws_url = format!("ws://{addr}/user/ws/{game_id}");
        let result = tokio_tungstenite::connect_async(&ws_url).await;

        assert!(
            result.is_err(),
            "WebSocket connection without ticket should fail"
        );
    }

    #[tokio::test]
    async fn ws_connect_with_invalid_ticket_closes_connection() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "Bad Ticket User Game").await;
        let user_token = get_user_token(&addr, "frank").await;

        // First request a valid ticket to put the session in Pending state
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Extract the user_id from the ticket URL query string
        let body: Value = resp.json().await.unwrap();
        let ws_url_str = body["url"].as_str().unwrap();
        let query = ws_url_str.split('?').nth(1).unwrap();
        let user_id = query
            .split('&')
            .find_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                if k == "user_id" { Some(v.to_string()) } else { None }
            })
            .unwrap();

        // Try connecting with a bogus ticket but the correct user_id
        let ws_url = format!("ws://{addr}/user/ws/{game_id}?ticket=bogus-ticket&user_id={user_id}");
        let (mut ws_stream, _) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .expect("HTTP upgrade itself may succeed");

        // The server should close the connection quickly since the ticket is invalid.
        use futures_util::StreamExt;
        let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws_stream.next())
            .await
            .expect("Timed out");

        match msg {
            Some(Ok(tungstenite::Message::Close(_))) | None => {
                // Expected: server closed connection
            }
            Some(Err(_)) => {
                // Connection error is also acceptable
            }
            other => panic!("Expected close or error after invalid ticket, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn ws_ticket_twice_without_connecting_returns_conflict() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "Double Ticket User Game").await;
        let user_token = get_user_token(&addr, "grace").await;

        let client = reqwest::Client::new();

        // First ticket request – should succeed
        let resp1 = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp1.status(), StatusCode::OK);

        // Second ticket request while first is still pending – should fail
        let resp2 = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp2.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn two_different_users_can_get_tickets_for_same_game() {
        let addr = spawn_app().await;
        let gm_token = get_gm_token(&addr).await;
        let game_id = create_game(&addr, &gm_token, "Multi User Game").await;
        let user_token_1 = get_user_token(&addr, "henry").await;
        let user_token_2 = get_user_token(&addr, "irene").await;

        let client = reqwest::Client::new();

        // User 1 gets a ticket
        let resp1 = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token_1}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp1.status(), StatusCode::OK);
        let body1: Value = resp1.json().await.unwrap();
        let url1 = body1["url"].as_str().unwrap();
        assert!(url1.starts_with("ws://"));
        assert!(url1.contains(&game_id));

        // User 2 gets a ticket for the same game – should also succeed
        let resp2 = client
            .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
            .header("Authorization", format!("Bearer {user_token_2}"))
            .send()
            .await
            .unwrap();
        assert_eq!(resp2.status(), StatusCode::OK);
        let body2: Value = resp2.json().await.unwrap();
        let url2 = body2["url"].as_str().unwrap();
        assert!(url2.starts_with("ws://"));
        assert!(url2.contains(&game_id));

        // The two URLs should have different tickets and user_ids
        assert_ne!(url1, url2);
    }
}
