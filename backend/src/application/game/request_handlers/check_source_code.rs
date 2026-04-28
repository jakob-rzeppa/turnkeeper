use crate::{
    application::{
        common::parser::error::ParsingError,
        game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    },
    domain::{
        common::identifier::Identifier,
        game::projections::{
            action::ActionMetadataProjection,
            page::PageMetadataProjection,
            stat::{GameStatMetadataProjection, PlayerStatMetadataProjection},
        },
    },
};

pub enum CheckSourceCodeResponse {
    Valid {
        game_stats: Vec<GameStatMetadataProjection>,
        player_stats: Vec<PlayerStatMetadataProjection>,
        actions: Vec<ActionMetadataProjection>,
        pages: Vec<PageMetadataProjection>,
    },
    Invalid {
        errors: Vec<ParsingError>,
    },
}

impl GameRequestHandler {
    pub async fn check_source_code(
        &self,
        game_id: Identifier,
    ) -> Result<CheckSourceCodeResponse, GameApplicationError> {
        let game = self
            .game_repository
            .get_by_id(&game_id)
            .await?
            .ok_or(GameApplicationError::GameNotFound)?;

        let parsing_result = self.game_root_parser.parse_game(&game.source_code());

        match parsing_result {
            Ok(projections) => Ok(CheckSourceCodeResponse::Valid {
                game_stats: projections
                    .game_stats
                    .into_iter()
                    .map(|s| s.get_metadata_projection())
                    .collect(),
                player_stats: projections
                    .player_stats
                    .into_iter()
                    .map(|s| s.get_metadata_projection())
                    .collect(),
                actions: projections
                    .actions
                    .into_iter()
                    .map(|a| a.get_metadata_projection())
                    .collect(),
                pages: projections
                    .pages
                    .into_iter()
                    .map(|p| p.get_metadata_projection())
                    .collect(),
            }),
            Err(error) => Ok(CheckSourceCodeResponse::Invalid {
                errors: vec![error],
            }),
        }
    }
}
