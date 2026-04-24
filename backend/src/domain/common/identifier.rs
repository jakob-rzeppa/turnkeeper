use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse UUID: {0}")]
pub struct UuidParseError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier {
    id: Uuid,
}

impl Identifier {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }

    pub fn parse_str(s: &str) -> Result<Self, UuidParseError> {
        Ok(Self {
            id: Uuid::parse_str(s).map_err(|_| UuidParseError(s.to_string()))?,
        })
    }
}

impl From<String> for Identifier {
    fn from(id: String) -> Self {
        Self::parse_str(&id).expect("Could not convert value to id")
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}
