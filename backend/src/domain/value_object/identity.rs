use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Identity {
    value: Uuid,
}

impl Identity {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}