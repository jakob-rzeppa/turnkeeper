use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::value_object::stat::key::Key;
use crate::domain::value_object::stat::boolean_value::BooleanValue;
use crate::domain::value_object::stat::number_value::NumberValue;
use crate::domain::value_object::stat::string_value::StringValue;

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
    key: Key,
    kind: StatKind,
}

#[derive(Debug, Clone, PartialEq)]
enum StatKind {
    Number {
        value: NumberValue,
    },
    String {
        value: StringValue,
    },
    Boolean {
        value: BooleanValue,
    },
}

impl Stat {
    pub fn try_new_string_stat(id: Uuid, key: String, value: String) -> Result<Self, Error> {
        let key = Key::try_new(key).map_err(|e| e.prefix("new string stat".to_string()))?;
        let value = StringValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::String { value }
        })
    }

    pub fn try_new_number_stat(id: Uuid, key: String, value: i64) -> Result<Self, Error> {
        let key = Key::try_new(key).map_err(|e| e.prefix("new number stat".to_string()))?;
        let value = NumberValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Number { value }
        })
    }

    pub fn try_new_bool_stat(id: Uuid, key: String, value: bool) -> Result<Self, Error> {
        let key = Key::try_new(key).map_err(|e| e.prefix("new bool stat".to_string()))?;
        let value = BooleanValue::new(value);

        Ok(Self {
            id,
            key,
            kind: StatKind::Boolean { value }
        })
    }
}