use crate::AppState;
use crate::application::game::request_handlers::create::CreateGameRequestHandler;
use crate::application::game::request_handlers::delete::DeleteGameRequestHandler;
use crate::application::game::request_handlers::get_overview::GameGetOverviewRequestHandler;
use crate::application::game::requests::{CreateGameRequest, DeleteGameRequest};
use crate::domain::game::projections::game_metadata::GameMetadata;
use crate::domain::game::value_objects::id::Id;
use crate::domain::user::entities::User;
use crate::infrastructure::error::HttpError;
use axum::Extension;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GamesGetResponseGameMetadata {
    pub id: Id,
    pub name: String,
    pub gm_user_id: Id,
}

impl From<GameMetadata> for GamesGetResponseGameMetadata {
    fn from(metadata: GameMetadata) -> Self {
        Self {
            id: metadata.id,
            name: metadata.name,
            gm_user_id: metadata.gm_user_id,
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
        games: games_overview
            .games_metadata
            .into_iter()
            .map(|metadata| metadata.into())
            .collect(),
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
pub async fn games_create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    request: GamesCreateHttpRequest,
) -> Result<GamesCreateHttpResponse, HttpError> {
    let handler = CreateGameRequestHandler::new(state.repository_manager.game());

    let id = handler
        .create_game(CreateGameRequest {
            name: request.name,
            gm_user_id: *user.id(),
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

    let id = Id::parse_str(&id)?;

    handler.delete_game(DeleteGameRequest { id }).await?;

    Ok(StatusCode::NO_CONTENT)
}
