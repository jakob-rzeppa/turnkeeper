use uuid::Uuid;
use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};

/// The representation of the game
///
/// # Creation
///
/// - For a new Game use `Game::new(id: Uuid)`.
/// - When instantiating an existing Game using `Game::builder()` is recommended.
///
/// # Invalid States
///
/// - Two Players have the same ID
/// - current_player_index is greater than length of players - 1
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

    pub fn id(&self) -> Uuid {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn add_player(&mut self, player: Player)-> Result<(), GameError> {
        if self.players.iter().any(|p| {
            p.name() == player.name()
        }) {
            return Err(GameError::new(GameErrorKind::PlayerWithSameNameAlreadyExists));
        }

        self.players.push(player);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add_player {
        use crate::domain::user::entities::User;
        use super::*;

        #[test]
        fn test_add_player() {
            let mut game = Game::new(Uuid::new_v4(), "test-game-name".to_string());

            let player = Player::new(
                Uuid::new_v4(),
                User::try_new(Uuid::new_v4(), "test-user-name".to_string(), "test-user-password".to_string()).unwrap(),
            );

            let res = game.add_player(player);

            assert!(res.is_ok());

            assert_eq!(game.players().len(), 1);
            assert_eq!(game.players()[0].name(), "test-user-name".to_string());
        }

        #[test]
        fn test_add_player_duplicate() {
            let mut game = Game::new(Uuid::new_v4(), "test-game-name".to_string());

            let player = Player::new(
                Uuid::new_v4(),
                User::try_new(Uuid::new_v4(), "test-user-name".to_string(), "test-user-password".to_string()).unwrap(),
            );
            let player2 = Player::new(
                Uuid::new_v4(),
                User::try_new(Uuid::new_v4(), "test-user-name".to_string(), "test-user-password".to_string()).unwrap(),
            );

            let res = game.add_player(player);
            assert!(res.is_ok());

            let res = game.add_player(player2);
            assert!(res.is_err());

            assert_eq!(game.players().len(), 1);
            assert_eq!(game.players()[0].name(), "test-user-name".to_string());
        }
    }
}