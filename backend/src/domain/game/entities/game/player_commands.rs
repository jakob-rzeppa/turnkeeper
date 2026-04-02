use super::Game;
use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::value_objects::id::Id;

impl Game {
    /// Adds a new player to the game with the specified ID.
    ///
    /// # Invariants
    ///
    /// - The `id` must be unique among all players in the game.
    /// - The new player should be added to all existing tradables with a default value.
    pub fn add_player(&mut self, id: Id) -> Result<(), GameError> {
        if self.players.iter().any(|p| p.id() == &id) {
            return Err(GameError::new(GameErrorKind::PlayerAlreadyExists));
        }

        let player = Player::new(id.clone());
        self.players.push(player);

        // Update all tradables to include the new player with a default value
        self.tradables.iter_mut().for_each(|t| t.add_player(id));

        Ok(())
    }

    /// Reorders players to match the given list of UUIDs.
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::InvalidPlayerOrder`] if the list length differs
    /// from the current player count, contains duplicates, or references
    /// unknown player IDs.
    pub fn change_player_order(&mut self, ids_in_order: Vec<Id>) -> Result<(), GameError> {
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

    /// Attaches a user to a player by their IDs.
    ///
    /// # Invariants
    ///
    /// - A user can only be attached to one player at a time.
    /// - The player must exist in the game.
    /// - A user can only be attached if there is no other player already attached to that user.
    ///
    /// The user is not validated here. If a user doesn't exist, it will be displayed as "User not found" in the UI, but it won't cause an error at this stage, since we don't care about the user details.
    pub fn attach_user_to_player(&mut self, user_id: Id, player_id: Id) -> Result<(), GameError> {
        if self.players.iter().any(|p| p.user_id() == Some(user_id)) {
            return Err(GameError::new(
                GameErrorKind::UserAlreadyAttachedToAnotherPlayer,
            ));
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.attach_user(user_id);
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    /// Detaches any user from the specified player.
    pub fn detach_user_from_player(&mut self, player_id: Id) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.detach_user();
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
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
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id = Id::new();

            assert!(game.add_player(player_id).is_ok());

            assert_eq!(game.players.len(), 1);
            assert_eq!(game.players[0].id(), &player_id);
        }

        #[test]
        fn test_add_duplicate_player_fails() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id = Id::new();

            assert!(game.add_player(player_id).is_ok());

            let result = game.add_player(player_id);
            assert!(result.is_err());

            match result {
                Err(e) => {
                    assert_eq!(e, GameError::new(GameErrorKind::PlayerAlreadyExists));
                }
                Ok(_) => panic!("Expected error"),
            }
        }

        #[test]
        fn test_add_player_with_tradables() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let tradable_id = Id::new();
            game.add_tradable(tradable_id, "Gold".to_string(), 100.0)
                .unwrap();

            let player_id = Id::new();
            assert!(game.add_player(player_id).is_ok());

            assert_eq!(game.players.len(), 1);
            assert_eq!(
                game.tradables[0].values().get(&player_id.to_string()),
                Some(&100.0)
            );
        }
    }

    mod change_player_order {
        use super::*;

        #[test]
        fn test_change_player_order() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id_1 = Id::new();
            let player_id_2 = Id::new();
            let player_id_3 = Id::new();

            game.add_player(player_id_1).unwrap();
            game.add_player(player_id_2).unwrap();
            game.add_player(player_id_3).unwrap();

            game.change_player_order(vec![player_id_3, player_id_1, player_id_2])
                .unwrap();

            assert_eq!(game.players[0].id(), &player_id_3);
            assert_eq!(game.players[1].id(), &player_id_1);
            assert_eq!(game.players[2].id(), &player_id_2);
        }

        #[test]
        fn test_change_player_order_fails_with_wrong_count() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id_1 = Id::new();
            let player_id_2 = Id::new();
            let player_id_3 = Id::new();

            game.add_player(player_id_1).unwrap();
            game.add_player(player_id_2).unwrap();
            game.add_player(player_id_3).unwrap();

            let result = game.change_player_order(vec![player_id_1, player_id_2]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_duplicate_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id_1 = Id::new();
            let player_id_2 = Id::new();

            game.add_player(player_id_1).unwrap();
            game.add_player(player_id_2).unwrap();

            let result = game.change_player_order(vec![player_id_1, player_id_1]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_unknown_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
            let player_id_1 = Id::new();
            let player_id_2 = Id::new();
            let player_id_3 = Id::new();

            game.add_player(player_id_1).unwrap();
            game.add_player(player_id_2).unwrap();

            let result = game.change_player_order(vec![player_id_1, player_id_3]);
            assert!(result.is_err());
        }
    }
}
