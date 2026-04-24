use crate::AppState;
use crate::application::game::request_handlers::create::{
    CreateGameRequest, CreateGameRequestHandler,
};
use crate::application::game::request_handlers::delete::{
    DeleteGameRequest, DeleteGameRequestHandler,
};
use crate::application::game::request_handlers::get_by_id::GameGetByIdRequestHandler;
use crate::application::game::request_handlers::list_all::GameListAllRequestHandler;
use crate::domain::common::date_time::DateTime;
use crate::domain::common::identifier::Identifier;
use crate::domain::game::projections::game::GameProjection;
use crate::domain::game::projections::game_metadata::GameMetadataProjection;
use crate::domain::user::entities::User;
use crate::infrastructure::error::HttpError;
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
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
    let handler = GameListAllRequestHandler::new(state.repository_manager.game());

    let games_overview = handler.list_all().await?;

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
    let handler = GameGetByIdRequestHandler::new(state.repository_manager.game());

    let res = handler.get_by_id(id).await?;

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
    let handler = CreateGameRequestHandler::new(state.repository_manager.game());

    let id = handler
        .create_game(CreateGameRequest {
            name: request.name,
            description: request.description,
        })
        .await?;

    Ok(GamesCreateHttpResponse { id: id.to_string() })
}

/// DELETE /games/{game_id}
///
/// Deletes a game if no current connection to it
pub async fn games_delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, HttpError> {
    let handler = DeleteGameRequestHandler::new(state.repository_manager.game());

    let id = Identifier::parse_str(&id)?;

    handler.delete_game(DeleteGameRequest { id }).await?;

    Ok(StatusCode::NO_CONTENT)
}
