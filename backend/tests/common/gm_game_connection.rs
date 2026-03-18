use std::net::SocketAddr;
use axum::http::StatusCode;
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[allow(dead_code)]
pub async fn create_game(addr: &SocketAddr, token: &str, name: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/gm/games"))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = resp.json().await.unwrap();
    body["id"].as_str().unwrap().to_string()
}

#[allow(dead_code)]
pub async fn gm_ws_ticket_request(addr: &SocketAddr, token: &str, game_id: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/gm/ws/ticket/{game_id}"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .expect("gm ws ticket request failed")
}

#[allow(dead_code)]
pub async fn gm_ws_connect(url: &str) -> Result< WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
    let (stream, _) = tokio_tungstenite::connect_async(url)
        .await.map_err(|e| format!("WebSocket handshake failed: {e}"))?;
    Ok(stream)
}