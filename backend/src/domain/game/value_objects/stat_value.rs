#[derive(Debug, Clone, PartialEq)]
pub struct BooleanStatValue {
    value: bool,
}

impl BooleanStatValue {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
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

    pub fn value(&self) -> i64 {
        self.value
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

    pub fn value(&self) -> &str {
        &self.value
    }
}