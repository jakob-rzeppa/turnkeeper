use crate::{
    application::game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    domain::common::identifier::Identifier,
};

impl GameRequestHandler {
    pub async fn set_source_code(
        &self,
        id: Identifier,
        source_code: String,
    ) -> Result<(), GameApplicationError> {
        let game = self.game_repository.get_by_id(&id).await?;

        if let Some(mut game) = game {
            game.set_source_code(source_code);
            self.game_repository.save(&game).await?;
            Ok(())
        } else {
            Err(GameApplicationError::GameNotFound)
        }
    }
}
