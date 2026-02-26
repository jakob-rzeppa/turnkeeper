use uuid::Uuid;
use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::{GmGameInfo, GmPlayerInfo, GmPlayerUserInfo, GmStatInfo};

/// The representation of the game
///
/// # Invariants
///
/// - Two Players have the same ID
/// - current_player_index is greater than length of players - 1
#[derive(Debug, PartialEq)]
pub struct Game {
    id: Uuid,
    name: String,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn add_player(&mut self) -> Result<(), GameError> {
        let player = Player::new(Uuid::new_v4());
        self.players.push(player);
        Ok(())
    }

    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), GameError> {
        match event {
            GameEvent::AddPlayer => self.add_player(),
            GameEvent::ChangePlayerOrder(ids_in_order) => {
                println!("Change player order event with new order: {:?}", ids_in_order);
                Ok(())
            }
            GameEvent::Debug(msg) => {
                println!("Debug event with message: {}", msg);
                Ok(())
            }
        }
    }
}

impl From<&Game> for GmGameInfo {
    fn from(value: &Game) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.to_string(),
            players: value.players.iter().map(|p| GmPlayerInfo {
                id: p.id().to_string(),
                user: p.user().map(|u| GmPlayerUserInfo {
                    id: u.id().to_string(),
                    name: u.name().to_string(),
                }),
                stats: p.stats().iter().map(|s| GmStatInfo {
                    id: s.id().to_string(),
                    key: s.key().as_str().to_string(),
                    value_type: s.kind_str().to_string(),
                    string_value: s.as_string().map(|s| s.to_string()),
                    number_value: s.as_number(),
                    boolean_value: s.as_boolean(),
                }).collect(),
            }).collect(),
            round_number: value.round_number,
            current_player_index: value.current_player_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod into_gm_game_info {
        use crate::domain::game::entities::stat::Stat;
        use crate::domain::user::entities::User;
        use super::*;

        #[test]
        fn test_converts_game_to_gm_game_info() {
            let game_id = Uuid::new_v4();
            let player_id = Uuid::new_v4();
            let user_id = Uuid::new_v4();
            
            let mut game = Game::new(game_id, "test-game".to_string());
            
            let mut player = Player::new(player_id);
            player.add_user(User::try_new(user_id, "user1".to_string(), "password".to_string()).unwrap());
            
            let stat_id = Uuid::new_v4();
            player.try_add_stat(Stat::try_new_number_stat(
                stat_id,
                "health".to_string(),
                100.0,
            ).unwrap()).unwrap();
            
            //game.add_player(player).unwrap();

            let gm_info: GmGameInfo = GmGameInfo::from(&game);

            assert_eq!(gm_info.id, game_id.to_string());
            assert_eq!(gm_info.name, "test-game");
            // assert_eq!(gm_info.players.len(), 1);
            // assert_eq!(gm_info.players[0].id, player_id.to_string());
            // assert_eq!(gm_info.players[0].user.as_ref().unwrap().name, "user1");
            // assert_eq!(gm_info.players[0].stats.len(), 1);
            // assert_eq!(gm_info.players[0].stats[0].id, stat_id.to_string());
            // assert_eq!(gm_info.players[0].stats[0].key, "health");
            // assert_eq!(gm_info.players[0].stats[0].number_value, Some(100.0));
            assert_eq!(gm_info.round_number, 0);
            assert_eq!(gm_info.current_player_index, 0);
        }
    }
}