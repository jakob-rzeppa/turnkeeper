#[derive(Debug, Clone, PartialEq)]
pub struct BooleanStatValue {
    value: bool,
}

impl BooleanStatValue {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberStatValue {
    value: i64,
}

impl NumberStatValue {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringStatValue {
    value: String,
}

impl StringStatValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}