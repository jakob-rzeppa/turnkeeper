use crate::domain::{
    common::identifier::Identifier,
    game::{entities::game_instance::GameInstance, value_objects::stat_value::StatValue},
};

impl GameInstance {
    pub fn set_game_stat(&mut self, stat_id: &Identifier, value: StatValue) {
        unimplemented!()
    }

    pub fn set_player_stat(
        &mut self,
        player_id: &Identifier,
        stat_id: &Identifier,
        value: StatValue,
    ) {
        unimplemented!()
    }

    pub fn advance_turn(&mut self) {
        unimplemented!()
    }
}
