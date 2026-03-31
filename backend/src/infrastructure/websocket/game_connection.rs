//! # WebSocket Connection
//!
//! Implements [`ConnectionContract`] over an Axum WebSocket, splitting the
//! socket into independent send and receive halves protected by `Mutex`.

use crate::application::common::connection::{ConnectionContract, ConnectionMessage};
use crate::application::game::commands::GameCommand;
use crate::application::game::dto::{IncomingConnectionMessageDto, OutgoingConnectionMessageDto};
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;

/// A WebSocket connection implementing [`ConnectionContract`].
///
/// Used for both GM and user connections.
pub struct GameWebSocketConnection {
    sender: Mutex<SplitSink<WebSocket, Message>>,
    receiver: Mutex<SplitStream<WebSocket>>,
}

impl GameWebSocketConnection {
    /// Creates a new connection by splitting the given WebSocket.
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }
}

impl ConnectionContract<IncomingConnectionMessageDto, OutgoingConnectionMessageDto>
    for GameWebSocketConnection
{
    async fn recv(&self) -> ConnectionMessage<IncomingConnectionMessageDto> {
        let mut receiver = self.receiver.lock().await;

        match receiver.next().await {
            Some(Ok(Message::Text(msg))) => {
                let command = serde_json::from_str::<GameCommand>(&msg);

                if let Ok(command) = command {
                    ConnectionMessage::Message(IncomingConnectionMessageDto::Command(command))
                } else {
                    println!("Received unknown command: {}", msg);
                    ConnectionMessage::Message(IncomingConnectionMessageDto::Unknown)
                }
            }
            _ => ConnectionMessage::Close,
        }
    }

    async fn send(&self, msg: OutgoingConnectionMessageDto) {
        let mut sender = self.sender.lock().await;

        sender.send(msg.to_string().into()).await.unwrap()
    }
}
