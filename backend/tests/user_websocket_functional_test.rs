use reqwest::StatusCode;
use serde_json::Value;
use tokio_tungstenite::tungstenite;

use crate::common::app::spawn_app;
use crate::common::auth::{gm_login, user_register};
use crate::common::gm_game_connection::create_game;

mod common;

mod user_websocket_functional_test {
    use super::*;

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
            let gm_token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &gm_token, "Test Game User WS").await;
            let user_token = user_register(&addr, "dave", "test").await.unwrap().token;

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

    mod ws_connection {
        use super::*;

        #[tokio::test]
        async fn ws_connect_with_valid_ticket_succeeds() {
            let addr = spawn_app().await;
            let gm_token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &gm_token, "User WS Connect Game").await;
            let user_token = user_register(&addr, "eve", "test").await.unwrap().token;

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
            let gm_token = gm_login(&addr).await.unwrap().token;
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
            let gm_token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &gm_token, "Bad Ticket User Game").await;
            let user_token = user_register(&addr, "frank", "test").await.unwrap().token;

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
            let gm_token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &gm_token, "Double Ticket User Game").await;
            let user_token = user_register(&addr, "grace", "test").await.unwrap().token;

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
            let gm_token = gm_login(&addr).await.unwrap().token;
            let game_id = create_game(&addr, &gm_token, "Multi User Game").await;
            let user_token_1 = user_register(&addr, "henry", "test").await.unwrap().token;
            let user_token_2 = user_register(&addr, "irene", "test").await.unwrap().token;

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
}