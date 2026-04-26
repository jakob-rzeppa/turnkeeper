use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{
            token::{Token, TokenVariant},
            token_stream::TokenStream,
        },
        macros::{expect_token, get_pos, is_token, nth_is_token},
        parsable::Parsable,
    },
    domain::game::{
        entities::stat::GameStat,
        value_objects::{stat_value::StatValue, stat_visibility::GameStatVisibility},
    },
};

impl Parsable<Token> for GameStat {
    fn is_next(ts: &TokenStream<Token>) -> bool {
        is_token!(
            ts,
            TokenVariant::Hidden | TokenVariant::Private | TokenVariant::Public
        ) && nth_is_token!(ts, 1, TokenVariant::Stat)
    }

    fn parse(ts: &mut TokenStream<Token>) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let visibility = match ts.next() {
            Some(Token {
                variant: TokenVariant::Hidden,
                ..
            }) => GameStatVisibility::Hidden,
            Some(Token {
                variant: TokenVariant::Private,
                ..
            }) => GameStatVisibility::Private,
            Some(Token {
                variant: TokenVariant::Public,
                ..
            }) => GameStatVisibility::Public,
            Some(token) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected visibility modifier (hidden, private, public)".to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected visibility modifier (hidden, private, public)".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Stat,
            "Expected 'stat' keyword after visibility modifier in stat declaration"
        );

