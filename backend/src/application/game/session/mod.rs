//! # Game Session
//!
//! A `GameSession` represents an active, in-memory instance of a single game.
//! It owns the [`GameRuntime`] and manages real-time communication with
//! the connected Game Master (GM) and user players over WebSocket connections.

use std::sync::Arc;

use crate::application::common::channels::mpsc::{
    MpscChannel, MpscChannelReceiver, MpscChannelSender,
};
use crate::application::common::channels::targeted_broadcast::{
    TargetedBroadcast, TargetedBroadcastReceiverCreator, TargetedBroadcastSender,
};
use crate::application::game::commands::GameCommand;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::dto::OutgoingConnectionMessageDto;
use crate::application::game::runtime::GameRuntime;
use crate::domain::game::error::GameError;
use crate::domain::game::value_objects::id::Id;

/// An active in-memory game session.
///
/// Owns the [`GameRuntime`] and manages connections
/// from the GM and multiple user players.
pub struct GameSession<GameRepository: GameRepositoryContract> {
    /// The live game runtime that holds all current game state.
    runtime: GameRuntime,
    /// Shared repository used for persistence operations.
    game_repo: Arc<GameRepository>,

    /// The user command receiver (user_id, command)
    command_receiver: MpscChannelReceiver<GameCommand>,
    // The broadcast sender for sending game state updates to connected clients.
    game_state_broadcaster: TargetedBroadcastSender<Id, OutgoingConnectionMessageDto>,
}

impl<GameRepository: GameRepositoryContract> GameSession<GameRepository> {
    pub async fn spawn_session(
        game_id: Id,
        game_repo: Arc<GameRepository>,
    ) -> Result<
        (
            MpscChannelSender<GameCommand>,
            TargetedBroadcastReceiverCreator<Id, OutgoingConnectionMessageDto>,
        ),
        GameError,
    > {
        let metadata = game_repo.get_metadata_by_id(game_id).await?;
        let mut runtime = GameRuntime::new(metadata);

        let history = game_repo.get_game_history(game_id).await?;
        for command in history {
            runtime
                .handle_command(command)
                .expect("Database contains invalid state.");
        }

        let (command_sender, command_receiver) = MpscChannel::new();
        let (game_state_broadcaster, game_state_broadcast_receiver_creator) =
            TargetedBroadcast::new();

        let mut session = Self {
            runtime,
            game_repo,
            command_receiver,
            game_state_broadcaster,
        };

        // Spawn a task to continuously process incoming commands
        tokio::spawn(async move {
            while let Some(command) = session.command_receiver.recv().await {
                session.handle_command(command).await;
            }
        });

        Ok((command_sender, game_state_broadcast_receiver_creator))
    }

    async fn handle_command(&mut self, command: GameCommand) {
        match self.runtime.handle_command(command.clone()) {
            Ok(_) => {
                // Persist the game state only if the command was handled successfully
                self.game_repo
                    .log_command(self.runtime.get_id(), command)
                    .await
                    .expect("Failed to log command to database.");
            }
            Err(e) => {
                eprintln!("Failed to handle command: {:?}", e);

                self.game_state_broadcaster
                    .send_to(
                        self.runtime.get_gm_user_id(),
                        OutgoingConnectionMessageDto::GameError(e.into()),
                    )
                    .await
                    .expect("Failed to send error message to GM.");
            }
        };

        self.broadcast_game_state().await;
    }

    async fn broadcast_game_state(&self) {
        // For the gm we can send the full game projection, which includes all details.
        let game_state = self.runtime.get_gm_game_projection();
        self.game_state_broadcaster
            .send_to(
                self.runtime.get_gm_user_id(),
                OutgoingConnectionMessageDto::FullGameProjection(game_state),
            )
            .await
            .expect("Broadcasting full game state failed.");

        // For the players we send a projection that hides secret information.
        for user_id in self.runtime.get_user_ids() {
            let game_state = self.runtime.get_player_game_projection(&user_id);
            self.game_state_broadcaster
                .send_to(
                    user_id,
                    OutgoingConnectionMessageDto::PlayerGameProjection(game_state),
                )
                .await
                .expect("Broadcasting player game state failed.");
        }
    }
}
