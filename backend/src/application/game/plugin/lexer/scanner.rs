
pub enum Lexeme {
    Identifier(String),
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    BoolValue(bool),
    Keyword(String),
    Operator(String),
    AssignmentOperator(String),
    Type(String),
    Eof,
}