use crate::application::game::request_handlers::check_source_code::CheckSourceCodeResponse;
use crate::application::game::request_handlers::create::CreateGameRequest;
use crate::application::game::request_handlers::delete::DeleteGameRequest;
use crate::domain::common::date_time::DateTime;
use crate::domain::common::identifier::Identifier;
use crate::domain::game::projections::action::ActionMetadataProjection;
use crate::domain::game::projections::game::GameProjection;
use crate::domain::game::projections::game_metadata::GameMetadataProjection;
use crate::domain::game::projections::page::PageMetadataProjection;
use crate::domain::game::projections::stat::GameStatMetadataProjection;
use crate::domain::game::projections::stat::PlayerStatMetadataProjection;
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::error::HttpError;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GamesGetResponseGameMetadata {
    pub id: Identifier,
    pub name: String,
    pub description: String,

    pub created_at: String,
    pub updated_at: String,
}

impl From<GameMetadataProjection> for GamesGetResponseGameMetadata {
    fn from(metadata: GameMetadataProjection) -> Self {
        Self {
            id: metadata.id,
            name: metadata.name,
            description: metadata.description,

            created_at: metadata.created_at.to_string(),
            updated_at: metadata.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GamesGetResponse {
    pub games: Vec<GamesGetResponseGameMetadata>,
}

/// GET /games
///
/// Returns the metadata for all created games
pub async fn games_get(State(state): State<AppState>) -> Result<GamesGetResponse, HttpError> {
    let games_overview = state.game_request_handler().list_all().await?;

    Ok(GamesGetResponse {
        games: games_overview
            .games_metadata
            .into_iter()
            .map(|metadata| metadata.into())
            .collect(),
    })
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GamesGetByIdResponse {
    pub id: Identifier,
    pub name: String,
    pub description: String,

    pub source_code: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<GameProjection> for GamesGetByIdResponse {
    fn from(metadata: GameProjection) -> Self {
        Self {
            id: metadata.id,
            name: metadata.name,
            description: metadata.description,
            source_code: metadata.source_code,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        }
    }
}

/// GET /games/{game_id}
///
/// Returns the full game projection for a game
pub async fn games_get_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<GamesGetByIdResponse, HttpError> {
    let res = state
        .game_request_handler()
        .get_by_id(Identifier::parse_str(&id)?)
        .await?;

    Ok(res.game.into())
}

#[derive(Deserialize, JsonRequest, Debug)]
pub struct GamesCreateHttpRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GamesCreateHttpResponse {
    pub id: String,
}

/// POST /games
///
/// Creates a game and returns the initial game state
pub async fn games_create(
    State(state): State<AppState>,
    request: GamesCreateHttpRequest,
) -> Result<GamesCreateHttpResponse, HttpError> {
    let id = state
        .game_request_handler()
        .create(CreateGameRequest {
            name: request.name,
            description: request.description,
        })
        .await?;

    Ok(GamesCreateHttpResponse { id: id.to_string() })
}

#[derive(Deserialize, JsonRequest, Debug)]
pub struct GamesUpdateSourceCodeHttpRequest {
    pub source_code: String,
}

/// PATCH /games/{game_id}/source-code
///
/// Updates the source code of a game
pub async fn games_update_source_code(
    State(state): State<AppState>,
    Path(id): Path<String>,
    request: GamesUpdateSourceCodeHttpRequest,
) -> Result<StatusCode, HttpError> {
    let id = Identifier::parse_str(&id)?;

    state
        .game_request_handler()
        .set_source_code(id, request.source_code)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize, Debug)]
struct GamesCheckSourceCodeValidRequestOutput {
    game_stats: Vec<GameStatMetadataProjection>,
    player_stats: Vec<PlayerStatMetadataProjection>,
    actions: Vec<ActionMetadataProjection>,
    pages: Vec<PageMetadataProjection>,
}

#[derive(Serialize, JsonResponse, Debug)]
struct GamesCheckSourceCodeValidResponse {
    is_valid: bool,
    output: GamesCheckSourceCodeValidRequestOutput,
}

#[derive(Serialize, JsonResponse, Debug)]
struct GamesCheckSourceCodeInvalidResponse {
    is_valid: bool,
    errors: Vec<String>,
}

/// GET /games/{game_id}/check
///
/// Checks if the source code of a game is valid and can be parsed
pub async fn games_check_source_code(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
) -> Result<Response, HttpError> {
    let game_id = Identifier::parse_str(&game_id)?;

    let response = state
        .game_request_handler()
        .check_source_code(game_id)
        .await?;

    match response {
        CheckSourceCodeResponse::Valid {
            game_stats,
            player_stats,
            actions,
            pages,
        } => Ok(GamesCheckSourceCodeValidResponse {
            is_valid: true,
            output: GamesCheckSourceCodeValidRequestOutput {
                game_stats,
                player_stats,
                actions,
                pages,
            },
        }
        .into_response()),
        CheckSourceCodeResponse::Invalid { errors } => Ok(GamesCheckSourceCodeInvalidResponse {
            is_valid: false,
            errors: errors.into_iter().map(|e| e.to_string()).collect(),
        }
        .into_response()),
    }
}

/// DELETE /games/{game_id}
///
/// Deletes a game if no current connection to it
pub async fn games_delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, HttpError> {
    let id = Identifier::parse_str(&id)?;

    state
        .game_request_handler()
        .delete(DeleteGameRequest { id })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
