use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::{game_instance::GameInstance, player::Player},
        error::GameInstanceError,
    },
};

impl GameInstance {
    /// Adds a new player to the game with the specified ID.
    ///
    /// # Invariants
    ///
    /// - The `id` must be unique among all players in the game.
    /// - The new player should be added to all existing tradables with a default value.
    pub fn add_player(&mut self) -> Result<(), GameInstanceError> {
        let player = Player::new();
        self.players.push(player);

        Ok(())
    }

    /// Reorders players to match the given list of UUIDs.
    ///
    /// # Errors
    ///
    /// Returns [`GameInstanceError::InvalidPlayerOrder`] if the list length differs
    /// from the current player count, contains duplicates, or references
    /// unknown player IDs.
    pub fn change_player_order(
        &mut self,
        ids_in_order: Vec<Identifier>,
    ) -> Result<(), GameInstanceError> {
        if ids_in_order.len() != self.players.len() {
            return Err(GameInstanceError::InvalidPlayerOrder);
        }

        let mut new_players = Vec::with_capacity(self.players.len());
        for id in ids_in_order {
            // Check for duplicate IDs in the input list
            if new_players.iter().any(|p: &Player| p.id() == &id) {
                return Err(GameInstanceError::InvalidPlayerOrder);
            }

            if let Some(player) = self.players.iter().find(|p| p.id() == &id) {
                new_players.push(player.clone());
            } else {
                return Err(GameInstanceError::InvalidPlayerOrder);
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
    pub fn attach_user_to_player(
        &mut self,
        user_id: Identifier,
        player_id: Identifier,
    ) -> Result<(), GameInstanceError> {
        if self.players.iter().any(|p| p.user_id() == Some(&user_id)) {
            return Err(GameInstanceError::UserAlreadyAttachedToAnotherPlayer);
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.attach_user(user_id);
            Ok(())
        } else {
            Err(GameInstanceError::PlayerNotFound(player_id.to_string()))
        }
    }

    /// Detaches any user from the specified player.
    pub fn detach_user_from_player(
        &mut self,
        player_id: Identifier,
    ) -> Result<(), GameInstanceError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.detach_user();
            Ok(())
        } else {
            Err(GameInstanceError::PlayerNotFound(player_id.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::game::entities::game::Game;

    use super::*;

    fn create_game_instance() -> GameInstance {
        GameInstance::new(
            "Test Game".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Game::new("Test Game".to_string(), "Test Game".to_string()),
        )
    }

    mod add_player {
        use super::*;

        #[test]
        fn test_add_player() {
            let mut game = create_game_instance();

            assert!(game.add_player().is_ok());

            assert_eq!(game.players.len(), 1);
        }
    }

    mod change_player_order {
        use super::*;

        #[test]
        fn test_change_player_order() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_id_1 = game.players[0].id().clone();
            let player_id_2 = game.players[1].id().clone();
            let player_id_3 = game.players[2].id().clone();

            game.change_player_order(vec![player_id_3, player_id_1, player_id_2])
                .unwrap();

            assert_eq!(game.players[0].id(), &player_id_3);
            assert_eq!(game.players[1].id(), &player_id_1);
            assert_eq!(game.players[2].id(), &player_id_2);
        }

        #[test]
        fn test_change_player_order_fails_with_wrong_count() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_id_1 = game.players[0].id().clone();
            let player_id_2 = game.players[1].id().clone();

            let result = game.change_player_order(vec![player_id_1, player_id_2]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_duplicate_ids() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_id_1 = game.players[0].id().clone();

            let result = game.change_player_order(vec![player_id_1, player_id_1]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_unknown_ids() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_id_1 = game.players[0].id().clone();
            let player_id_3 = Identifier::new(); // This ID does not exist in the game

            let result = game.change_player_order(vec![player_id_1, player_id_3]);
            assert!(result.is_err());
        }
    }
}
