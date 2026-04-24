use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::game_instance::GameInstance,
        projections::{
            game_display_template::GameDisplayTemplateProjection,
            game_instance_state::GameInstanceStateProjection,
        },
    },
};

impl GameInstance {
    /// Returns a template for the game, like the available actions, pages etc.
    /// This can be sent to the frontend for rendering the game's pages.
    pub fn get_display_template(&self, _user_id: Identifier) -> GameDisplayTemplateProjection {
        GameDisplayTemplateProjection {}
    }

    /// Returns the current state of the game instance for the specified user, including any relevant data.
    /// This can be used to update the frontend with the latest game state.
    /// The game state is used to populate the pages with the updated state.
    pub fn get_state(&self, _user_id: Identifier) -> GameInstanceStateProjection {
        GameInstanceStateProjection {}
    }
}
