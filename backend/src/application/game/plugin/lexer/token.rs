#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    If,
    Else,
    ElseIf,
    Match,
    While,
    Do,
    For,
    Break,
    Continue,
    Return,
    Reject,
    Throw,
    Exec,
    Catch,
    Fn,

    // Types
    IntType,
    FloatType,
    StringType,
    BoolType,
    ArrayType(Box<Token>),

    // Object Types
    IdType,
    GameType,
    PlayerType,
    StatType,
    TradableType,

    // Literals
    Identifier(String),
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    BoolValue(bool),

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