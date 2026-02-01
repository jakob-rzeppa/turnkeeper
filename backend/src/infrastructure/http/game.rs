use axum::extract::{Path, State};
use axum::http::StatusCode;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::AppState;
use crate::domain::game::entities::game::Game;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::persistence::repositories::game::InMemoryGameRepository;

#[derive(Deserialize, JsonRequest, Debug)]
pub struct GamesCreateHttpRequest {
    pub name: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct GamesCreateHttpResponse {
    pub game_id: String,
}

/// POST /games
///
/// creates a game and returns the initial game state
pub async fn games_create(State(state): State<AppState>, request: GamesCreateHttpRequest) -> Result<GamesCreateHttpResponse, HttpError> {
    let repo = InMemoryGameRepository::new(state.in_memory_game_db);
    
    let id = Uuid::new_v4();
    let game = Game::new(id.clone(), request.name);
    
    repo.save(game).await?;
    
    Ok(GamesCreateHttpResponse {
        game_id: id.to_string(),
    })
}

/// DELETE /games/{game_id}
///
/// deletes a game if no current connection to it
pub async fn games_delete(State(state): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, HttpError> {
    let repo = InMemoryGameRepository::new(state.in_memory_game_db);

    let id = Uuid::try_from(id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;

    repo.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}