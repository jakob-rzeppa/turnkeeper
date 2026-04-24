#[derive(Debug, Clone, PartialEq)]
pub enum StatValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}
