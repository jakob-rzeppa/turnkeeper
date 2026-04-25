#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StatValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}
