use std::marker::PhantomData;

use tokio::sync::mpsc;

#[derive(Debug, thiserror::Error)]
pub enum MpscChannelError {
    #[error("Attempted to send message to closed channel: {0}")]
    SendToClosedChannel(String),
}

/// A channel for sending game commands from WebSocket handlers to the game session.
pub struct MpscChannel<T> {
    message_type: PhantomData<T>,
}

impl<T> MpscChannel<T> {
    /// Creates a new MPSC channel and returns the sender and receiver.
    pub fn new() -> (MpscChannelSender<T>, MpscChannelReceiver<T>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (
            MpscChannelSender { sender },
            MpscChannelReceiver { receiver },
        )
    }
}

pub struct MpscChannelSender<T> {
    sender: mpsc::UnboundedSender<T>,
}

impl<T> MpscChannelSender<T> {
    /// Sends a message through the channel. Returns an error if the channel is closed.
    pub fn send(&self, message: T) -> Result<(), MpscChannelError> {
        self.sender
            .send(message)
            .map_err(|e| MpscChannelError::SendToClosedChannel(e.to_string()))
    }
}

impl<T> Clone for MpscChannelSender<T> {
    fn clone(&self) -> Self {
        MpscChannelSender {
            sender: self.sender.clone(),
        }
    }
}

pub struct MpscChannelReceiver<T> {
    receiver: mpsc::UnboundedReceiver<T>,
}

impl<T> MpscChannelReceiver<T> {
    /// Receives the next message from the channel. Returns `None` if the channel is closed.
    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mpsc_channel() {
        let (sender, mut receiver) = MpscChannel::<String>::new();

        sender.send("Hello".to_string()).unwrap();
        sender.send("World".to_string()).unwrap();

        assert_eq!(receiver.recv().await, Some("Hello".to_string()));
        assert_eq!(receiver.recv().await, Some("World".to_string()));

        drop(sender); // Close the channel
        assert_eq!(receiver.recv().await, None); // Channel is closed
    }
}
