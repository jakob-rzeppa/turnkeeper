use std::fmt::Display;

use crate::application::interpreter::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub variant: TokenVariant,
    pub pos: Position,
}

impl Token {
    pub fn new(token: TokenVariant, pos: Position) -> Self {
        Token {
            variant: token,
            pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenVariant {
    // --- FROM Text LEXEME ---
    // Keywords
    Let,
    If,
    Else,
    Match,
    While,
    Do,
    For,
    Break,
    Continue,
    Return,
    Reject,
    Throw,
    Exit,
    Exec,
    Catch,
    Fn,

    // Types
    IntType,
    FloatType,
    StringType,
    BoolType,
    ArrayType(Box<TokenVariant>),

    Identifier(String),
    BoolLiteral(bool),

    // --- FROM Number, NumberWithDot LEXEME ---
    IntLiteral(i64),
    FloatLiteral(f64),

    // --- FROM Quote LEXEME ---
    StringLiteral(String),

    // --- FROM Symbol and DoubleSymbol LEXEME ---
    // Assignment Operators
    Assign,    // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    PowAssign, // ^=

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Caret,        // ^
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
    Not,          // !

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Pipe,         // |
    Underscore,   // _
    ThickArrow,   // =>
    ThinArrow,    // ->

    // Special
    Question, // ?
}

impl Display for TokenVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_str = match self {
            TokenVariant::Let => "let".to_string(),
            TokenVariant::If => "if".to_string(),
            TokenVariant::Else => "else".to_string(),
            TokenVariant::Match => "match".to_string(),
            TokenVariant::While => "while".to_string(),
            TokenVariant::Do => "do".to_string(),
            TokenVariant::For => "for".to_string(),
            TokenVariant::Break => "break".to_string(),
            TokenVariant::Continue => "continue".to_string(),
            TokenVariant::Return => "return".to_string(),
            TokenVariant::Reject => "reject".to_string(),
            TokenVariant::Throw => "throw".to_string(),
            TokenVariant::Exit => "exit".to_string(),
            TokenVariant::Exec => "exec".to_string(),
            TokenVariant::Catch => "catch".to_string(),
            TokenVariant::Fn => "fn".to_string(),

            // Types
            TokenVariant::IntType => "int".to_string(),
            TokenVariant::FloatType => "float".to_string(),
            TokenVariant::StringType => "string".to_string(),
            TokenVariant::BoolType => "bool".to_string(),

            // Literals and identifiers
            TokenVariant::Identifier(name) => name.clone(),
            TokenVariant::BoolLiteral(value) => value.to_string(),
            TokenVariant::IntLiteral(value) => value.to_string(),
            TokenVariant::FloatLiteral(value) => value.to_string(),
            TokenVariant::StringLiteral(value) => format!("\"{}\"", value),

            // Operators and delimiters
            TokenVariant::Assign => "=".to_string(),
            TokenVariant::AddAssign => "+=".to_string(),
            TokenVariant::SubAssign => "-=".to_string(),
            TokenVariant::MulAssign => "*=".to_string(),
            TokenVariant::DivAssign => "/=".to_string(),
            TokenVariant::ModAssign => "%=".to_string(),
            TokenVariant::PowAssign => "^=".to_string(),
            TokenVariant::Plus => "+".to_string(),
            TokenVariant::Minus => "-".to_string(),
            TokenVariant::Star => "*".to_string(),
            TokenVariant::Slash => "/".to_string(),
            TokenVariant::Percent => "%".to_string(),
            TokenVariant::Caret => "^".to_string(),
            TokenVariant::EqualEqual => "==".to_string(),
            TokenVariant::NotEqual => "!=".to_string(),
            TokenVariant::Less => "<".to_string(),
            TokenVariant::Greater => ">".to_string(),
            TokenVariant::LessEqual => "<=".to_string(),
            TokenVariant::GreaterEqual => ">=".to_string(),
            TokenVariant::And => "&&".to_string(),
            TokenVariant::Or => "||".to_string(),
            TokenVariant::Not => "!".to_string(),
            TokenVariant::LeftParen => "(".to_string(),
            TokenVariant::RightParen => ")".to_string(),
            TokenVariant::LeftBrace => "{".to_string(),
            TokenVariant::RightBrace => "}".to_string(),
            TokenVariant::LeftBracket => "[".to_string(),
            TokenVariant::RightBracket => "]".to_string(),
            TokenVariant::Semicolon => ";".to_string(),
            TokenVariant::Colon => ":".to_string(),
            TokenVariant::Comma => ",".to_string(),
            TokenVariant::Pipe => "|".to_string(),
            TokenVariant::Underscore => "_".to_string(),
            TokenVariant::ThickArrow => "=>".to_string(),
            TokenVariant::ThinArrow => "->".to_string(),
            TokenVariant::Question => "?".to_string(),
            TokenVariant::ArrayType(inner) => format!("{}[]", inner),
        };
        write!(f, "{}", variant_str)
    }
}
