use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn user_ws_ticket_request(addr: &SocketAddr, token: &str, game_id: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/user/ws/ticket/{game_id}"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .expect("user ws ticket request failed")
}

pub async fn user_ws_connect(url: &str) -> Result< WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
    let (stream, _) = tokio_tungstenite::connect_async(url)
        .await.map_err(|e| format!("WebSocket handshake failed: {e}"))?;
    Ok(stream)
}