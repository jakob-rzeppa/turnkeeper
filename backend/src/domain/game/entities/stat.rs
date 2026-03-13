//! # Stat Entity
//!
//! Represents customizable stats attached to players in a game.

use crate::domain::game::error::{GameError};
use crate::domain::game::value_objects::id::Id;
use crate::domain::game::value_objects::stat_key::StatKey;
use crate::domain::game::value_objects::stat_value::{BooleanStatValue, NumberStatValue, StringStatValue};

/// A customizable stat for a player.
///
/// Stats can be one of three types:
/// - **Number**: Numeric values (e.g., health = 100)
/// - **String**: Text values (e.g., class = "Warrior")
/// - **Boolean**: True/false flags (e.g., is_active = true)
///
/// Use the factory methods to create stats:
/// - [`Stat::try_new_string_stat`]
/// - [`Stat::try_new_number_stat`]
/// - [`Stat::try_new_bool_stat`]
#[derive(Debug, Clone, PartialEq)]
pub struct Stat {
    id: Id,
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
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn key(&self) -> &StatKey {
        &self.key
    }

    pub fn as_number(&self) -> Option<f64> {
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

    pub fn try_new_string_stat(id: Id, key: String, value: String) -> Result<Self, GameError> {
        let key = StatKey::try_new(key)?;
        let value = StringStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::String { value }
        })
    }

    pub fn try_new_number_stat(id: Id, key: String, value: f64) -> Result<Self, GameError> {
        let key = StatKey::try_new(key)?;
        let value = NumberStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Number { value }
        })
    }

    pub fn try_new_bool_stat(id: Id, key: String, value: bool) -> Result<Self, GameError> {
        let key = StatKey::try_new(key)?;
        let value = BooleanStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Boolean { value }
        })
    }

    pub fn change_value_string(&mut self, new_value: String) -> Result<(), GameError> {
        self.kind = StatKind::String {
            value: StringStatValue::new(new_value),
        };
        Ok(())
    }

    pub fn change_value_number(&mut self, new_value: f64) -> Result<(), GameError> {
        self.kind = StatKind::Number {
            value: NumberStatValue::new(new_value),
        };
        Ok(())
    }

    pub fn change_value_boolean(&mut self, new_value: bool) -> Result<(), GameError> {
        self.kind = StatKind::Boolean {
            value: BooleanStatValue::new(new_value),
        };
        Ok(())
    }
}