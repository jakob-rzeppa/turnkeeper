use axum::extract::{Path, State};
use axum::http::StatusCode;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::game::request_handlers::create::{CreateGameRequestHandler};
use crate::application::game::request_handlers::delete::DeleteGameRequestHandler;
use crate::application::game::request_handlers::get_overview::GameGetOverviewRequestHandler;
use crate::application::game::requests::{CreateGameRequest, DeleteGameRequest};
use crate::AppState;
use crate::domain::game::projections::game_metadata::GameMetadata;
use crate::infrastructure::error::HttpError;

#[derive(Serialize, Debug)]
pub struct GamesGetResponseGameMetadata {
    pub id: String,
    pub name: String,
}

impl From<GameMetadata> for GamesGetResponseGameMetadata {
    fn from(metadata: GameMetadata) -> Self {
        Self {
            id: metadata.id.to_string(),
            name: metadata.name,
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
    let handler = GameGetOverviewRequestHandler::new(state.repository_manager.game());
    
    let games_overview = handler.get_overview().await?;
    
    Ok(GamesGetResponse {
        games: games_overview.games_metadata.into_iter().map(|metadata| metadata.into()).collect(),
    })
}

#[derive(Deserialize, JsonRequest, Debug)]
pub struct GamesCreateHttpRequest {
    pub name: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GamesCreateHttpResponse {
    pub id: String,
}

/// POST /games
///
/// Creates a game and returns the initial game state
pub async fn games_create(State(state): State<AppState>, request: GamesCreateHttpRequest) -> Result<GamesCreateHttpResponse, HttpError> {
    let handler = CreateGameRequestHandler::new(state.repository_manager.game());

    let id = handler.create_game(CreateGameRequest {
        name: request.name,
    }).await?;

    Ok(GamesCreateHttpResponse {
        id: id.to_string()
    })
}

/// DELETE /games/{game_id}
///
/// Deletes a game if no current connection to it
pub async fn games_delete(State(state): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, HttpError> {
    let handler = DeleteGameRequestHandler::new(state.repository_manager.game());

    let id = Uuid::try_from(id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;

    handler.delete_game(DeleteGameRequest { id }).await?;

    Ok(StatusCode::NO_CONTENT)
}