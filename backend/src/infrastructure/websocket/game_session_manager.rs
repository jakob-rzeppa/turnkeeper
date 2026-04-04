use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    application::{
        common::channels::{
            mpsc::MpscChannelSender,
            targeted_broadcast::{TargetedBroadcastReceiver, TargetedBroadcastReceiverCreator},
        },
        game::{
            commands::GameCommand, contracts::GameRepositoryContract,
            dto::OutgoingConnectionMessageDto, session::GameSession,
        },
    },
    domain::game::{error::GameError, value_objects::id::Id},
};

pub struct GameSessionManager<GameRepository: GameRepositoryContract> {
    game_repo: Arc<GameRepository>,

    /// Active game sessions mapped by game ID.
    /// Each session holds the command sender and a creator for game state update receivers.
    sessions: Arc<
        Mutex<
            HashMap<
                Id,
                (
                    MpscChannelSender<GameCommand>,
                    TargetedBroadcastReceiverCreator<Id, OutgoingConnectionMessageDto>,
                ),
            >,
        >,
    >,
}

impl<GameRepository: GameRepositoryContract> GameSessionManager<GameRepository> {
    pub fn new(game_repo: Arc<GameRepository>) -> Self {
        Self {
            game_repo,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Retrieves the command sender and receiver creator for a given game ID.
    ///
    /// This is used by WebSocket handlers to route commands and subscribe to updates.
    pub async fn get_session(
        &self,
        game_id: &Id,
        user_id: &Id,
    ) -> Result<
        (
            MpscChannelSender<GameCommand>,
            TargetedBroadcastReceiver<Id, OutgoingConnectionMessageDto>,
        ),
        GameError,
    > {
        let mut sessions_lock = self.sessions.lock().await;

        let session = match sessions_lock.get(game_id) {
            Some(session) => session,
            None => {
                println!("Creating new session for game_id: {}", game_id);
                let new_session =
                    GameSession::spawn_session(game_id.clone(), self.game_repo.clone()).await?;

                sessions_lock.insert(game_id.clone(), new_session);
                sessions_lock
                    .get(game_id)
                    .expect("Game session right after creation not found.")
            }
        };

        let command_sender_clone = session.0.clone();
        let game_state_receiver = session
            .1
            .create_receiver(user_id.clone())
            .await
            .expect("Game state broadcast receiver creation failed.");

        Ok((command_sender_clone, game_state_receiver))
    }
}

impl<GameRepository: GameRepositoryContract> Clone for GameSessionManager<GameRepository> {
    fn clone(&self) -> Self {
        Self {
            game_repo: self.game_repo.clone(),
            sessions: self.sessions.clone(),
        }
    }
}
