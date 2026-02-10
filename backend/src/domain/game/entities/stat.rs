use uuid::Uuid;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::value_objects::stat_key::StatKey;
use crate::domain::game::value_objects::stat_value::{BooleanStatValue, NumberStatValue, StringStatValue};

/// The representation of a stat
///
/// Stat kinds are
///
/// 1. Number
/// 2. String
/// 3. Boolean
///
/// Use `try_new_string_stat`, `new_number_stat` or `new_bool_stat` for instantiating the Stat.
#[derive(Debug, Clone, PartialEq)]
pub struct Stat {
    id: Uuid,
    key: StatKey,
    kind: StatKind,
}

#[derive(Debug, Clone, PartialEq)]
enum StatKind {
    Number {
        value: NumberStatValue,
    },
    String {
        value: StringStatValue,
    },
    Boolean {
        value: BooleanStatValue,
    },
}

impl Stat {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn key(&self) -> &StatKey {
        &self.key
    }

    pub fn as_number(&self) -> Option<i64> {
        match &self.kind {
            StatKind::Number { value } => Some(value.value()),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match &self.kind {
            StatKind::String { value } => Some(value.value()),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match &self.kind {
            StatKind::Boolean { value } => Some(value.value()),
            _ => None,
        }
    }

    pub fn kind_str(&self) -> &str {
        match self.kind {
            StatKind::Number { .. } => "number",
            StatKind::String { .. } => "string",
            StatKind::Boolean { .. } => "boolean",
        }
    }

    pub fn try_new_string_stat(id: Uuid, key: String, value: String) -> Result<Self, GameError> {
        let key = StatKey::try_new(key).map_err(|e| { GameError::with_source(GameErrorKind::InvalidStat, Box::new(e)) })?;
        let value = StringStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::String { value }
        })
    }

    pub fn try_new_number_stat(id: Uuid, key: String, value: i64) -> Result<Self, GameError> {
        let key = StatKey::try_new(key).map_err(|e| { GameError::with_source(GameErrorKind::InvalidStat, Box::new(e)) })?;
        let value = NumberStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Number { value }
        })
    }

    pub fn try_new_bool_stat(id: Uuid, key: String, value: bool) -> Result<Self, GameError> {
        let key = StatKey::try_new(key).map_err(|e| { GameError::with_source(GameErrorKind::InvalidStat, Box::new(e)) })?;
        let value = BooleanStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Boolean { value }
        })
    }
}