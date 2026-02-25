use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use crate::application::game::contracts::GmConnectionContract;
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::events::GameEvent;

pub struct WebSocketGmConnection {
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
}

impl WebSocketGmConnection {
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self { sender, receiver }
    }
}

impl GmConnectionContract for WebSocketGmConnection {
    async fn recv(&mut self) -> ConnectionMessageDto {
        while let Some(Ok(message)) = self.receiver.next().await {
            match message {
                Message::Text(msg) => {
                    match msg.as_str() {
                        "ADD_PLAYER" => return ConnectionMessageDto::Event(GameEvent::AddPlayer),
                        _ => println!("Received unknown event: {}", msg),
                    }
                }
                Message::Close(_) => {
                    return ConnectionMessageDto::Close;
                }
                _ => {}
            }
        }

        ConnectionMessageDto::Close
    }

    async fn send(&mut self, msg: String) {
        println!("Sending message: {}", msg);
        self.sender.send(msg.into()).await.unwrap()
    }
}