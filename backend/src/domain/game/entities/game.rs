use std::str::FromStr;
use futures_util::SinkExt;
use uuid::Uuid;
use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;

/// The aggregate root representing a game.
///
/// # Invariants
///
/// - No two players have the same ID
/// - `current_player_index` does not exceed `players.len() - 1`
#[derive(Debug, PartialEq)]
pub struct Game {
    id: Uuid,
    name: String,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn round_number(&self) -> u32 {
        self.round_number
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }

    pub fn add_player(&mut self) -> Result<(), GameError> {
        let player = Player::new(Uuid::new_v4());
        self.players.push(player);
        Ok(())
    }

    /// Reorders players to match the given list of UUIDs.
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::InvalidPlayerOrder`] if the list length differs
    /// from the current player count, contains duplicates, or references
    /// unknown player IDs.
    pub fn change_player_order(&mut self, ids_in_order: Vec<Uuid>) -> Result<(), GameError> {
        if ids_in_order.len() != self.players.len() {
            return Err(GameError::new(GameErrorKind::InvalidPlayerOrder));
        }

        let mut new_players = Vec::with_capacity(self.players.len());
        for id in ids_in_order {
            // Check for duplicate IDs in the input list
            if new_players.iter().any(|p: &Player| p.id() == &id) {
                return Err(GameError::new(GameErrorKind::InvalidPlayerOrder));
            }

            if let Some(player) = self.players.iter().find(|p| p.id() == &id) {
                new_players.push(player.clone());
            } else {
                return Err(GameError::new(GameErrorKind::InvalidPlayerOrder));
            }
        }

        self.players = new_players;
        Ok(())
    }

    fn add_stat_to_player(&mut self, player_id: Uuid, stat_key: String, stat_type: String, stat_value: String) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            match stat_type.as_str() {
                "string" => player.try_add_string_stat(Uuid::new_v4(), stat_key, stat_value),
                "number" => {
                    let number_value = stat_value.parse::<f64>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.try_add_number_stat(Uuid::new_v4(), stat_key, number_value)
                },
                "boolean" => {
                    let boolean_value = stat_value.parse::<bool>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.try_add_bool_stat(Uuid::new_v4(), stat_key, boolean_value)
                },
                _ => Err(GameError::new(GameErrorKind::InvalidStat))
            }
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    /// Attaches a user to a player by their IDs.
    ///
    /// # Invariants
    ///
    /// - A user can only be attached to one player at a time.
    /// - The player must exist in the game.
    /// - A user can only be attached if there is no other player already attached to that user.
    ///
    /// The user is not validated here. If a user doesn't exist, it will be displayed as "User not found" in the UI, but it won't cause an error at this stage, since we don't care about the user details.
    fn attach_user_to_player(&mut self, user_id: Uuid, player_id: Uuid) -> Result<(), GameError> {
        if self.players.iter().any(|p| p.user_id() == Some(user_id)) {
            return Err(GameError::new(GameErrorKind::UserAlreadyAttachedToAnotherPlayer));
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.attach_user(user_id);
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    /// Detaches any user from the specified player.
    fn detach_user_from_player(&mut self, player_id: Uuid) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.detach_user();
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    /// Dispatches a [`GameEvent`] to the appropriate handler method.
    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), GameError> {
        match event {
            GameEvent::AddPlayer => self.add_player(),
            GameEvent::AddStatToPlayer { player_id, stat_key, stat_type, stat_value } => {
                self.add_stat_to_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    stat_key,
                    stat_type,
                    stat_value)
            },
            GameEvent::AttachUserToPlayer { user_id, player_id } => self.attach_user_to_player(
                Uuid::from_str(&user_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
            ),
            GameEvent::DetachUserFromPlayer { player_id } => self.detach_user_from_player(
                Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
            ),
            GameEvent::ChangePlayerOrder(ids_in_order) => {
                let ids_in_order: Vec<Uuid> = ids_in_order.into_iter()
                    .map(|s| Uuid::from_str(&s).map_err(|_| GameError::new(GameErrorKind::InvalidUuid)))
                    .collect::<Result<Vec<Uuid>, GameError>>()?;

                self.change_player_order(ids_in_order)
            },
            GameEvent::Debug(msg) => {
                println!("Debug event with message: {}", msg);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod attach_detach_user {
        use super::*;

        #[test]
        fn test_attach_and_detach_user() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();

            let player_id = *game.players().first().unwrap().id();
            let user_id = Uuid::new_v4();

            // Attach user to player
            let res = game.attach_user_to_player(user_id, player_id);
            assert!(res.is_ok());

            assert_eq!(game.players().first().unwrap().user_id(), Some(user_id));

            // Detach user from player
            let res = game.detach_user_from_player(player_id);
            assert!(res.is_ok());

            assert_eq!(game.players().first().unwrap().user_id(), None);
        }

        #[test]
        fn test_attach_user_already_attached() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player1_id = *game.players().get(0).unwrap().id();
            let player2_id = *game.players().get(1).unwrap().id();
            let user_id = Uuid::new_v4();

            // Attach user to first player
            let res = game.attach_user_to_player(user_id, player1_id);
            assert!(res.is_ok());

            // Attempt to attach same user to second player should fail
            let res = game.attach_user_to_player(user_id, player2_id);
            assert!(res.is_err());
        }
    }

    mod change_player_order {
        use super::*;

        #[test]
        fn test_change_player_order() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            let reversed_ids: Vec<Uuid> = player_ids.iter().rev().cloned().collect();

            game.change_player_order(reversed_ids.clone()).unwrap();

            let new_order_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            assert_eq!(new_order_ids, reversed_ids);
        }

        #[test]
        fn test_change_player_order_with_invalid_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();
            game.add_player().unwrap();

            let invalid_ids = vec![Uuid::new_v4(), Uuid::new_v4()];

            let result = game.change_player_order(invalid_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_duplicate_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            let duplicate_ids = vec![player_ids[0], player_ids[0]];

            let result = game.change_player_order(duplicate_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_many_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();

            let too_many_ids = vec![Uuid::new_v4(), Uuid::new_v4()];

            let result = game.change_player_order(too_many_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_few_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player().unwrap();
            game.add_player().unwrap();

            let too_few_ids = vec![Uuid::new_v4()];

            let result = game.change_player_order(too_few_ids);
            assert!(result.is_err());
        }
    }
}