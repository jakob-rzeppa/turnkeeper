use std::{collections::HashMap, sync::Arc};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{application::{common::channels::{mpsc::MpscChannelSender, targeted_broadcast::TargetedBroadcastReceiver}, game::error::GameApplicationError, game_instance::{contracts::GameInstanceRepositoryContract, dto::{IncomingMessageDto, OutgoingMessageDto}, session::GameSession}}, domain::common::identifier::Id};

pub struct GameSessionManager {
    game_instance_repo: Arc<dyn GameInstanceRepositoryContract>,

    /// Active game sessions and connected user ids mapped by game ID.
    sessions: Arc<Mutex<HashMap<Id, (GameSession, Vec<Id>)>>>,
}

impl GameSessionManager {
    pub fn new(game_instance_repo: Arc<dyn GameInstanceRepositoryContract>) -> Self {

        let sessions = Arc::new(Mutex::new(HashMap::<Id, (GameSession, Vec<Id>)>::new()));
        let sessions_clone = sessions.clone();

        // This thread is responsible for logging the active game sessions and connected users every few seconds.
        // The thread is spawned and runs indefinitly. This is save because the GameSessionManager is only created once and lives for the entire duration of the application, so we don't have to worry about the thread outliving the GameSessionManager or being spawned multiple times.
        tokio::spawn( async move {
            println!("[STARTUP] Starting Game Session Manager logger task...");
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                let sessions_guard = sessions_clone.lock().await;
                println!("Active Game Sessions: {}", sessions_guard.len());
                for (game_id, (_session, user_ids)) in sessions_guard.iter() {
                    println!("-> Game ID: {}, Connected Users: {}", game_id, user_ids.len());
                }
            }
        });
        
        Self {
            game_instance_repo,
            sessions,
        }
    }

    /// Connects to a game session for the given game ID, creating a new session if one doesn't already exist.
    /// 
    /// Returns the channels for sending commands to the session and receiving updates from the session.
    /// 
    /// You MUST call `disconnect_from_session` when the connection is closed to ensure that we don't leave any sessions running indefinitely after all users have disconnected.
    pub async fn connect_to_session(&self, game_id: &Id, user_id: &Id) -> Result<(
        MpscChannelSender<IncomingMessageDto>, 
        TargetedBroadcastReceiver<Id, OutgoingMessageDto>
    ), GameApplicationError> {
        let mut sessions_guard = self.sessions.lock().await;

        // Spawn a new session if it doesn't exist
        if !sessions_guard.contains_key(game_id) {
            let new_session = GameSession::spawn(game_id.clone(), self.game_instance_repo.clone()).await?;
            sessions_guard.insert(game_id.clone(), (new_session, Vec::new()));
        }

        if let Some(session) = sessions_guard.get_mut(game_id) {
            session.1.push(user_id.clone()); // Add the user to the list of connected users
            Ok(session.0.get_channels(user_id).await)
        } else {
            unreachable!("Session should have been created if it didn't exist.");
        }
    }

    /// Disconnects from a game session for the given game ID, and stops the session if there are no more connected users.
    /// 
    /// This should be called when a WebSocket connection is closed to ensure that we don't leave any sessions running indefinitely after all users have disconnected.
    pub async fn disconnect_from_session(&self, game_id: &Id, user_id: &Id) {
        let mut sessions_guard = self.sessions.lock().await;

        let mut should_stop_session = false;
        if let Some(session) = sessions_guard.get_mut(game_id) {
            session.1.remove(session.1.iter().position(|id| id == user_id)
                .unwrap_or_else(|| panic!("Disconnect called for user_id {} which is not in the session's connected users list", user_id)));

            if session.1.is_empty() {
                // Mark the session for stopping if there are no more connected users
                should_stop_session = true;
            }
        }

        // If there are no more connected users, stop the session and remove it from the active sessions.
        if should_stop_session {
            if let Some(session) = sessions_guard.remove(game_id) {
                // Stop the session gracefully (consumes the session)
                session.0.stop().await;
            }
        }
    }
}

impl Clone for GameSessionManager {
    fn clone(&self) -> Self {
        Self {
            game_instance_repo: self.game_instance_repo.clone(),
            sessions: self.sessions.clone(),
        }
    }
}