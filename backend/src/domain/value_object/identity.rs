use uuid::Uuid;
use crate::domain::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Identity {
    value: Uuid,
}

impl Identity {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl From<Uuid> for Identity {
    fn from(value: Uuid) -> Self {
        Self { value }
    }
}

impl TryFrom<String> for Identity {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            value: Uuid::parse_str(&value).map_err(|e| Error::InvalidState { msg: "Invalid UUID".to_string() })?,
        })
    }
}