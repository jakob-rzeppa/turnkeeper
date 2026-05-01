use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::{
    application::{
        common::channels::{
            mpsc::{MpscChannel, MpscChannelSender},
            targeted_broadcast::{TargetedBroadcast, TargetedBroadcastReceiverCreator},
        },
        game_instance::{
            commands::GameSessionCommand,
            contracts::GameInstanceRepositoryContract,
            dto::{IncomingMessageDto, OutgoingMessageDto},
            error::GameInstanceApplicationError,
        },
    },
    domain::common::identifier::Identifier,
};

pub struct GameSession {
    /// The task handle for the session task, which continuously processes incoming commands and updates the game instance state.
    task_handle: Option<JoinHandle<()>>,
    /// The shutdown sender is used to signal the session task to shut down gracefully when the GameSession is stopped.
    shutdown_sender: Option<tokio::sync::oneshot::Sender<()>>,
    // Both need to be kept in a Option, because they are consumed when stopping the session and the session is only dropped when session stopped gracefully. This means we need to set them to None after consuming them.
}

impl GameSession {
    pub async fn spawn_session(
        game_id: Identifier,
        game_instance_repository: Arc<dyn GameInstanceRepositoryContract>,
    ) -> Result<
        (
            Self,
            MpscChannelSender<IncomingMessageDto>,
            TargetedBroadcastReceiverCreator<Identifier, OutgoingMessageDto>,
        ),
        GameInstanceApplicationError,
    > {
        let (incoming_sender, mut incoming_receiver) = MpscChannel::new();
        let (outgoing_sender, outgoing_sender_creator) = TargetedBroadcast::new();
        let (shutdown_sender, mut shutdown_receiver) = tokio::sync::oneshot::channel::<()>();

        // Spawn a task to continuously process incoming commands
        let task_handle = tokio::spawn(async move {
            let mut game_instance = match game_instance_repository.get_by_id(game_id).await {
                Ok(Some(instance)) => instance,
                Ok(None) => {
                    eprintln!("Game instance with id {} not found", game_id);
                    return;
                }
                Err(e) => {
                    eprintln!(
                        "Database error while fetching game instance {}: {}",
                        game_id, e
                    );
                    return;
                }
            };

            // Process incoming messages until the channel is closed (i.e., the senders are dropped)
            while let Some(msg) = incoming_receiver.recv().await {
                match msg {
                    IncomingMessageDto::Command {
                        command,
                        sending_user_id,
                    } => match command {
                        GameSessionCommand::Connect => {
                            let res = outgoing_sender
                                .send_to(
                                    sending_user_id,
                                    OutgoingMessageDto::DisplayTemplate(
                                        game_instance.get_display_template(sending_user_id),
                                    ),
                                )
                                .await;
                            if let Err(e) = res {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(format!(
                                            "Sending display template failed: {}",
                                            e
                                        )),
                                    )
                                    .await;
                            }
                        }
                        GameSessionCommand::AddPlayer => {
                            if &sending_user_id != game_instance.gm_user_id() {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(
                                            "Only the GM can add players".to_string(),
                                        ),
                                    )
                                    .await;
                                continue;
                            }

                            let res = game_instance.add_player();
                            if let Err(e) = res {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(format!(
                                            "Adding player failed: {}",
                                            e
                                        )),
                                    )
                                    .await;
                            }
                        }
                        GameSessionCommand::ChangePlayerOrder { names_in_order } => {
                            if &sending_user_id != game_instance.gm_user_id() {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(
                                            "Only the GM can change player order".to_string(),
                                        ),
                                    )
                                    .await;
                                continue;
                            }

                            let res = game_instance.change_player_order(names_in_order);
                            if let Err(e) = res {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(format!(
                                            "Changing player order failed: {}",
                                            e
                                        )),
                                    )
                                    .await;
                            }
                        }
                        GameSessionCommand::AttachUserToPlayer { user_id, player } => {
                            if &sending_user_id != game_instance.gm_user_id() {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(
                                            "Only the GM can attach users to players".to_string(),
                                        ),
                                    )
                                    .await;
                                continue;
                            }

                            let res = game_instance.attach_user_to_player(user_id, player);
                            if let Err(e) = res {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(format!(
                                            "Attaching user to player failed: {}",
                                            e
                                        )),
                                    )
                                    .await;
                            }
                        }
                        GameSessionCommand::DetachUserFromPlayer { player } => {
                            if &sending_user_id != game_instance.gm_user_id() {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(
                                            "Only the GM can detach users from players".to_string(),
                                        ),
                                    )
                                    .await;
                                continue;
                            }

                            let res = game_instance.detach_user_from_player(player);
                            if let Err(e) = res {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(format!(
                                            "Detaching user from player failed: {}",
                                            e
                                        )),
                                    )
                                    .await;
                            }
                        }
                        GameSessionCommand::AdvanceTurn => {
                            if &sending_user_id != game_instance.gm_user_id() {
                                _ = outgoing_sender
                                    .send_to(
                                        sending_user_id,
                                        OutgoingMessageDto::Error(
                                            "Only the GM can advance the turn".to_string(),
                                        ),
                                    )
                                    .await;
                                continue;
                            }

                            game_instance.advance_turn();
                        }
                        GameSessionCommand::Debug(msg) => {
                            println!("Debug command received: {}", msg);
                        }
                        GameSessionCommand::ExecuteAction { action, payload } => {
                            unimplemented!(
                                "Action execution not implemented yet: {} with payload {}",
                                action,
                                payload
                            );
                        }
                    },
                }

                // Try to save the updated game instance after processing the command, but don't crash the session if saving fails
                // Instead, broadcast the error and continue processing further commands
                match game_instance_repository.save(&game_instance).await {
                    Ok(()) => {}
                    Err(e) => {
                        _ = outgoing_sender.broadcast(OutgoingMessageDto::Error(format!(
                            "Game Instance couldn't be saved: {}",
                            e
                        )));
                    }
                }

                // Send the updated state to the gm
                _ = outgoing_sender.send_to(
                    game_instance.gm_user_id().clone(),
                    OutgoingMessageDto::State(game_instance.get_state(game_instance.gm_user_id())),
                ); // Ignore errors since the gm might not be attached to a player and thus not receive state updates

                // Send the updated state to all attached players
                for user_id in game_instance.get_attatched_user_ids() {
                    _ = outgoing_sender.send_to(
                        user_id,
                        OutgoingMessageDto::State(game_instance.get_state(&user_id)),
                    ); // Ignore errors since some users might not be attached to a player and thus not receive state updates
                }

                // Check for shutdown signal without blocking, so that we can shut down the session gracefully when requested
                match shutdown_receiver.try_recv() {
                    Ok(_) | Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                        println!("Shutdown signal received, stopping game session task");

                        match game_instance_repository.save(&game_instance).await {
                            Ok(()) => {}
                            Err(e) => {
                                eprintln!("Game Instance couldn't be saved during shutdown: {}", e);
                            }
                        }

                        break;
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {} // No shutdown signal, continue processing
                }
            }

            // On shutdown, try to save the final state of the game instance, but ignore any errors since we're shutting down anyway
            _ = game_instance_repository.save(&game_instance).await;
        });

        Ok((
            Self {
                task_handle: Some(task_handle),
                shutdown_sender: Some(shutdown_sender),
            },
            incoming_sender,
            outgoing_sender_creator,
        ))
    }

    /// Stops the game session by sending a shutdown signal to the session task and waiting for it to finish.
    /// This allows the session task to shut down gracefully, ensuring that any in-flight commands are processed and the final state of the game instance is saved before the task is terminated.
    /// Since it consumes self, it ensures, that GameSession is also dropped.
    pub async fn stop(mut self) {
        if let Some(shutdown_sender) = self.shutdown_sender.take() {
            let _ = shutdown_sender.send(()); // Ignore error since it just means the session task has already been shut down
            if let Some(handle) = self.task_handle.take() {
                let _ = handle.await; // Wait for the session task to finish, but ignore any errors since we're shutting down anyway
            }
        }
    }
}

/// Ensure that the session task is aborted when the GameSession is dropped.
impl Drop for GameSession {
    fn drop(&mut self) {
        if let Some(handle) = self.task_handle.take() {
            let _ = handle.abort();
        }
    }
}
