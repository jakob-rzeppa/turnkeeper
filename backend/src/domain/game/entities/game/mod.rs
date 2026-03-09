use std::str::FromStr;
use uuid::Uuid;
use crate::domain::game::entities::player::Player;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;

mod player_events;
mod notes_events;
mod player_stat_events;

/// The aggregate root representing a game.
///
/// # Invariants
///
/// - No two players have the same ID
/// - `current_player_index` does not exceed `players.len() - 1`
#[derive(Debug, PartialEq)]
pub struct Game {
    id: Uuid,
    name: String,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,

    notes: String,
    hidden_notes: String,
}

impl Game {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
            notes: String::new(),
            hidden_notes: String::new(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn round_number(&self) -> u32 {
        self.round_number
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn hidden_notes(&self) -> &str {
        &self.hidden_notes
    }

    /// Dispatches a [`GameEvent`] to the appropriate handler method.
    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), GameError> {
        match event {
            GameEvent::SetNotes(notes) => {
                self.set_notes(notes);
                Ok(())
            },
            GameEvent::SetHiddenNotes(hidden_notes) => {
                self.set_hidden_notes(hidden_notes);
                Ok(())
            },
            GameEvent::AddPlayer { player_id } => {
                self.add_player(Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?)
            },
            GameEvent::AddStatToPlayer { player_id, stat_key, stat_type, stat_value } => {
                self.add_stat_to_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    stat_key,
                    stat_type,
                    stat_value)
            },
            GameEvent::ChangeStatOfPlayer { player_id, stat_id, stat_type, stat_value } => {
                self.change_stat_of_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    Uuid::from_str(&stat_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    stat_type,
                    stat_value,
                )
            },
            GameEvent::RemoveStatFromPlayer { player_id, stat_id } => {
                self.remove_stat_from_player(
                    Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                    Uuid::from_str(&stat_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                )
            }
            GameEvent::AttachUserToPlayer { user_id, player_id } => self.attach_user_to_player(
                Uuid::from_str(&user_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
                Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
            ),
            GameEvent::DetachUserFromPlayer { player_id } => self.detach_user_from_player(
                Uuid::from_str(&player_id).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?,
            ),
            GameEvent::ChangePlayerOrder(ids_in_order) => {
                let ids_in_order: Vec<Uuid> = ids_in_order.into_iter()
                    .map(|s| Uuid::from_str(&s).map_err(|_| GameError::new(GameErrorKind::InvalidUuid)))
                    .collect::<Result<Vec<Uuid>, GameError>>()?;

                self.change_player_order(ids_in_order)
            },
            GameEvent::Debug(msg) => {
                println!("Debug event with message: {}", msg);
                Ok(())
            }
        }
    }
}
