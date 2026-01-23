use uuid::Uuid;
use crate::domain::value_object::identifier::Identifier;
use crate::domain::value_object::stat::key::Key;
use crate::domain::value_object::stat::boolean_value::BooleanValue;
use crate::domain::value_object::stat::number_value::NumberValue;
use crate::domain::value_object::stat::string_value::StringValue;
use crate::error::DomainError;

/// The representation of a stat
/// 
/// Stat kinds are
///
/// 1. Number
/// 2. String
/// 3. Boolean
/// 
/// Use `try_new_string_stat`, `new_number_stat` or `new_bool_stat` for instantiating the Stat.
pub struct Stat {
    kind: StatKind,
}

enum StatKind {
    Number {
        id: Identifier,
        key: Key,
        value: NumberValue,
    },
    String {
        id: Identifier,
        key: Key,
        value: StringValue,
    },
    Boolean {
        id: Identifier,
        key: Key,
        value: BooleanValue,
    },
}

impl Stat {
    pub fn try_new_string_stat(id: Uuid, key: String, value: String) -> Result<Self, DomainError> {
        let id = Identifier::new(id);
        let key = Key::try_new(key)?;
        let value = StringValue::new(value);

        Ok(Self {
            kind: StatKind::String { id, key, value }
        })
    }

    pub fn new_number_stat(id: Uuid, key: String, value: i64) -> Result<Self, DomainError> {
        let id = Identifier::new(id);
        let key = Key::try_new(key)?;
        let value = NumberValue::new(value);

        Ok(Self {
            kind: StatKind::Number { id, key, value }
        })
    }

    pub fn new_bool_stat(id: Uuid, key: String, value: bool) -> Result<Self, DomainError> {
        let id = Identifier::new(id);
        let key = Key::try_new(key)?;
        let value = BooleanValue::new(value);

        Ok(Self {
            kind: StatKind::Boolean { id, key, value }
        })
    }
}