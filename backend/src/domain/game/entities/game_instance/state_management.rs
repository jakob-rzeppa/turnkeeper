use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::game_instance::GameInstance, error::GameInstanceError,
        value_objects::stat_value::StatValue,
    },
};

impl GameInstance {
    pub fn set_game_stat_value(
        &mut self,
        stat_id: &Identifier,
        value: StatValue,
    ) -> Result<(), GameInstanceError> {
        if let Some(stat) = self.game_stats.iter_mut().find(|s| s.id() == stat_id) {
            stat.set_value(value);
            Ok(())
        } else {
            Err(GameInstanceError::StatNotFound(stat_id.clone()))
        }
    }

    pub fn set_player_stat_value(
        &mut self,
        player_id: &Identifier,
        stat_id: &Identifier,
        value: StatValue,
    ) -> Result<(), GameInstanceError> {
        if let Some(stat) = self.player_stats.iter_mut().find(|s| s.id() == stat_id) {
            stat.set_value_for_player(player_id, value)
        } else {
            Err(GameInstanceError::PlayerStatNotFound(stat_id.clone()))
        }
    }

    pub fn advance_turn(&mut self) {
        if self.players.is_empty() {
            return; // No players, do nothing
        }

        self.current_player_index += 1;
        if self.current_player_index >= self.players.len() {
            self.current_player_index = 0;
            self.round += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::game::{
        entities::{game::Game, stat::GameStat, stat::PlayerStat},
        value_objects::stat_visibility::GameStatVisibility,
    };

    /// Helper function to create a test GameInstance with empty collections
    fn create_test_game_instance(
        game_stats: Vec<GameStat>,
        player_stats: Vec<PlayerStat>,
    ) -> GameInstance {
        GameInstance::new(
            "Test Game".to_string(),
            game_stats,
            player_stats,
            Vec::new(),
            Vec::new(),
            Game::new("Test Game".to_string(), "Test Description".to_string()),
        )
    }

    fn create_test_game_instance_minimal() -> GameInstance {
        GameInstance::new(
            "Test Game".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Game::new("Test Game".to_string(), "Test Description".to_string()),
        )
    }

    mod set_game_stat_value {
        use super::*;

        #[test]
        fn test_sets_game_stat_value_successfully() {
            let mut game_stats = Vec::new();
            let stat_id = Identifier::new();
            game_stats.push(GameStat::new_raw(
                stat_id.clone(),
                "Score".to_string(),
                StatValue::Integer(0),
                StatValue::Integer(0),
                GameStatVisibility::Public,
            ));

            let mut game_instance = create_test_game_instance(game_stats, Vec::new());

            let result = game_instance.set_game_stat_value(&stat_id, StatValue::Integer(42));

            assert!(result.is_ok());
        }

        #[test]
        fn test_set_game_stat_value_with_float() {
            let mut game_stats = Vec::new();
            let stat_id = Identifier::new();
            game_stats.push(GameStat::new_raw(
                stat_id.clone(),
                "Damage".to_string(),
                StatValue::Float(0.0),
                StatValue::Float(0.0),
                GameStatVisibility::Public,
            ));

            let mut game_instance = create_test_game_instance(game_stats, Vec::new());

            let result = game_instance.set_game_stat_value(&stat_id, StatValue::Float(3.14));

            assert!(result.is_ok());
        }

        #[test]
        fn test_set_game_stat_value_with_boolean() {
            let mut game_stats = Vec::new();
            let stat_id = Identifier::new();
            game_stats.push(GameStat::new_raw(
                stat_id.clone(),
                "Active".to_string(),
                StatValue::Boolean(false),
                StatValue::Boolean(false),
                GameStatVisibility::Public,
            ));

            let mut game_instance = create_test_game_instance(game_stats, Vec::new());

            let result = game_instance.set_game_stat_value(&stat_id, StatValue::Boolean(true));

            assert!(result.is_ok());
        }

        #[test]
        fn test_set_game_stat_value_with_string() {
            let mut game_stats = Vec::new();
            let stat_id = Identifier::new();
            game_stats.push(GameStat::new_raw(
                stat_id.clone(),
                "Status".to_string(),
                StatValue::String("inactive".to_string()),
                StatValue::String("inactive".to_string()),
                GameStatVisibility::Public,
            ));

            let mut game_instance = create_test_game_instance(game_stats, Vec::new());

            let result = game_instance
                .set_game_stat_value(&stat_id, StatValue::String("active".to_string()));

            assert!(result.is_ok());
        }

        #[test]
        fn test_set_nonexistent_game_stat_returns_error() {
            let nonexistent_stat_id = Identifier::new();

            let mut game_instance = create_test_game_instance(Vec::new(), Vec::new());

            let result =
                game_instance.set_game_stat_value(&nonexistent_stat_id, StatValue::Integer(42));

            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                GameInstanceError::StatNotFound(_)
            ));
        }
    }

    mod advance_turn {
        use super::*;

        #[test]
        fn test_advance_turn_with_no_players_does_nothing() {
            let mut game_instance = create_test_game_instance_minimal();

            let initial_index = game_instance.current_player_index;
            let initial_round = game_instance.round;

            game_instance.advance_turn();

            assert_eq!(game_instance.current_player_index, initial_index);
            assert_eq!(game_instance.round, initial_round);
        }

        #[test]
        fn test_advance_turn_with_single_player_increments_round() {
            let mut game_instance = create_test_game_instance_minimal();

            game_instance.add_player().unwrap();

            assert_eq!(game_instance.current_player_index, 0);
            assert_eq!(game_instance.round, 1);

            game_instance.advance_turn();

            assert_eq!(game_instance.current_player_index, 0);
            assert_eq!(game_instance.round, 2);
        }

        #[test]
        fn test_advance_turn_cycles_through_players() {
            let mut game_instance = create_test_game_instance_minimal();

            game_instance.add_player().unwrap();
            game_instance.add_player().unwrap();
            game_instance.add_player().unwrap();

            assert_eq!(game_instance.current_player_index, 0);
            assert_eq!(game_instance.round, 1);

            game_instance.advance_turn();
            assert_eq!(game_instance.current_player_index, 1);
            assert_eq!(game_instance.round, 1);

            game_instance.advance_turn();
            assert_eq!(game_instance.current_player_index, 2);
            assert_eq!(game_instance.round, 1);

            game_instance.advance_turn();
            assert_eq!(game_instance.current_player_index, 0);
            assert_eq!(game_instance.round, 2);

            game_instance.advance_turn();
            assert_eq!(game_instance.current_player_index, 1);
            assert_eq!(game_instance.round, 2);
        }

        #[test]
        fn test_advance_turn_multiple_rounds() {
            let mut game_instance = create_test_game_instance_minimal();

            game_instance.add_player().unwrap();
            game_instance.add_player().unwrap();

            // Simulate 5 turns
            for _ in 0..5 {
                game_instance.advance_turn();
            }

            // After 5 turns with 2 players: 5 / 2 = 2 full rounds + 1 turn in the 3rd round
            // So current_player_index should be 1, and round should be 3
            assert_eq!(game_instance.current_player_index, 1);
            assert_eq!(game_instance.round, 3);
        }
    }
}
