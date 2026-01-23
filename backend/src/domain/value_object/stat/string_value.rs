pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}