        let name = match ts.next() {
            Some(Token {
                variant: TokenVariant::Identifier(name),
                ..
            }) => name.clone(),
            Some(token) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected identifier for stat name".to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected identifier for stat name".to_string(),
                });
            }
        };

        // Optional: Check for a colon after the stat name, which could indicate a type declaration.
        let type_decl = if let Some(Token {
            variant: TokenVariant::Colon,
            ..
        }) = ts.peek()
        {
            ts.next(); // Consume the colon

            match ts.next() {
                Some(token) => {
                    if matches!(
                        token.variant,
                        TokenVariant::IntType
                            | TokenVariant::FloatType
                            | TokenVariant::StringType
                            | TokenVariant::BoolType
                    ) {
                        Some(token.variant.clone())
                    } else {
                        return Err(ParsingError::UnexpectedToken {
                            expected: "Expected type declaration (int, float, string, bool) after ':' in stat declaration".to_string(),
                            found: token.variant.clone(),
                            pos: token.pos,
                        });
                    }
                }
                None => {
                    return Err(ParsingError::UnexpectedEOF {
                        expected: "Expected type declaration (int, float, string, bool) after ':' in stat declaration".to_string(),
                    });
                }
            }
        } else {
            None
        };

        expect_token!(
            ts,
            TokenVariant::Equal,
            "Expected '=' after stat name (and optional type declaration) in stat declaration"
        );

        let value: StatValue = match ts.next() {
            Some(token) => match token.variant.clone() {
                TokenVariant::IntLiteral(num) => StatValue::Int(num),
                TokenVariant::FloatLiteral(num) => StatValue::Float(num),
                TokenVariant::StringLiteral(s) => StatValue::String(s),
                TokenVariant::BoolLiteral(b) => StatValue::Bool(b),
                _ => {
                    return Err(ParsingError::UnexpectedToken {
                        expected: "Expected literal value (int, float, string, bool) after '=' in stat declaration".to_string(),
                        found: token.variant.clone(),
                        pos: token.pos,
                    });
                }
            },
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected literal value (int, float, string, bool) after '=' in stat declaration".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "Expected ';' at the end of player stat declaration"
        );

        // Optional: Check for a semicolon at the end of the stat declaration
        if let Some(type_decl) = type_decl {
            match type_decl {
                TokenVariant::IntType => {
                    if let StatValue::Int(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(ParsingError::SyntaxError {
                            message: "Type mismatch: expected int literal for stat declared as int"
                                .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::FloatType => {
                    if let StatValue::Float(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(ParsingError::SyntaxError {
                            message:
                                "Type mismatch: expected float literal for stat declared as float"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::StringType => {
                    if let StatValue::String(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(ParsingError::SyntaxError {
                            message:
                                "Type mismatch: expected string literal for stat declared as string"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::BoolType => {
                    if let StatValue::Bool(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(ParsingError::SyntaxError {
                            message:
                                "Type mismatch: expected bool literal for stat declared as bool"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                _ => {
                    return Err(ParsingError::SyntaxError {
                        message: "Invalid type declaration in stat declaration".to_string(),
                        pos,
                    });
                }
            }
        }

        Ok(GameStat::new(name, value, visibility, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

    use super::*;

    #[test]
    fn test_parse_stat_no_type_decl() {
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Stat,
            TokenVariant::Identifier("health".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(100),
            TokenVariant::Semicolon
        );

        let stat = GameStat::parse(&mut ts).unwrap();
        assert_eq!(stat.name(), "health");
        assert_eq!(stat.default(), &StatValue::Int(100));
        assert_eq!(stat.visibility(), &GameStatVisibility::Public);
    }

    #[test]
    fn test_parse_stat_with_type_decl_int() {
        let mut ts = test_token_stream!(
            TokenVariant::Private,
            TokenVariant::Stat,
            TokenVariant::Identifier("mana".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::Equal,
            TokenVariant::IntLiteral(50),
            TokenVariant::Semicolon
        );

        let stat = GameStat::parse(&mut ts).unwrap();
        assert_eq!(stat.name(), "mana");
        assert_eq!(stat.default(), &StatValue::Int(50));
        assert_eq!(stat.visibility(), &GameStatVisibility::Private);
    }

    #[test]
    fn test_parse_stat_with_type_decl_float() {
        let mut ts = test_token_stream!(
            TokenVariant::Hidden,
            TokenVariant::Stat,
            TokenVariant::Identifier("stamina".to_string()),
            TokenVariant::Colon,
            TokenVariant::FloatType,
            TokenVariant::Equal,
            TokenVariant::FloatLiteral(75.5),
            TokenVariant::Semicolon
        );

        let stat = GameStat::parse(&mut ts).unwrap();
        assert_eq!(stat.name(), "stamina");
        assert_eq!(stat.default(), &StatValue::Float(75.5));
        assert_eq!(stat.visibility(), &GameStatVisibility::Hidden);
    }

    #[test]
    fn test_parse_stat_with_type_decl_string() {
        let mut ts = test_token_stream!(
            TokenVariant::Hidden,
            TokenVariant::Stat,
            TokenVariant::Identifier("status".to_string()),
            TokenVariant::Colon,
            TokenVariant::StringType,
            TokenVariant::Equal,
            TokenVariant::StringLiteral("healthy".to_string()),
            TokenVariant::Semicolon
        );

        let stat = GameStat::parse(&mut ts).unwrap();
        assert_eq!(stat.name(), "status");
        assert_eq!(stat.default(), &StatValue::String("healthy".to_string()));
        assert_eq!(stat.visibility(), &GameStatVisibility::Hidden);
    }

    #[test]
    fn test_parse_stat_with_type_decl_bool() {
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Stat,
            TokenVariant::Identifier("isAlive".to_string()),
            TokenVariant::Colon,
            TokenVariant::BoolType,
            TokenVariant::Equal,
            TokenVariant::BoolLiteral(true),
            TokenVariant::Semicolon
        );

        let stat = GameStat::parse(&mut ts).unwrap();
        assert_eq!(stat.name(), "isAlive");
        assert_eq!(stat.default(), &StatValue::Bool(true));
        assert_eq!(stat.visibility(), &GameStatVisibility::Public);
    }

    #[test]
    fn test_parse_stat_type_mismatch() {
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Stat,
            TokenVariant::Identifier("health".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::Equal,
            TokenVariant::StringLiteral("not a number".to_string()),
            TokenVariant::Semicolon
        );

        let err = GameStat::parse(&mut ts).unwrap_err();
        match err {
            ParsingError::SyntaxError { message, .. } => {
                assert_eq!(
                    message,
                    "Type mismatch: expected int literal for stat declared as int"
                );
            }
            _ => panic!("Expected syntax error due to type mismatch"),
        }
    }
}
