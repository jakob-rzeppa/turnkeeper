use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    Debug(String),
}

impl GameEvent {
    pub fn is_user_permitted(&self) -> bool {
        match self {
            GameEvent::Debug(_) => true,
        }
    }
}