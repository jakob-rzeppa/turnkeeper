use crate::{
    application::game_instance::{
        error::GameInstanceApplicationError, request_handler::GameInstanceRequestHandler,
    },
    domain::common::identifier::Identifier,
};

pub struct GameInstanceDeleteRequest {
    pub game_id: Identifier,
    pub instance_id: Identifier,
}

impl GameInstanceRequestHandler {
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
        common::parser::MockGameParserContract, game::contracts::MockGameRepositoryContract, game_instance::contracts::MockGameInstanceRepositoryContract
    };

    #[tokio::test]
    async fn test_delete_game_instance_success() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();
        let game_id = Identifier::new();
        let game_instance_id = Identifier::new();

        game_instance_repository
            .expect_delete()
            .times(1)
            .returning(|_, _| Ok(()));

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
            Arc::new(game_root_parser),
        );
        let request = GameInstanceDeleteRequest {
            game_id,
            instance_id: game_instance_id,
        };
        let result = handler.delete(request).await;

        assert!(result.is_ok());
    }
}
