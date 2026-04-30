use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::{game_instance::GameInstance, weak::player::Player},
        error::GameInstanceError,
    },
};

impl GameInstance {
    /// Adds a new player to the game with the specified ID.
    ///
    /// # Invariants
    ///
    /// - The `name` must be unique among all players in the game.
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
        names_in_order: Vec<String>,
    ) -> Result<(), GameInstanceError> {
        if names_in_order.len() != self.players.len() {
            return Err(GameInstanceError::InvalidPlayerOrder(names_in_order));
        }

        let mut new_players = Vec::with_capacity(self.players.len());
        for name in names_in_order.clone() {
            // Check for duplicate names in the input list
            if new_players.iter().any(|p: &Player| p.name() == &name) {
                return Err(GameInstanceError::InvalidPlayerOrder(names_in_order));
            }

            if let Some(player) = self.players.iter().find(|p| p.name() == &name) {
                new_players.push(player.clone());
            } else {
                return Err(GameInstanceError::InvalidPlayerOrder(names_in_order));
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
        player_name: String,
    ) -> Result<(), GameInstanceError> {
        if self.players.iter().any(|p| p.user_id() == Some(&user_id)) {
            return Err(GameInstanceError::UserAlreadyAttachedToAnotherPlayer);
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.name() == &player_name) {
            player.attach_user(user_id);
            Ok(())
        } else {
            Err(GameInstanceError::PlayerNotFound(player_name))
        }
    }

    /// Detaches any user from the specified player.
    pub fn detach_user_from_player(
        &mut self,
        player_name: String,
    ) -> Result<(), GameInstanceError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.name() == &player_name) {
            player.detach_user();
            Ok(())
        } else {
            Err(GameInstanceError::PlayerNotFound(player_name))
        }
    }

    pub fn get_player_names(&self) -> Vec<String> {
        self.players.iter().map(|p| p.name().to_string()).collect()
    }

    pub fn get_attatched_user_ids(&self) -> Vec<Identifier> {
        self.players
            .iter()
            .filter_map(|p| p.user_id().cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::game::entities::game::Game;

    use super::*;

    fn create_game_instance() -> GameInstance {
        GameInstance::new(
            "Test Game".to_string(),
            Identifier::new(),
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

            let player_name_1 = game.players[0].name().to_string();
            let player_name_2 = game.players[1].name().to_string();
            let player_name_3 = game.players[2].name().to_string();

            game.change_player_order(vec![
                player_name_3.clone(),
                player_name_1.clone(),
                player_name_2.clone(),
            ])
            .unwrap();

            assert_eq!(game.players[0].name(), &player_name_3);
            assert_eq!(game.players[1].name(), &player_name_1);
            assert_eq!(game.players[2].name(), &player_name_2);
        }

        #[test]
        fn test_change_player_order_fails_with_wrong_count() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_name_1 = game.players[0].name().to_string();
            let player_name_2 = game.players[1].name().to_string();

            let result = game.change_player_order(vec![player_name_1, player_name_2]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_duplicate_ids() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_name_1 = game.players[0].name().to_string();

            let result = game.change_player_order(vec![player_name_1.clone(), player_name_1]);
            assert!(result.is_err());
        }

        #[test]
        fn test_change_player_order_fails_with_unknown_ids() {
            let mut game = create_game_instance();

            game.add_player().unwrap();
            game.add_player().unwrap();

            let player_name_1 = game.players[0].name().to_string();
            let player_name_3 = "Unknown Player".to_string(); // This name does not exist in the game

            let result = game.change_player_order(vec![player_name_1, player_name_3]);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_get_attached_user_ids() {
        let mut game = create_game_instance();

        game.add_player().unwrap();
        game.add_player().unwrap();

        // No attachments yet, should return an empty list
        let user_ids = game.get_attatched_user_ids();
        assert_eq!(user_ids.len(), 0);

        game.get_player_names();
        game.attach_user_to_player(Identifier::new(), game.players[0].name().to_string())
            .unwrap();

        let user_ids = game.get_attatched_user_ids();
        assert_eq!(user_ids.len(), 1);

        game.attach_user_to_player(Identifier::new(), game.players[1].name().to_string())
            .unwrap();

        let user_ids = game.get_attatched_user_ids();
        assert_eq!(user_ids.len(), 2);
    }
}
