use crate::application::interpreter::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub lexeme: LexemeType,
    pub pos: Position,
}

impl Lexeme {
    pub fn new(lexeme: LexemeType, pos: Position) -> Self {
        Lexeme { lexeme, pos }
    }

    pub fn char_is_symbol(char: char) -> bool {
        let symbols = [
            '=', '+', '-', '*', '/', '%', '^', '!', '&', '|', '<', '>', ';', '(', ')', '{', '}',
            '[', ']', ',', '.', ':', '?',
        ];
        symbols.contains(&char)
    }

    pub fn chars_are_double_symbol(first: &str, second: char) -> bool {
        let double_symbols = [
            "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "^=", "=>", "->",
        ];
        let candidate = format!("{}{}", first, second);
        double_symbols.contains(&candidate.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexemeType {
    Text(String),
    Number(String),
    NumberWithDot(String),
    Quote(String),
    Symbol(String),
    DoubleSymbol(String),
}
