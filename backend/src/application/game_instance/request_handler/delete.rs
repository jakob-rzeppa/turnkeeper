use crate::{
    application::{
        game::contracts::GameRepositoryContract,
        game_instance::{
            contracts::GameInstanceRepositoryContract, error::GameInstanceApplicationError,
            request_handler::GameInstanceRequestHandler,
        },
    },
    domain::common::identifier::Identifier,
};

pub struct GameInstanceDeleteRequest {
    pub game_id: Identifier,
    pub instance_id: Identifier,
}

impl<GameInstanceRepository: GameInstanceRepositoryContract, GameRepository: GameRepositoryContract>
    GameInstanceRequestHandler<GameInstanceRepository, GameRepository>
{
    pub async fn delete(
        &self,
        request: GameInstanceDeleteRequest,
    ) -> Result<(), GameInstanceApplicationError> {
        self.game_instance_repository
            .delete(request.game_id, request.instance_id)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::{
        game::contracts::MockGameRepositoryContract,
        game_instance::contracts::MockGameInstanceRepositoryContract,
    };

    #[tokio::test]
    async fn test_delete_game_instance_success() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_repository = MockGameRepositoryContract::new();
        let game_id = Identifier::new();
        let game_instance_id = Identifier::new();

        game_instance_repository
            .expect_delete()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
        );
        let request = GameInstanceDeleteRequest {
            game_id,
            instance_id: game_instance_id,
        };
        let result = handler.delete(request).await;

        assert!(result.is_ok());
    }
}
