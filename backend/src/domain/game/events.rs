use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    AddPlayer,
    Debug(String),
}

impl GameEvent {
    pub fn is_user_permitted(&self) -> bool {
        match self {
            GameEvent::AddPlayer => false,
            GameEvent::Debug(_) => true,
        }
    }
}