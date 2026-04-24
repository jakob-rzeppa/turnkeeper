use axum::{
    Extension,
    extract::{Path, State},
};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    application::game_instance::request_handlers::{
        create::{CreateGameInstanceRequest, CreateGameInstanceRequestHandler},
        delete::{DeleteGameInstanceRequest, DeleteGameInstanceRequestHandler},
        list_by_game::{GameInstanceListByGameRequest, GameInstanceListByGameRequestHandler},
    },
    domain::{
        common::identifier::Identifier,
        game::projections::game_instance_metadata::GameInstanceMetadataProjection,
        user::entities::User,
    },
    infrastructure::error::HttpError,
};

pub struct GameInstancesGetByGameIdResponse {
    pub games: Vec<GameInstanceMetadataProjection>,
}

/// GET /games/{game_id}/instances
///
/// Returns the metadata for all game instances of a game
pub async fn game_instances_get_metadata_by_game_id(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
) -> Result<GameInstancesGetByGameIdResponse, HttpError> {
    let handler =
        GameInstanceListByGameRequestHandler::new(state.repository_manager.game_instance());

    let request = GameInstanceListByGameRequest {
        game_id: game_id.into(),
    };

    let response = handler.list_all_games(request).await?;

    Ok(GameInstancesGetByGameIdResponse {
        games: response.games_metadata,
    })
}

#[derive(Deserialize, JsonRequest, Debug)]
pub struct GameInstanceCreateHttpRequest {
    pub name: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GameInstanceCreateHttpResponse {
    pub id: Identifier,
}

/// POST /games/{game_id}/instances
pub async fn game_instances_post(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(game_id): Path<String>,
    request: GameInstanceCreateHttpRequest,
) -> Result<GameInstanceCreateHttpResponse, HttpError> {
    let handler = CreateGameInstanceRequestHandler::new(
        state.repository_manager.game_instance(),
        state.repository_manager.game(),
    );

    let request = CreateGameInstanceRequest {
        name: request.name,
        gm_user_id: user.id().clone(),
        game_id: game_id.into(),
    };

    let id = handler.create_game(request).await?;

    Ok(GameInstanceCreateHttpResponse { id })
}

/// DELETE /games/{game_id}/instances/{instance_id}
pub async fn game_instances_delete(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
    Path(instance_id): Path<String>,
) -> Result<(), HttpError> {
    let handler = DeleteGameInstanceRequestHandler::new(state.repository_manager.game_instance());

    let game_id = Identifier::parse_str(&game_id)?;
    let instance_id = Identifier::parse_str(&instance_id)?;

    handler
        .delete(DeleteGameInstanceRequest {
            game_id,
            instance_id,
        })
        .await?;

    Ok(())
}
