use std::str::FromStr;
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

    notes: String,
    hidden_notes: String,
}

impl Game {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
            notes: String::new(),
            hidden_notes: String::new(),
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

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn hidden_notes(&self) -> &str {
        &self.hidden_notes
    }

    fn add_player(&mut self, id: Uuid) -> Result<(), GameError> {
        if self.players.iter().any(|p| p.id() == &id) {
            return Err(GameError::new(GameErrorKind::PlayerAlreadyExists));
        }

        let player = Player::new(id);
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
    fn change_player_order(&mut self, ids_in_order: Vec<Uuid>) -> Result<(), GameError> {
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
                "string" => player.add_stat_string(Uuid::new_v4(), stat_key, stat_value),
                "number" => {
                    let number_value = stat_value.parse::<f64>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.add_stat_number(Uuid::new_v4(), stat_key, number_value)
                },
                "boolean" => {
                    let boolean_value = stat_value.parse::<bool>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.add_stat_bool(Uuid::new_v4(), stat_key, boolean_value)
                },
                _ => Err(GameError::new(GameErrorKind::InvalidStat))
            }
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    fn change_stat_of_player(&mut self, player_id: Uuid, stat_id: Uuid, stat_type: String, stat_value: String) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            match stat_type.as_str() {
                "string" => player.change_stat_string(&stat_id, stat_value),
                "number" => {
                    let number_value = stat_value.parse::<f64>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.change_stat_number(&stat_id, number_value)
                },
                "boolean" => {
                    let boolean_value = stat_value.parse::<bool>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat))?;
                    player.change_stat_bool(&stat_id, boolean_value)
                },
                _ => Err(GameError::new(GameErrorKind::InvalidStat))
            }
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    fn remove_stat_from_player(&mut self, player_id: Uuid, stat_id: Uuid) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.remove_stat(&stat_id)
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

    fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }

    fn set_hidden_notes(&mut self, hidden_notes: String) {
        self.hidden_notes = hidden_notes;
    }

    /// Dispatches a [`GameEvent`] to the appropriate handler method.
    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), GameError> {
        match event {
            GameEvent::SetNotes(notes) => {
                self.set_notes(notes);
                Ok(())
            },
            GameEvent::SetHiddenNotes(hidden_notes) => {
                self.set_hidden_notes(hidden_notes);
                Ok(())
            },
            GameEvent::AddPlayer { player_id } => {
                self.add_player(Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?)
            },
            GameEvent::AddStatToPlayer { player_id, stat_key, stat_type, stat_value } => {
                self.add_stat_to_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    stat_key,
                    stat_type,
                    stat_value)
            },
            GameEvent::ChangeStatOfPlayer { player_id, stat_id, stat_type, stat_value } => {
                self.change_stat_of_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    Uuid::from_str(&stat_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    stat_type,
                    stat_value,
                )
            },
            GameEvent::RemoveStatFromPlayer { player_id, stat_id } => {
                self.remove_stat_from_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    Uuid::from_str(&stat_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                )
            }
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

    mod add_player {
        use super::*;

        #[test]
        fn test_add_player() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            let player_id = Uuid::new_v4();
            assert!(game.add_player(player_id).is_ok());
            assert_eq!(game.players().len(), 1);
            assert_eq!(game.players().first().unwrap().id(), &player_id);
        }

        #[test]
        fn test_add_duplicate_player() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            let player_id = Uuid::new_v4();
            assert!(game.add_player(player_id).is_ok());
            let result = game.add_player(player_id);
            assert!(result.is_err());
        }
    }

    mod attach_detach_user {
        use super::*;

        #[test]
        fn test_attach_and_detach_user() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player(Uuid::new_v4()).unwrap();

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
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();

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
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();

            let player_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            let reversed_ids: Vec<Uuid> = player_ids.iter().rev().cloned().collect();

            game.change_player_order(reversed_ids.clone()).unwrap();

            let new_order_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            assert_eq!(new_order_ids, reversed_ids);
        }

        #[test]
        fn test_change_player_order_with_invalid_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();

            let invalid_ids = vec![Uuid::new_v4(), Uuid::new_v4()];

            let result = game.change_player_order(invalid_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_duplicate_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();

            let player_ids: Vec<Uuid> = game.players.iter().map(|p| *p.id()).collect();
            let duplicate_ids = vec![player_ids[0], player_ids[0]];

            let result = game.change_player_order(duplicate_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_many_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player(Uuid::new_v4()).unwrap();

            let too_many_ids = vec![Uuid::new_v4(), Uuid::new_v4()];

            let result = game.change_player_order(too_many_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_few_ids() {
            let mut game = Game::new(Uuid::new_v4(), "test-game".to_string());
            game.add_player(Uuid::new_v4()).unwrap();
            game.add_player(Uuid::new_v4()).unwrap();

            let too_few_ids = vec![Uuid::new_v4()];

            let result = game.change_player_order(too_few_ids);
            assert!(result.is_err());
        }
    }
}