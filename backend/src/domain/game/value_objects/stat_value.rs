#[derive(Debug, Clone, PartialEq)]
pub enum StatValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}
