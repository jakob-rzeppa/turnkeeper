use uuid::Uuid;
use crate::domain::game::error::{GameError, GameErrorKind};
use super::Game;

impl Game {
    pub fn add_stat_to_player(&mut self, player_id: Uuid, stat_id: Uuid, stat_key: String, stat_type: String, stat_value: String) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            match stat_type.as_str() {
                "string" => player.add_stat_string(stat_id, stat_key, stat_value),
                "number" => {
                    let number_value = stat_value.parse::<f64>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat(format!("parsing {} as number failed", stat_value.clone()))))?;
                    player.add_stat_number(stat_id, stat_key, number_value)
                },
                "boolean" => {
                    let boolean_value = stat_value.parse::<bool>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat(format!("parsing {} as boolean failed", stat_value.clone()))))?;
                    player.add_stat_bool(stat_id, stat_key, boolean_value)
                },
                _ => Err(GameError::new(GameErrorKind::InvalidStat(format!("invalid stat type: {}", stat_type.clone()))))
            }
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    pub fn change_stat_of_player(&mut self, player_id: Uuid, stat_id: Uuid, stat_type: String, stat_value: String) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            match stat_type.as_str() {
                "string" => player.change_stat_string(&stat_id, stat_value),
                "number" => {
                    let number_value = stat_value.parse::<f64>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat(format!("parsing {} as number failed", stat_value.clone()))))?;
                    player.change_stat_number(&stat_id, number_value)
                },
                "boolean" => {
                    let boolean_value = stat_value.parse::<bool>()
                        .map_err(|_| GameError::new(GameErrorKind::InvalidStat(format!("parsing {} as number failed", stat_value.clone()))))?;
                    player.change_stat_bool(&stat_id, boolean_value)
                },
                _ => Err(GameError::new(GameErrorKind::InvalidStat(format!("invalid stat type: {}", stat_type.clone()))))
            }
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }

    pub fn remove_stat_from_player(&mut self, player_id: Uuid, stat_id: Uuid) -> Result<(), GameError> {
        if let Some(player) = self.players.iter_mut().find(|p| p.id() == &player_id) {
            player.remove_stat(&stat_id)
        } else {
            Err(GameError::new(GameErrorKind::PlayerNotFound))
        }
    }
}