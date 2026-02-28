use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use crate::application::game::contracts::{ConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::events::GameEvent;

pub struct WebSocketConnection {
    sender: Mutex<SplitSink<WebSocket, Message>>,
    receiver: Mutex<SplitStream<WebSocket>>,
}

impl WebSocketConnection {
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver)
        }
    }
}

impl ConnectionContract for WebSocketConnection {
    async fn recv(&self) -> ConnectionMessageDto {
        let mut receiver = self.receiver.lock().await;

        match receiver.next().await {
            Some(Ok(Message::Text(msg))) => {
                let event = serde_json::from_str::<GameEvent>(&msg);

                if let Ok(event) = event {
                    ConnectionMessageDto::Event(event)
                } else {
                    println!("Received unknown event: {}", msg);
                    ConnectionMessageDto::Unknown
                }
            }
            Some(Ok(Message::Close(_))) => {
                ConnectionMessageDto::Close
            }
            _ => ConnectionMessageDto::Unknown,
        }
    }

    async fn send(&self, msg: String) {
        let mut sender = self.sender.lock().await;

        sender.send(msg.into()).await.unwrap()
    }
}