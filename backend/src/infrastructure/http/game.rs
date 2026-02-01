use axum::extract::State;
use axum::Json;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::AppState;
use crate::domain::game::entities::game::Game;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::persistence::repositories::game::InMemoryGameRepository;

/// GET /games
///
/// returns a list of all available games
pub async fn games_get_all() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

/// GET /games/:id
///
/// returns the game state of the game with given id
pub async fn games_get() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

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

/// DELETE /games
///
/// deletes a game if no current connection to it
pub async fn games_delete(State(state): State<AppState>) -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}