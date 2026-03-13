use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::game::error::{GameError, GameErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    id: Uuid,
}

impl Id {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }

    pub fn parse_str(s: &str) -> Result<Self, GameError> {
        Ok(Self {
            id: Uuid::parse_str(s).map_err(|_| GameError::new(GameErrorKind::InvalidUuid))?
        })
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Self::parse_str(&id).expect("Could not convert value to id")
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}