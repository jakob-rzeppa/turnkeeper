use crate::application::game::plugin::common::Position;



#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub pos: Position,
}

impl Token {
    pub fn new(token: TokenType, pos: Position) -> Self {
        Token {
            token,
            pos,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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
    ArrayType(Box<TokenType>),

    Identifier(String),
    BoolLiteral(bool),

    // --- FROM Number, NumberWithDot LEXEME ---
    IntLiteral(i64),
    FloatLiteral(f64),

    // --- FROM Quote LEXEME ---
    StringLiteral(String),

    // --- FROM Symbol and DoubleSymbol LEXEME ---
    // Assignment Operators
    Assign,                 // =
    AddAssign,              // +=
    SubAssign,              // -=
    MulAssign,              // *=
    DivAssign,              // /=
    ModAssign,              // %=
    PowAssign,              // ^=

    // Operators
    Plus,                   // +
    Minus,                  // -
    Star,                   // *
    Slash,                  // /
    Percent,                // %
    Caret,                  // ^
    EqualEqual,             // ==
    NotEqual,               // !=
    Less,                   // <
    Greater,                // >
    LessEqual,              // <=
    GreaterEqual,           // >=
    And,                    // &&
    Or,                     // ||
    Not,                    // !

    // Delimiters
    LeftParen,              // (
    RightParen,             // )
    LeftBrace,              // {
    RightBrace,             // }
    LeftBracket,            // [
    RightBracket,           // ]
    Semicolon,              // ;
    Colon,                  // :
    Comma,                  // ,
    Pipe,                   // |
    Underscore,             // _
    ThickArrow,             // =>
    ThinArrow,              // ->

    // Special
    Question,               // ?
}