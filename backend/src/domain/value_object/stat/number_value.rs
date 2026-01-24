#[derive(Debug, Clone, PartialEq)]
pub struct NumberValue {
    value: i64,
}

impl NumberValue {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}