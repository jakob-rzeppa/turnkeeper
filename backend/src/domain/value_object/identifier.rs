use uuid::Uuid;

pub struct Identifier {
    value: Uuid,
}

impl Identifier {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }
}