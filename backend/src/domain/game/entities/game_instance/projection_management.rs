use crate::domain::{
    common::identifier::Identifier,
    game::{
        entities::game_instance::GameInstance,
        projections::{
            game_display_template::GameDisplayTemplateProjection,
            game_instance_metadata::GameInstanceMetadataProjection,
            game_instance_state::{
                GameInstanceStateProjection, GameStatStateProjection, PlayerProjection,
                PlayerStatStateProjection,
            },
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
    pub fn get_state(&self, _user_id: &Identifier) -> GameInstanceStateProjection {
        // For now, we return everything to everyone, but in the future we will need to filter out some data based on the user's permissions and the visibility of the stats.
        GameInstanceStateProjection {
            round: self.round,
            current_player_index: self.current_player_index,
            game_stats: self
                .game_stats
                .iter()
                .map(|s| GameStatStateProjection {
                    name: s.name().into(),
                    value: s.value().clone().into(),
                    default: s.default().clone().into(),
                    visibility: s.visibility().to_string(),
                })
                .collect(),
            player_stats: self
                .player_stats
                .iter()
                .map(|s| PlayerStatStateProjection {
                    name: s.name().into(),
                    values: s
                        .values()
                        .iter()
                        .map(|(player_name, value)| (player_name.clone(), value.clone().into()))
                        .collect(),
                    default: s.default().clone().into(),
                    visibility: s.visibility().to_string(),
                })
                .collect(),
            players: self
                .players
                .iter()
                .map(|p| PlayerProjection {
                    name: p.name().to_string(),
                    user_id: p.user_id().cloned(),
                })
                .collect(),
        }
    }

    pub fn get_metadata_projection(&self) -> GameInstanceMetadataProjection {
        GameInstanceMetadataProjection {
            id: self.id.clone(),
            name: self.name.clone(),
            game_id: self.source_game.id().clone(),
            gm_user_id: self.gm_user_id.clone(),
            created_at: self.created_at.clone(),
            last_played_at: self.last_played_at.clone(),
            player_count: self.players.len(),
            current_round: self.round,
        }
    }
}
