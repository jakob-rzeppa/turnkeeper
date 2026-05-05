use crate::domain::common::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub lexeme: LexemeVariant,
    pub pos: Position,
}

impl Lexeme {
    pub fn new(lexeme: LexemeVariant, pos: Position) -> Self {
        Lexeme { lexeme, pos }
    }

    pub fn char_is_symbol(char: char) -> bool {
        let symbols = [
            '=', '+', '-', '*', '/', '%', '^', '!', '&', '|', '<', '>', ';', '(', ')', '{', '}',
            '[', ']', ',', '.', ':', '?',
        ];
        symbols.contains(&char)
    }

    pub fn chars_are_double_symbol(first: char, second: char) -> bool {
        let double_symbols = [
            "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "^=", "=>", "->",
        ];
        let candidate = format!("{}{}", first, second);
        double_symbols.contains(&candidate.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexemeVariant {
    Text(String),
    Number(String),
    DecimalNumber(String),
    Quote(String),
    Symbol(String),
    DoubleSymbol(String),
}
