use std::sync::Arc;
use uuid::Uuid;
use crate::application::game::contracts::{GameRepositoryContract, GmConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::GameError;
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

    pub async fn gm_connect(&mut self, gm_conn: GmConnection) {
        println!("Gm connection established");
        self.gm_conn = Some(gm_conn);

        while let ConnectionMessageDto::Event(event) = self.gm_conn.as_mut().expect("gm_conn is some").recv().await {
            //self.game.handle(event);

            // Send game to gm
            match serde_json::to_string(&GmGameInfo::from(&self.game)) {
                Ok(json) => self.gm_conn.as_mut().expect("gm_conn is some").send(json).await,
                Err(e) => eprintln!("failed to serialize GmGameInfo: {}", e),
            }
        }

        println!("Closing GmWebSocket connection.");
    }
}
