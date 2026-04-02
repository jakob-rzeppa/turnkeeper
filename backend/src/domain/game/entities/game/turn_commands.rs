use super::Game;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::value_objects::id::Id;

impl Game {
    pub fn next_turn(&mut self) {
        if self.players.is_empty() {
            return; // No players, do nothing
        }

        self.current_player_index += 1;
        if self.current_player_index >= self.players.len() {
            self.current_player_index = 0;
            self.round_number += 1;
        }
    }

    pub fn prev_turn(&mut self) {
        if self.players.is_empty() {
            return; // No players, do nothing
        }

        if self.current_player_index == 0 {
            if self.round_number > 0 {
                self.current_player_index = self.players.len() - 1;
                self.round_number -= 1;
            }
        } else {
            self.current_player_index -= 1;
        }
    }

    pub fn skip_turn_to_player(&mut self, player_id: Id) -> Result<(), GameError> {
        let new_index = self
            .players
            .iter()
            .position(|p| p.id() == &player_id)
            .ok_or_else(|| GameError::new(GameErrorKind::PlayerNotFound))?;

        if new_index < self.current_player_index {
            self.round_number += 1;
        }

        self.current_player_index = new_index;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_next_turn() {
        let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
        let player_id_1 = Id::new();
        game.add_player(player_id_1).unwrap();
        let player_id_2 = Id::new();
        game.add_player(player_id_2).unwrap();
        let player_id_3 = Id::new();
        game.add_player(player_id_3).unwrap();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 0);

        game.next_turn();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 1);

        game.next_turn();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 2);

        game.next_turn();

        assert_eq!(game.round_number, 1);
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    fn text_prev_turn() {
        let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
        let player_id_1 = Id::new();
        game.add_player(player_id_1).unwrap();
        let player_id_2 = Id::new();
        game.add_player(player_id_2).unwrap();
        let player_id_3 = Id::new();
        game.add_player(player_id_3).unwrap();

        game.next_turn();
        game.next_turn();
        game.next_turn();
        game.next_turn();

        assert_eq!(game.round_number, 1);
        assert_eq!(game.current_player_index, 1);

        game.prev_turn();

        assert_eq!(game.round_number, 1);
        assert_eq!(game.current_player_index, 0);

        game.prev_turn();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 2);

        game.prev_turn();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 1);

        game.prev_turn();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    fn text_skip_turn_to_player() {
        let mut game = Game::new(Id::new(), "test-game".to_string(), Id::new());
        let player_id_1 = Id::new();
        game.add_player(player_id_1).unwrap();
        let player_id_2 = Id::new();
        game.add_player(player_id_2).unwrap();
        let player_id_3 = Id::new();
        game.add_player(player_id_3).unwrap();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 0);

        game.skip_turn_to_player(player_id_3).unwrap();

        assert_eq!(game.round_number, 0);
        assert_eq!(game.current_player_index, 2);

        game.skip_turn_to_player(player_id_1).unwrap();

        assert_eq!(game.round_number, 1);
        assert_eq!(game.current_player_index, 0);
    }
}
