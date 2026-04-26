use std::fmt::Display;

use crate::{
    application::{
        common::parser::lexeme::{Lexeme, LexemeVariant},
        game::root_parser::error::GameParsingError,
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
    On,
    TurnStart,
    TurnEnd,
    RoundStart,
    RoundEnd,

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
    Semicolon,    // ;

    // --- FROM Number and DecimalNumber LEXEME ---
    IntLiteral(i64),
    FloatLiteral(f64),

    // --- FROM Quote LEXEME ---
    StringLiteral(String),
}

impl TryFrom<Lexeme> for Token {
    type Error = GameParsingError;

    fn try_from(lexeme: Lexeme) -> Result<Self, Self::Error> {
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
                "on" => TokenVariant::On,
                "turn_start" => TokenVariant::TurnStart,
                "turn_end" => TokenVariant::TurnEnd,
                "round_start" => TokenVariant::RoundStart,
                "round_end" => TokenVariant::RoundEnd,
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
                ";" => TokenVariant::Semicolon,
                _ => {
                    return Err(GameParsingError::InvalidToken {
                        pos: lexeme.pos,
                        message: format!("Unexpected symbol lexeme {}", symbol),
                    });
                }
            },
            LexemeVariant::DoubleSymbol(double_symbol) => {
                return Err(GameParsingError::InvalidToken {
                    pos: lexeme.pos,
                    message: format!("Unexpected double symbol lexeme {}", double_symbol),
                });
            }
            LexemeVariant::Number(num) => {
                TokenVariant::IntLiteral(num.parse::<i64>().map_err(|_| {
                    GameParsingError::InvalidToken {
                        pos: lexeme.pos.clone(),
                        message: format!("Invalid integer literal {}", num),
                    }
                })?)
            }
            LexemeVariant::DecimalNumber(num) => {
                TokenVariant::FloatLiteral(num.parse::<f64>().map_err(|_| {
                    GameParsingError::InvalidToken {
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
            TokenVariant::On => write!(f, "on"),
            TokenVariant::TurnStart => write!(f, "TurnStart"),
            TokenVariant::TurnEnd => write!(f, "TurnEnd"),
            TokenVariant::RoundStart => write!(f, "RoundStart"),
            TokenVariant::RoundEnd => write!(f, "RoundEnd"),
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
            TokenVariant::IntLiteral(num) => write!(f, "IntLiteral({})", num),
            TokenVariant::FloatLiteral(num) => write!(f, "FloatLiteral({})", num),
            TokenVariant::StringLiteral(s) => write!(f, "StringLiteral({})", s),
            TokenVariant::BoolLiteral(b) => write!(f, "BoolLiteral({})", b),
            TokenVariant::IntType => write!(f, "int"),
            TokenVariant::FloatType => write!(f, "float"),
            TokenVariant::StringType => write!(f, "string"),
            TokenVariant::BoolType => write!(f, "bool"),
        }
    }
}
