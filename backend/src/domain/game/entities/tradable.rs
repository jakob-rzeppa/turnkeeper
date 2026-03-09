use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::game::error::{GameError, GameErrorKind};

/// The representation of a tradable item in the game, which can have different values for different players.
///
/// # Invariants
///
/// - all the player ids should be valid
///     - if a player is added or removed it should update the tradable values accordingly
#[derive(Debug, Clone, PartialEq)]
pub struct Tradable {
    id: Uuid,
    name: String,

    // The default value for this tradable, used when a new player is added or when a tradable is created with existing players.
    initial_value: f64,

    // PlayerId -> value
    values: HashMap<String, f64>,
}

impl Tradable {
    pub fn id(&self) -> &Uuid { &self.id }

    pub fn name(&self) -> &str { &self.name }

    pub fn values(&self) -> &HashMap<String, f64> { &self.values }

    pub fn value_for_player(&self, player_id: Uuid) -> Result<f64, GameError> {
        if let Some(value) = self.values.get(&player_id.to_string()) {
            Ok(*value)
        } else {
            Err(GameError::new(GameErrorKind::TradablePlayerNotFound))
        }
    }

    pub fn new(id: Uuid, name: String, initial_value: f64, player_ids: Vec<Uuid> ) -> Self {
        let mut values = HashMap::new();
        for player_id in player_ids {
            values.insert(player_id.to_string(), initial_value);
        }

        Self {
            id,
            name,
            initial_value,
            values,
        }
    }

    pub fn add_player(&mut self, player_id: Uuid) {
        if (self.values.contains_key(&player_id.to_string())) {
            return; // Player already has a value, do nothing
        }

        self.values.insert(player_id.to_string(), self.initial_value);
    }

    pub fn remove_player(&mut self, player_id: Uuid) {
        self.values.remove(&player_id.to_string());
    }

    pub fn change_value(&mut self, player_id: Uuid, new_value: f64) -> Result<(), GameError> {
        if let Some(value) = self.values.get_mut(&player_id.to_string()) {
            *value = new_value;
            Ok(())
        } else {
            Err(GameError::new(GameErrorKind::TradablePlayerNotFound))
        }
    }

    pub fn send_amount(&mut self, from_id: Uuid, to_id: Uuid, amount: f64) -> Result<(), GameError> {
        let from_value = self.values.get(&from_id.to_string()).ok_or(GameError::new(GameErrorKind::TradablePlayerNotFound))?.clone();
        let to_value = self.values.get(&to_id.to_string()).ok_or(GameError::new(GameErrorKind::TradablePlayerNotFound))?.clone();

        if from_value < amount {
            return Err(GameError::new(GameErrorKind::InsufficientTradableValue));
        }

        self.change_value(from_id, from_value - amount)?;
        self.change_value(to_id, to_value + amount)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tradable_creation() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1, player_id2]);

        assert_eq!(tradable.id(), &tradable_id);
        assert_eq!(tradable.name(), "Gold");
        assert_eq!(tradable.values().get(&player_id1.to_string()), Some(&100.0));
        assert_eq!(tradable.values().get(&player_id2.to_string()), Some(&100.0));
    }

    #[test]
    fn test_add_player_to_tradable() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1]);

        assert_eq!(tradable.values().get(&player_id1.to_string()), Some(&100.0));
        assert_eq!(tradable.values().get(&player_id2.to_string()), None);

        tradable.add_player(player_id2);

        assert_eq!(tradable.values().get(&player_id2.to_string()), Some(&100.0));
    }

    #[test]
    fn test_remove_player_from_tradable() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1, player_id2]);

        assert_eq!(tradable.values().get(&player_id1.to_string()), Some(&100.0));
        assert_eq!(tradable.values().get(&player_id2.to_string()), Some(&100.0));

        tradable.remove_player(player_id1);

        assert_eq!(tradable.values().get(&player_id1.to_string()), None);
        assert_eq!(tradable.values().get(&player_id2.to_string()), Some(&100.0));
    }

    #[test]
    fn test_change_tradable_value() {
        let tradable_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id]);

        assert_eq!(tradable.values().get(&player_id.to_string()), Some(&100.0));

        tradable.change_value(player_id, 150.0).unwrap();

        assert_eq!(tradable.values().get(&player_id.to_string()), Some(&150.0));
    }

    #[test]
    fn test_change_tradable_value_nonexistent_player() {
        let tradable_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![]);

        let result = tradable.change_value(player_id, 150.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradablePlayerNotFound));
    }

    #[test]
    fn test_send_tradable() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1, player_id2]);

        assert!(tradable.send_amount(player_id1, player_id2, 30.0).is_ok());
        assert_eq!(tradable.values().get(&player_id1.to_string()), Some(&70.0));
        assert_eq!(tradable.values().get(&player_id2.to_string()), Some(&130.0));
    }

    #[test]
    fn test_send_tradable_insufficient_value() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1, player_id2]);

        let result = tradable.send_amount(player_id1, player_id2, 150.0);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::InsufficientTradableValue));
    }

    #[test]
    fn test_send_tradable_nonexistent_sender() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id2]);

        let result = tradable.send_amount(player_id1, player_id2, 30.0);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradablePlayerNotFound));
    }

    #[test]
    fn test_send_tradable_nonexistent_receiver() {
        let tradable_id = Uuid::new_v4();
        let player_id1 = Uuid::new_v4();
        let player_id2 = Uuid::new_v4();

        let mut tradable = Tradable::new(tradable_id, "Gold".to_string(), 100.0, vec![player_id1]);

        let result = tradable.send_amount(player_id1, player_id2, 30.0);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::TradablePlayerNotFound));
    }
}