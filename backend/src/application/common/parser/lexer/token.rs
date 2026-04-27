use std::fmt::Display;

use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::lexeme::{Lexeme, LexemeVariant},
    },
    domain::common::position::Position,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub variant: TokenVariant,
    pub pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenVariant {
    // --- FROM Text LEXEME ---
    Hidden,
    Private,
    Protected,
    Public,

    Stat,
    Pstat,
    Action,

    After,
    Before,
    TurnAdvance,
    RoundAdvance,

    IntType,
    FloatType,
    StringType,
    BoolType,

    BoolLiteral(bool),

    Identifier(String),

    // --- FROM Symbol and DoubleSymbol LEXEME ---
    Colon,        // :
    Equal,        // =
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Comma,        // ,
    Dot,          // .
    Semicolon,    // ;
    Pipe,         // |
    Plus,         // +
    Minus,        // -
    Asterisk,     // *
    Slash,        // /
    Percent,      // %

    // --- FROM Number and DecimalNumber LEXEME ---
    IntLiteral(i64),
    FloatLiteral(f64),

    // --- FROM Quote LEXEME ---
    StringLiteral(String),
}

impl TryFrom<Lexeme> for Token {
    type Error = ParsingError;

    fn try_from(lexeme: Lexeme) -> Result<Self, ParsingError> {
        let variant = match lexeme.lexeme {
            LexemeVariant::Text(text) => match text.as_str() {
                "hidden" => TokenVariant::Hidden,
                "private" => TokenVariant::Private,
                "protected" => TokenVariant::Protected,
                "public" => TokenVariant::Public,
                "stat" => TokenVariant::Stat,
                "pstat" => TokenVariant::Pstat,
                "action" => TokenVariant::Action,
                "after" => TokenVariant::After,
                "before" => TokenVariant::Before,
                "TurnAdvance" => TokenVariant::TurnAdvance,
                "RoundAdvance" => TokenVariant::RoundAdvance,
                "int" => TokenVariant::IntType,
                "float" => TokenVariant::FloatType,
                "string" => TokenVariant::StringType,
                "bool" => TokenVariant::BoolType,
                "true" => TokenVariant::BoolLiteral(true),
                "false" => TokenVariant::BoolLiteral(false),
                _ => TokenVariant::Identifier(text),
            },
            LexemeVariant::Symbol(symbol) => match symbol.as_str() {
                ":" => TokenVariant::Colon,
                "=" => TokenVariant::Equal,
                "(" => TokenVariant::OpenParen,
                ")" => TokenVariant::CloseParen,
                "{" => TokenVariant::OpenBrace,
                "}" => TokenVariant::CloseBrace,
                "[" => TokenVariant::OpenBracket,
                "]" => TokenVariant::CloseBracket,
                "," => TokenVariant::Comma,
                "." => TokenVariant::Dot,
                ";" => TokenVariant::Semicolon,
                "|" => TokenVariant::Pipe,
                "+" => TokenVariant::Plus,
                "-" => TokenVariant::Minus,
                "*" => TokenVariant::Asterisk,
                "/" => TokenVariant::Slash,
                "%" => TokenVariant::Percent,
                _ => {
                    return Err(ParsingError::InvalidToken {
                        pos: lexeme.pos,
                        message: format!("Unexpected symbol lexeme {}", symbol),
                    });
                }
            },
            LexemeVariant::DoubleSymbol(double_symbol) => {
                return Err(ParsingError::InvalidToken {
                    pos: lexeme.pos,
                    message: format!("Unexpected double symbol lexeme {}", double_symbol),
                });
            }
            LexemeVariant::Number(num) => {
                TokenVariant::IntLiteral(num.parse::<i64>().map_err(|_| {
                    ParsingError::InvalidToken {
                        pos: lexeme.pos.clone(),
                        message: format!("Invalid integer literal {}", num),
                    }
                })?)
            }
            LexemeVariant::DecimalNumber(num) => {
                TokenVariant::FloatLiteral(num.parse::<f64>().map_err(|_| {
                    ParsingError::InvalidToken {
                        pos: lexeme.pos.clone(),
                        message: format!("Invalid float literal {}", num),
                    }
                })?)
            }
            LexemeVariant::Quote(quote) => TokenVariant::StringLiteral(quote),
        };

        Ok(Token {
            variant,
            pos: lexeme.pos,
        })
    }
}

impl Display for TokenVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenVariant::Hidden => write!(f, "hidden"),
            TokenVariant::Private => write!(f, "private"),
            TokenVariant::Protected => write!(f, "protected"),
            TokenVariant::Public => write!(f, "public"),
            TokenVariant::Stat => write!(f, "stat"),
            TokenVariant::Pstat => write!(f, "pstat"),
            TokenVariant::Action => write!(f, "action"),
            TokenVariant::After => write!(f, "after"),
            TokenVariant::Before => write!(f, "before"),
            TokenVariant::TurnAdvance => write!(f, "TurnAdvance"),
            TokenVariant::RoundAdvance => write!(f, "RoundAdvance"),
            TokenVariant::Identifier(id) => write!(f, "Identifier({})", id),
            TokenVariant::Colon => write!(f, ":"),
            TokenVariant::Equal => write!(f, "="),
            TokenVariant::OpenParen => write!(f, "("),
            TokenVariant::CloseParen => write!(f, ")"),
            TokenVariant::OpenBrace => write!(f, "{{"),
            TokenVariant::CloseBrace => write!(f, "}}"),
            TokenVariant::OpenBracket => write!(f, "["),
            TokenVariant::CloseBracket => write!(f, "]"),
            TokenVariant::Comma => write!(f, ","),
            TokenVariant::Semicolon => write!(f, ";"),
            TokenVariant::Dot => write!(f, "."),
            TokenVariant::IntLiteral(num) => write!(f, "IntLiteral({})", num),
            TokenVariant::FloatLiteral(num) => write!(f, "FloatLiteral({})", num),
            TokenVariant::StringLiteral(s) => write!(f, "StringLiteral({})", s),
            TokenVariant::BoolLiteral(b) => write!(f, "BoolLiteral({})", b),
            TokenVariant::IntType => write!(f, "int"),
            TokenVariant::FloatType => write!(f, "float"),
            TokenVariant::StringType => write!(f, "string"),
            TokenVariant::BoolType => write!(f, "bool"),
            TokenVariant::Pipe => write!(f, "|"),
            TokenVariant::Plus => write!(f, "+"),
            TokenVariant::Minus => write!(f, "-"),
            TokenVariant::Asterisk => write!(f, "*"),
            TokenVariant::Slash => write!(f, "/"),
            TokenVariant::Percent => write!(f, "%"),
        }
    }
}
