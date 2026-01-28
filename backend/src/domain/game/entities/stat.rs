use uuid::Uuid;
use crate::domain::error::Error;
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

    pub fn try_new_string_stat(id: Uuid, key: String, value: String) -> Result<Self, Error> {
        let key = StatKey::try_new(key).map_err(|e| e.prefix("new string stat".to_string()))?;
        let value = StringStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::String { value }
        })
    }

    pub fn try_new_number_stat(id: Uuid, key: String, value: i64) -> Result<Self, Error> {
        let key = StatKey::try_new(key).map_err(|e| e.prefix("new number stat".to_string()))?;
        let value = NumberStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Number { value }
        })
    }

    pub fn try_new_bool_stat(id: Uuid, key: String, value: bool) -> Result<Self, Error> {
        let key = StatKey::try_new(key).map_err(|e| e.prefix("new bool stat".to_string()))?;
        let value = BooleanStatValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Boolean { value }
        })
    }
}