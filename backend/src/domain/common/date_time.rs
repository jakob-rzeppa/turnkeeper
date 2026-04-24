use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    value: chrono::DateTime<chrono::Local>,
}

impl DateTime {
    pub fn now() -> Self {
        Self {
            value: chrono::Local::now(),
        }
    }

    pub fn parse_str(s: &str) -> Result<Self, chrono::ParseError> {
        let dt = chrono::DateTime::parse_from_rfc3339(s)?;
        Ok(Self {
            value: dt.with_timezone(&chrono::Local),
        })
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.to_rfc3339())
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_str(&s).map_err(serde::de::Error::custom)
    }
}
