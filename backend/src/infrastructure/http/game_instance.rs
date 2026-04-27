use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

use crate::{
    application::game_instance::request_handler::{
        create::GameInstanceCreateRequest, delete::GameInstanceDeleteRequest,
        list_by_game::GameInstanceListByGameRequest,
    },
    domain::{
        common::identifier::Identifier,
        game::projections::game_instance_metadata::GameInstanceMetadataProjection,
        user::entities::User,
    },
    infrastructure::{app_state::AppState, error::HttpError},
};

#[derive(Serialize, JsonResponse, Debug)]
pub struct GameInstancesGetByGameIdResponse {
    pub game_instances: Vec<GameInstanceMetadataProjection>,
}

/// GET /games/{game_id}/instances
///
/// Returns the metadata for all game instances of a game
pub async fn game_instances_get_metadata_by_game_id(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
) -> Result<GameInstancesGetByGameIdResponse, HttpError> {
    let request = GameInstanceListByGameRequest {
        game_id: game_id.into(),
    };

    let response = state
        .game_instance_request_handler()
        .list_all_games(request)
        .await?;

    Ok(GameInstancesGetByGameIdResponse {
        game_instances: response.games_metadata,
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
    let request = GameInstanceCreateRequest {
        name: request.name,
        gm_user_id: user.id().clone(),
        game_id: game_id.into(),
    };

    let id = state
        .game_instance_request_handler()
        .create(request)
        .await?;

    Ok(GameInstanceCreateHttpResponse { id })
}

/// DELETE /games/{game_id}/instances/{instance_id}
pub async fn game_instances_delete(
    State(state): State<AppState>,
    Path((game_id, instance_id)): Path<(String, String)>,
) -> Result<StatusCode, HttpError> {
    let game_id = Identifier::parse_str(&game_id)?;
    let instance_id = Identifier::parse_str(&instance_id)?;

    state
        .game_instance_request_handler()
        .delete(GameInstanceDeleteRequest {
            game_id,
            instance_id,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
