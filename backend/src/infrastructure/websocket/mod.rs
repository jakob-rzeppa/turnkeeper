//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game events.

use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::response::Response;
use crate::AppState;

/// Handles WebSocket upgrade requests.
pub async fn websocket_handler(State(state): State<AppState>, Path(id): Path<String>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

/// Handles an established WebSocket connection.
///
/// # Arguments
///
/// * `socket` - The WebSocket connection
async fn handle_socket(mut socket: WebSocket) {
    // Send a greeting message to the client
    if let Err(e) = socket
        .send(Message::Text(Utf8Bytes::from("Hello from the server!")))
        .await
    {
        eprintln!("Error sending message: {}", e);
        return;
    }

    // Loop to keep the connection alive
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(msg) => {
                println!("Received message: {}", msg);
                if let Err(e) = socket.send(Message::Text(Utf8Bytes::from(format!("Echo: {}", msg)))).await {
                    eprintln!("Error sending message: {}", e);
                }
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}