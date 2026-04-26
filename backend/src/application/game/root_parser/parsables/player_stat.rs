use crate::{
    application::{
        common::parser::{
            macros::{expect_token, get_pos, is_token, nth_is_token},
            parsable::Parsable,
            token_stream::TokenStream,
        },
        game::root_parser::{
            error::GameParsingError,
            token::{Token, TokenVariant},
        },
    },
    domain::game::{
        entities::stat::PlayerStat,
        value_objects::{stat_value::StatValue, stat_visibility::PlayerStatVisibility},
    },
};

impl Parsable<Token> for PlayerStat {
    type Error = GameParsingError;

    fn is_next(ts: &TokenStream<Token>) -> bool {
        is_token!(
            ts,
            TokenVariant::Hidden
                | TokenVariant::Private
                | TokenVariant::Public
                | TokenVariant::Protected
        ) && nth_is_token!(ts, 1, TokenVariant::Pstat)
    }

    fn parse(ts: &mut TokenStream<Token>) -> Result<Self, Self::Error> {
        let pos = get_pos!(ts);

        let visibility = match ts.next() {
            Some(Token {
                variant: TokenVariant::Hidden,
                ..
            }) => PlayerStatVisibility::Hidden,
            Some(Token {
                variant: TokenVariant::Private,
                ..
            }) => PlayerStatVisibility::Private,
            Some(Token {
                variant: TokenVariant::Public,
                ..
            }) => PlayerStatVisibility::Public,
            Some(Token {
                variant: TokenVariant::Protected,
                ..
            }) => PlayerStatVisibility::Protected,
            Some(token) => {
                return Err(Self::Error::UnexpectedToken {
                    expected: "Expected visibility modifier (hidden, private, public, protected)"
                        .to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(Self::Error::UnexpectedEOF {
                    expected: "Expected visibility modifier (hidden, private, public, protected)"
                        .to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Pstat,
            "Expected 'pstat' keyword after visibility modifier in player stat declaration"
        );

        let name = match ts.next() {
            Some(Token {
                variant: TokenVariant::Identifier(name),
                ..
            }) => name.clone(),
            Some(token) => {
                return Err(Self::Error::UnexpectedToken {
                    expected: "Expected identifier for player stat name".to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(Self::Error::UnexpectedEOF {
                    expected: "Expected identifier for player stat name".to_string(),
                });
            }
        };

        // Optional: Check for a colon after the pstat name, which could indicate a type declaration.
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
                        return Err(Self::Error::UnexpectedToken {
                            expected: "Expected type declaration (int, float, string, bool) after ':' in player stat declaration".to_string(),
                            found: token.variant.clone(),
                            pos: token.pos,
                        });
                    }
                }
                None => {
                    return Err(Self::Error::UnexpectedEOF {
                        expected: "Expected type declaration (int, float, string, bool) after ':' in player stat declaration".to_string(),
                    });
                }
            }
        } else {
            None
        };

        expect_token!(
            ts,
            TokenVariant::Equal,
            "Expected '=' after player stat name (and optional type declaration) in player stat declaration"
        );

        let value: StatValue = match ts.next() {
            Some(token) => match token.variant.clone() {
                TokenVariant::IntLiteral(num) => StatValue::Int(num),
                TokenVariant::FloatLiteral(num) => StatValue::Float(num),
                TokenVariant::StringLiteral(s) => StatValue::String(s),
                TokenVariant::BoolLiteral(b) => StatValue::Bool(b),
                _ => {
                    return Err(Self::Error::UnexpectedToken {
                        expected: "Expected literal value (int, float, string, bool) after '=' in player stat declaration".to_string(),
                        found: token.variant.clone(),
                        pos: token.pos,
                    });
                }
            },
            None => {
                return Err(Self::Error::UnexpectedEOF {
                    expected: "Expected literal value (int, float, string, bool) after '=' in player stat declaration".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "Expected ';' at the end of player stat declaration"
        );

        // Optional: Check for type match between declared type and provided value
        if let Some(type_decl) = type_decl {
            match type_decl {
                TokenVariant::IntType => {
                    if let StatValue::Int(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(Self::Error::SyntaxError {
                            message: "Type mismatch: expected int literal for player stat declared as int"
                                .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::FloatType => {
                    if let StatValue::Float(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(Self::Error::SyntaxError {
                            message:
                                "Type mismatch: expected float literal for player stat declared as float"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::StringType => {
                    if let StatValue::String(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(Self::Error::SyntaxError {
                            message:
                                "Type mismatch: expected string literal for player stat declared as string"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                TokenVariant::BoolType => {
                    if let StatValue::Bool(_) = value {
                        // Type matches, continue
                    } else {
                        return Err(Self::Error::SyntaxError {
                            message:
                                "Type mismatch: expected bool literal for player stat declared as bool"
                                    .to_string(),
                            pos,
                        });
                    }
                }
                _ => {
                    return Err(Self::Error::SyntaxError {
                        message: "Invalid type declaration in player stat declaration".to_string(),
                        pos,
                    });
                }
            }
        }

        Ok(PlayerStat::new(name, value, visibility, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

    use super::*;

    #[test]
    fn test_parse_pstat_no_type_decl() {
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Pstat,
            TokenVariant::Identifier("gold".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(100),
            TokenVariant::Semicolon
        );

        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.name(), "gold");
        assert_eq!(pstat.default(), &StatValue::Int(100));
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Public);
    }

    #[test]
    fn test_parse_pstat_with_type_decl_int() {
        let mut ts = test_token_stream!(
            TokenVariant::Protected,
            TokenVariant::Pstat,
            TokenVariant::Identifier("experience".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::Equal,
            TokenVariant::IntLiteral(0),
            TokenVariant::Semicolon
        );

        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.name(), "experience");
        assert_eq!(pstat.default(), &StatValue::Int(0));
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Protected);
    }

    #[test]
    fn test_parse_pstat_with_type_decl_float() {
        let mut ts = test_token_stream!(
            TokenVariant::Private,
            TokenVariant::Pstat,
            TokenVariant::Identifier("stamina".to_string()),
            TokenVariant::Colon,
            TokenVariant::FloatType,
            TokenVariant::Equal,
            TokenVariant::FloatLiteral(50.0),
            TokenVariant::Semicolon
        );

        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.name(), "stamina");
        assert_eq!(pstat.default(), &StatValue::Float(50.0));
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Private);
    }

    #[test]
    fn test_parse_pstat_with_type_decl_string() {
        let mut ts = test_token_stream!(
            TokenVariant::Hidden,
            TokenVariant::Pstat,
            TokenVariant::Identifier("secret".to_string()),
            TokenVariant::Colon,
            TokenVariant::StringType,
            TokenVariant::Equal,
            TokenVariant::StringLiteral("hidden_value".to_string()),
            TokenVariant::Semicolon
        );

        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.name(), "secret");
        assert_eq!(
            pstat.default(),
            &StatValue::String("hidden_value".to_string())
        );
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Hidden);
    }

    #[test]
    fn test_parse_pstat_with_type_decl_bool() {
        let mut ts = test_token_stream!(
            TokenVariant::Protected,
            TokenVariant::Pstat,
            TokenVariant::Identifier("hasSpecialAbility".to_string()),
            TokenVariant::Colon,
            TokenVariant::BoolType,
            TokenVariant::Equal,
            TokenVariant::BoolLiteral(false),
            TokenVariant::Semicolon
        );

        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.name(), "hasSpecialAbility");
        assert_eq!(pstat.default(), &StatValue::Bool(false));
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Protected);
    }

    #[test]
    fn test_parse_pstat_type_mismatch() {
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Pstat,
            TokenVariant::Identifier("level".to_string()),
            TokenVariant::Colon,
            TokenVariant::IntType,
            TokenVariant::Equal,
            TokenVariant::StringLiteral("not a number".to_string()),
            TokenVariant::Semicolon
        );

        let err = PlayerStat::parse(&mut ts).unwrap_err();
        match err {
            GameParsingError::SyntaxError { message, .. } => {
                assert_eq!(
                    message,
                    "Type mismatch: expected int literal for player stat declared as int"
                );
            }
            _ => panic!("Expected syntax error due to type mismatch"),
        }
    }

    #[test]
    fn test_parse_pstat_all_visibility_levels() {
        // Test Protected
        let mut ts = test_token_stream!(
            TokenVariant::Protected,
            TokenVariant::Pstat,
            TokenVariant::Identifier("test".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(1),
            TokenVariant::Semicolon
        );
        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Protected);

        // Test Public
        let mut ts = test_token_stream!(
            TokenVariant::Public,
            TokenVariant::Pstat,
            TokenVariant::Identifier("test".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(1),
            TokenVariant::Semicolon
        );
        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Public);

        // Test Private
        let mut ts = test_token_stream!(
            TokenVariant::Private,
            TokenVariant::Pstat,
            TokenVariant::Identifier("test".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(1),
            TokenVariant::Semicolon
        );
        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Private);

        // Test Hidden
        let mut ts = test_token_stream!(
            TokenVariant::Hidden,
            TokenVariant::Pstat,
            TokenVariant::Identifier("test".to_string()),
            TokenVariant::Equal,
            TokenVariant::IntLiteral(1),
            TokenVariant::Semicolon
        );
        let pstat = PlayerStat::parse(&mut ts).unwrap();
        assert_eq!(pstat.visibility(), &PlayerStatVisibility::Hidden);
    }
}
