use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use tokio::sync::{mpsc, oneshot};

#[derive(Debug, thiserror::Error)]
pub enum TargetedBroadcastError {
    #[error("Failed to send game state: {0}")]
    SendError(String),
    #[error("Failed to create receiver: {0}")]
    CreateReceiverError(String),
}

/// A channel for sending messages from one producer (e.g. the game session) to a specific user connection.
///
/// This is used for sending game state updates or error messages to a specific user, rather than broadcasting to all users.
pub struct TargetedBroadcast<K, T> {
    receiver_key: PhantomData<K>,
    message_type: PhantomData<T>,
}

impl<K: Clone + Debug + Send + PartialEq + Eq + Hash + 'static, T: Clone + Debug + Send + 'static>
    TargetedBroadcast<K, T>
{
    pub fn new() -> (
        TargetedBroadcastSender<K, T>,
        TargetedBroadcastReceiverCreator<K, T>,
    ) {
        // The producer side of the channel, which gets the messages to be send
        let (producer_sender, mut producer_receiver) = mpsc::unbounded_channel();
        let sender = TargetedBroadcastSender {
            sender: producer_sender,
        };

        // The consumer side of the channel, which allows creating a receiver for a specific user ID
        let (creator_sender, mut creator_receiver) =
            mpsc::unbounded_channel::<(K, oneshot::Sender<TargetedBroadcastReceiver<K, T>>)>();
        let receiver_creator = TargetedBroadcastReceiverCreator {
            creator: creator_sender,
        };

        // Spawn a task to route messages from the producer to the correct consumer based on user ID
        tokio::spawn(async move {
            // Map of user_id to the sender for that user's receiver
            let mut consumer_senders: std::collections::HashMap<K, mpsc::UnboundedSender<T>> =
                std::collections::HashMap::new();

            loop {
                tokio::select! {
                    // A new message to send to a user or broadcast
                    msg = producer_receiver.recv() => {
                        match msg {
                            Some(msg) => {
                                if let Some(to_user_id) = msg.0 {
                                    // Send to specific user
                                    if let Some(consumer_sender) = consumer_senders.get(&to_user_id) {
                                        let _ = consumer_sender.send(msg.1);
                                    }
                                } else {
                                    // Broadcast to all users
                                    for consumer_sender in consumer_senders.values() {
                                        let _ = consumer_sender.send(msg.1.clone());
                                    }
                                }
                            }
                            None => {
                                // The producer channel was closed, so we can stop the task
                                creator_receiver.close(); // Close the creator channel
                                break; // Exit the loop -> drop all consumer senders, which will close those channels as well
                            },
                        }
                    },
                    // A request to create a new receiver for a specific user ID
                    creator_msg = creator_receiver.recv() => {
                        match creator_msg {
                            Some((user_id, response_sender)) => {
                                // Create a new channel for this user
                                let (consumer_sender, consumer_receiver) = mpsc::unbounded_channel();
                                consumer_senders.insert(user_id.clone(), consumer_sender);

                                // Send the receiver back to the requester
                                let _ = response_sender.send(TargetedBroadcastReceiver {
                                    user_id,
                                    receiver: consumer_receiver,
                                });
                            }
                            None => panic!("TargetedBroadcastReceiverCreator was dropped, but TargetedBroadcastSender is still alive"),
                        }
                    },
                }
            }
        });

        (sender, receiver_creator)
    }
}

pub struct TargetedBroadcastSender<K, T> {
    sender: mpsc::UnboundedSender<(Option<K>, T)>,
}

impl<K, T> TargetedBroadcastSender<K, T> {
    pub async fn send_to(&self, to_user_id: K, message: T) -> Result<(), TargetedBroadcastError> {
        self.sender
            .send((Some(to_user_id), message))
            .map_err(|e| TargetedBroadcastError::SendError(e.to_string()))?;

        Ok(())
    }

    pub async fn broadcast(&self, message: T) -> Result<(), TargetedBroadcastError> {
        self.sender
            .send((None, message))
            .map_err(|e| TargetedBroadcastError::SendError(e.to_string()))?;

        Ok(())
    }
}

pub struct TargetedBroadcastReceiverCreator<K, T> {
    creator: mpsc::UnboundedSender<(K, oneshot::Sender<TargetedBroadcastReceiver<K, T>>)>,
}

impl<K, T> TargetedBroadcastReceiverCreator<K, T> {
    pub async fn create_receiver(
        &self,
        user_id: K,
    ) -> Result<TargetedBroadcastReceiver<K, T>, TargetedBroadcastError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.creator
            .send((user_id, response_sender))
            .map_err(|e| TargetedBroadcastError::CreateReceiverError(e.to_string()))?;

        response_receiver
            .await
            .map_err(|e| TargetedBroadcastError::CreateReceiverError(e.to_string()))
    }
}

pub struct TargetedBroadcastReceiver<K, T> {
    user_id: K,
    receiver: mpsc::UnboundedReceiver<T>,
}

impl<K, T> TargetedBroadcastReceiver<K, T> {
    pub fn user_id(&self) -> &K {
        &self.user_id
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_targeted_broadcast() {
        let (sender, receiver_creator) = TargetedBroadcast::<String, String>::new();

        // Create receivers for two users
        let mut receiver1 = receiver_creator
            .create_receiver("user1".to_string())
            .await
            .unwrap();
        let mut receiver2 = receiver_creator
            .create_receiver("user2".to_string())
            .await
            .unwrap();

        // Send a message to user1
        sender
            .send_to("user1".to_string(), "Hello User 1".to_string())
            .await
            .unwrap();
        assert_eq!(receiver1.recv().await.unwrap(), "Hello User 1");

        // Send a message to user2
        sender
            .send_to("user2".to_string(), "Hello User 2".to_string())
            .await
            .unwrap();
        assert_eq!(receiver2.recv().await.unwrap(), "Hello User 2");

        // Broadcast a message to all users
        sender
            .broadcast("Hello Everyone".to_string())
            .await
            .unwrap();
        assert_eq!(receiver1.recv().await.unwrap(), "Hello Everyone");
        assert_eq!(receiver2.recv().await.unwrap(), "Hello Everyone");

        drop(sender); // Close the sender to stop the routing task

        // After the sender is dropped, the receivers should stop receiving messages
        assert!(receiver1.recv().await.is_none());
        assert!(receiver2.recv().await.is_none());

        // The receiver creator should also be closed, so creating a new receiver should fail
        let result = receiver_creator.create_receiver("user3".to_string()).await;
        assert!(result.is_err());
    }
}
