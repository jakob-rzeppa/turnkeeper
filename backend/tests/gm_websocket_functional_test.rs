use reqwest::StatusCode;
use serde_json::Value;
use tokio_tungstenite::tungstenite;

use crate::common::app::spawn_app;
use crate::common::auth::gm_login;
use crate::common::gm_game_connection::create_game;

mod common;

mod gm_websocket_functional_test {
    use super::*;

    mod ws_ticket_endpoint {
        use super::*;

        #[tokio::test]
        async fn ws_ticket_without_auth_returns_unauthorized() {
            let addr = spawn_app().await;

            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "http://{addr}/gm/ws/ticket/00000000-0000-0000-0000-000000000000"
                ))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        async fn ws_ticket_for_existing_game_returns_url() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &token, "Test Game WS").await;

            let client = reqwest::Client::new();
            let resp = client
                .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            let url = body["url"].as_str().unwrap();
            assert!(url.starts_with("ws://"));
            assert!(url.contains(&game_id));
            assert!(url.contains("ticket="));
        }
    }

    mod ws_connection {
        use super::*;

        #[tokio::test]
        async fn ws_connect_with_valid_ticket_succeeds() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &token, "WS Connect Game").await;

            // 1. Obtain a ticket
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
                .header("Authorization", format!("Bearer {token}"))
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
                    // The initial game state should have the expected structure
                    assert!(state["id"].is_string());
                    assert!(state["name"].is_string());
                    assert_eq!(state["name"].as_str().unwrap(), "WS Connect Game");
                }
                other => panic!("Expected text message with game state, got: {other:?}"),
            }
        }

        #[tokio::test]
        async fn ws_connect_without_ticket_fails() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &token, "No Ticket Game").await;

            // Try connecting without a ticket query param
            let ws_url = format!("ws://{addr}/gm/ws/{game_id}");
            let result = tokio_tungstenite::connect_async(&ws_url).await;

            // The server should reject the connection (non-101 response)
            assert!(
                result.is_err(),
                "WebSocket connection without ticket should fail"
            );
        }

        #[tokio::test]
        async fn ws_connect_with_invalid_ticket_closes_connection() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &token, "Bad Ticket Game").await;

            // First request a valid ticket to put the session in Pending state
            let client = reqwest::Client::new();
            let _resp = client
                .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();

            // Try connecting with a bogus ticket
            let ws_url = format!("ws://{addr}/gm/ws/{game_id}?ticket=bogus-ticket");
            let (mut ws_stream, _) = tokio_tungstenite::connect_async(&ws_url)
                .await
                .expect("HTTP upgrade itself may succeed");

            // The server should close the connection quickly since the ticket is invalid.
            // We should receive no valid game state or a close frame.
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
            let token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &token, "Double Ticket Game").await;

            let client = reqwest::Client::new();

            // First ticket request – should succeed
            let resp1 = client
                .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();
            assert_eq!(resp1.status(), StatusCode::OK);

            // Second ticket request while first is still pending – should fail
            let resp2 = client
                .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();
            assert_eq!(resp2.status(), StatusCode::CONFLICT);
        }
    }
}