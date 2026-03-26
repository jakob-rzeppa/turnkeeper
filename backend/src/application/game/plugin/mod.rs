pub mod lexer;
pub mod parser;
pub mod runtime;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub first_char: usize,
}