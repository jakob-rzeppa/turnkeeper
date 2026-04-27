use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::game_instance::GameInstance, error::GameInstanceError,
        value_objects::data::VariableValue,
    },
};

impl GameInstance {
    pub fn set_game_stat_value(
        &mut self,
        stat_name: &str,
        value: VariableValue,
    ) -> Result<(), GameInstanceError> {
        if let Some(stat) = self.game_stats.iter_mut().find(|s| s.name() == stat_name) {
            stat.set_value(value);
            Ok(())
        } else {
            Err(GameInstanceError::StatNotFound(stat_name.to_string()))
        }
    }

    pub fn set_player_stat_value(
        &mut self,
        player_name: &str,
        stat_name: &str,
        value: VariableValue,
    ) -> Result<(), GameInstanceError> {
        if let Some(stat) = self.player_stats.iter_mut().find(|s| s.name() == stat_name) {
            stat.set_value_for_player(player_name, value)
        } else {
            Err(GameInstanceError::PlayerStatNotFound(stat_name.to_string()))
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
