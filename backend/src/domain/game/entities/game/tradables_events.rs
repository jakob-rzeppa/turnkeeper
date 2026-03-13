use crate::domain::game::entities::tradable::Tradable;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::value_objects::id::Id;
use super::Game;

impl Game {
    /// Adds a new tradable to the game with the specified name and initial value.
    ///
    /// # Invariants
    ///
    /// - The `id` and `name` must be unique among all tradables in the game.
    /// - The new tradable should be initialized with a value for each existing player (defaulting to the provided `initial_value`).
    pub fn add_tradable(&mut self, tradable_id: Id, name: String, initial_value: f64) -> Result<(), GameError> {
        if self.tradables.iter().any(|t| t.id() == &tradable_id || t.name() == name) {
            return Err(GameError::new(GameErrorKind::TradableAlreadyExists));
        }

        self.tradables.push(Tradable::new(
            tradable_id,
            name,
            initial_value,
            self.players.iter().map(|p| p.id()).cloned().collect::<Vec<Id>>()
        ));
        Ok(())
    }

    pub fn remove_tradable(&mut self, tradable_id: Id) -> Result<(), GameError> {
        if let Some(pos) = self.tradables.iter().position(|t| t.id() == &tradable_id) {
            self.tradables.remove(pos);
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::TradableNotFound))
        }
    }

    /// Changes the value of a tradable for a specific player.
    ///
    /// This will only be called by the gm, so we allow negative values.
    pub fn change_player_tradable_value(&mut self, player_id: Id, tradable_id: Id, new_value: f64) -> Result<(), GameError> {
        if let Some(tradable) = self.tradables.iter_mut().find(|t| t.id() == &tradable_id) {
            tradable.change_value(player_id, new_value)
        } else {
            Err(GameError::new(GameErrorKind::TradablePlayerNotFound))
        }
    }

    /// Transfers a specified amount of a tradable from one player to another.
    ///
    /// Returns an error if the `from_id` player does not have enough of the tradable to transfer, or if either player or the tradable is not found.
    pub fn send_tradable(&mut self, from_id: Id, to_id: Id, tradable_id: Id, amount: f64) -> Result<(), GameError> {
        if let Some(tradable) = self.tradables.iter_mut().find(|t| t.id() == &tradable_id) {
            tradable.send_amount(from_id, to_id, amount)
        } else {
            Err(GameError::new(GameErrorKind::TradablePlayerNotFound))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_tradable_no_players() {
        let mut game = Game::new(Id::new(), "test-game".to_string());
        let tradable_id = Id::new();

        assert!(game.add_tradable(tradable_id, "Gold".to_string(), 100.0).is_ok());

        assert_eq!(game.tradables.len(), 1);
        assert_eq!(game.tradables[0].id(), &tradable_id);
        assert_eq!(game.tradables[0].name(), "Gold");
    }

    #[test]
    pub fn test_add_tradable_with_players() {
        let mut game = Game::new(Id::new(), "test-game".to_string());
        let player_id = Id::new();
        game.add_player(player_id).unwrap();

        let tradable_id = Id::new();
        assert!(game.add_tradable(tradable_id, "Gold".to_string(), 100.0).is_ok());

        assert_eq!(game.tradables.len(), 1);
        assert_eq!(game.tradables[0].id(), &tradable_id);
        assert_eq!(game.tradables[0].name(), "Gold");
        assert_eq!(game.tradables[0].values().get(&player_id.to_string()), Some(&100.0));
    }

    #[test]
    pub fn test_add_duplicate_tradable_fails() {
        let mut game = Game::new(Id::new(), "test-game".to_string());
        let tradable_id = Id::new();
        assert!(game.add_tradable(tradable_id, "Gold".to_string(), 100.0).is_ok());
        let result = game.add_tradable(tradable_id, "Gold".to_string(), 100.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradableAlreadyExists));
    }

    #[test]
    fn test_remove_tradable() {
        let mut game = Game::new(Id::new(), "test-game".to_string());
        let tradable_id = Id::new();
        game.add_tradable(tradable_id, "Gold".to_string(), 100.0).unwrap();

        assert!(game.remove_tradable(tradable_id).is_ok());
        assert!(game.tradables.is_empty());
    }

    #[test]
    fn test_change_player_tradable_value() {
        let mut game = Game::new(Id::new(), "test-game".to_string());

        let player_id = Id::new();
        game.add_player(player_id).unwrap();

        let tradable_id = Id::new();
        game.add_tradable(tradable_id, "Gold".to_string(), 100.0).unwrap();

        assert!(game.change_player_tradable_value(player_id, tradable_id, 150.0).is_ok());
        assert_eq!(game.tradables[0].values().get(&player_id.to_string()), Some(&150.0));
    }

    #[test]
    fn test_change_player_tradable_value_nonexistent_tradable() {
        let mut game = Game::new(Id::new(), "test-game".to_string());

        let player_id = Id::new();
        game.add_player(player_id).unwrap();

        let tradable_id = Id::new(); // Not added to the game

        let result = game.change_player_tradable_value(player_id, tradable_id, 150.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradablePlayerNotFound));
    }

    #[test]
    fn test_send_tradable() {
        let mut game = Game::new(Id::new(), "test-game".to_string());

        let player_id1 = Id::new();
        let player_id2 = Id::new();
        game.add_player(player_id1).unwrap();
        game.add_player(player_id2).unwrap();

        let tradable_id = Id::new();
        game.add_tradable(tradable_id, "Gold".to_string(), 100.0).unwrap();

        assert!(game.send_tradable(player_id1, player_id2, tradable_id, 50.0).is_ok());
        assert_eq!(game.tradables[0].values().get(&player_id1.to_string()), Some(&50.0));
        assert_eq!(game.tradables[0].values().get(&player_id2.to_string()), Some(&150.0));
    }

    #[test]
    fn test_send_nonexistent_tradable() {
        let mut game = Game::new(Id::new(), "test-game".to_string());

        let player_id1 = Id::new();
        let player_id2 = Id::new();
        game.add_player(player_id1).unwrap();
        game.add_player(player_id2).unwrap();

        let tradable_id = Id::new(); // Not added to the game

        let result = game.send_tradable(player_id1, player_id2, tradable_id, 50.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradablePlayerNotFound));
    }
}