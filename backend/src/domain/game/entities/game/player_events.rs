use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::value_objects::id::Id;
use super::Game;

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
            let mut game = Game::new(Id::new(), "test-game".to_string());
            let player_id = Id::new();
            assert!(game.add_player(player_id).is_ok());
            assert_eq!(game.players().len(), 1);
            assert_eq!(game.players().first().unwrap().id(), &player_id);
        }

        #[test]
        fn test_add_duplicate_player() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            let player_id = Id::new();
            assert!(game.add_player(player_id).is_ok());
            let result = game.add_player(player_id);
            assert!(result.is_err());
        }

        #[test]
        fn test_adding_player_updates_tradables() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            let tradable_id1 = Id::new();
            let tradable_id2 = Id::new();
            game.add_tradable(tradable_id1, "Gold".to_string(), 100.0).unwrap();
            game.add_tradable(tradable_id2, "Money".to_string(), 50.0).unwrap();

            assert_eq!(game.tradables.len(), 2);

            let player_id = Id::new();
            game.add_player(player_id).unwrap();

            let tradable1 = &game.tradables[0];
            assert_eq!(tradable1.values().get(&player_id.to_string()), Some(&100.0));

            let tradable2 = &game.tradables[1];
            assert_eq!(tradable2.values().get(&player_id.to_string()), Some(&50.0));
        }
    }

    mod attach_detach_user {
        use super::*;

        #[test]
        fn test_attach_and_detach_user() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();

            let player_id = *game.players().first().unwrap().id();
            let user_id = Id::new();

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
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();

            let player1_id = *game.players().get(0).unwrap().id();
            let player2_id = *game.players().get(1).unwrap().id();
            let user_id = Id::new();

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
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();

            let player_ids: Vec<Id> = game.players.iter().map(|p| *p.id()).collect();
            let reversed_ids: Vec<Id> = player_ids.iter().rev().cloned().collect();

            game.change_player_order(reversed_ids.clone()).unwrap();

            let new_order_ids: Vec<Id> = game.players.iter().map(|p| *p.id()).collect();
            assert_eq!(new_order_ids, reversed_ids);
        }

        #[test]
        fn test_change_player_order_with_invalid_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();

            let invalid_ids = vec![Id::new(), Id::new()];

            let result = game.change_player_order(invalid_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_duplicate_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();

            let player_ids: Vec<Id> = game.players.iter().map(|p| *p.id()).collect();
            let duplicate_ids = vec![player_ids[0], player_ids[0]];

            let result = game.change_player_order(duplicate_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_many_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();

            let too_many_ids = vec![Id::new(), Id::new()];

            let result = game.change_player_order(too_many_ids);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_with_too_few_ids() {
            let mut game = Game::new(Id::new(), "test-game".to_string());
            game.add_player(Id::new()).unwrap();
            game.add_player(Id::new()).unwrap();

            let too_few_ids = vec![Id::new()];

            let result = game.change_player_order(too_few_ids);
            assert!(result.is_err());
        }
    }
}