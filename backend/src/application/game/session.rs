use std::sync::Arc;
use uuid::Uuid;
use crate::application::game::contracts::{GameRepositoryContract, GmConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::GameError;
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::GmGameInfo;

pub struct GameSession<GmConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    GameRepository: GameRepositoryContract
{
    game: Game,
    gm_conn: Option<GmConnection>,
    game_repo: Arc<GameRepository>,
}

impl<GmConnection, GameRepository> GameSession<GmConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    GameRepository: GameRepositoryContract
{
    pub async fn try_new(game_id: Uuid, game_repository: Arc<GameRepository>) -> Result<Self, GameError> {
        let game_metadata = game_repository.get_metadata_by_id(game_id).await?;

        let game = Game::new(game_metadata.id, game_metadata.name);

        Ok(Self {
            game,
            gm_conn: None,
            game_repo: game_repository,
        })
    }

    async fn handle_event(&mut self, event: GameEvent) {
        let res = self.game.handle_event(event.clone());

        if res.is_ok() {
            // Persist the game state only if the event was handled successfully
            // if let Err(e) = self.game_repo.log_event(event).await {
            //     eprintln!("Failed to save game state: {}", e);
            // }
        } else {
            // TODO: Send error to gm
            eprintln!("Failed to handle event: {}", res.err().unwrap());
        }

        self.broadcast_game_state().await;
    }

    async fn broadcast_game_state(&mut self) {
        if let Some(gm_conn) = &mut self.gm_conn {
            match serde_json::to_string(&GmGameInfo::from(&self.game)) {
                Ok(json) => gm_conn.send(json).await,
                Err(e) => eprintln!("failed to serialize GmGameInfo: {}", e),
            }
        }
    }

    pub async fn gm_connect(&mut self, gm_conn: GmConnection) {
        println!("Gm connection established");
        self.gm_conn = Some(gm_conn);

        while let ConnectionMessageDto::Event(event) = self.gm_conn.as_mut().expect("gm_conn is some").recv().await {
            self.handle_event(event).await;
        }

        println!("Closing GmWebSocket connection.");
        self.gm_conn = None;
    }
}
