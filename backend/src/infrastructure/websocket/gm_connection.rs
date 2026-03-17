//! # WebSocket Connection
//!
//! Implements [`ConnectionContract`] over an Axum WebSocket, splitting the
//! socket into independent send and receive halves protected by `Mutex`.

use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use crate::application::game::contracts::{ConnectionContract};
use crate::application::game::dto::{IncomingConnectionMessageDto, OutgoingConnectionMessageDto};
use crate::domain::game::commands::GameCommand;

/// A WebSocket connection implementing [`ConnectionContract`].
///
/// Used for both GM and user connections.
pub struct WebSocketConnection {
    sender: Mutex<SplitSink<WebSocket, Message>>,
    receiver: Mutex<SplitStream<WebSocket>>,
}

impl WebSocketConnection {
    /// Creates a new connection by splitting the given WebSocket.
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver)
        }
    }
}

impl ConnectionContract for WebSocketConnection {
    async fn recv(&self) -> IncomingConnectionMessageDto {
        let mut receiver = self.receiver.lock().await;

        match receiver.next().await {
            Some(Ok(Message::Text(msg))) => {
                let command = serde_json::from_str::<GameCommand>(&msg);

                if let Ok(command) = command {
                    IncomingConnectionMessageDto::Command(command)
                } else {
                    println!("Received unknown command: {}", msg);
                    IncomingConnectionMessageDto::Unknown
                }
            }
            Some(Ok(Message::Close(_))) => {
                IncomingConnectionMessageDto::Close
            }
            _ => IncomingConnectionMessageDto::Unknown,
        }
    }

    async fn send(&self, msg: OutgoingConnectionMessageDto) {
        let mut sender = self.sender.lock().await;

        sender.send(msg.to_string().into()).await.unwrap()
    }
}