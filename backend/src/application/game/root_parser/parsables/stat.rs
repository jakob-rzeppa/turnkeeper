use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{
            token::{Token, TokenVariant},
            token_stream::TokenStream,
        },
        macros::{expect_token, get_pos, nth_is_token},
        parsable::Parsable,
    },
    domain::game::{
        entities::weak::stat::GameStat,
        value_objects::{
            data::{VariableType, VariableValue},
            visibility::GameStatVisibility,
        },
    },
};

impl Parsable for GameStat {
    fn is_next(ts: &TokenStream) -> bool {
        nth_is_token!(ts, 1, TokenVariant::Stat)
    }

    fn parse(ts: &mut TokenStream, _source_code: &str) -> Result<Self, ParsingError> {
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
        let datatype = if let Some(Token {
            variant: TokenVariant::Colon,
            ..
        }) = ts.peek()
        {
            ts.next(); // Consume the colon

            match ts.next() {
                Some(token) => match token.variant {
                    TokenVariant::IntType => Some(VariableType::Int),
                    TokenVariant::FloatType => Some(VariableType::Float),
                    TokenVariant::StringType => Some(VariableType::String),
                    TokenVariant::BoolType => Some(VariableType::Bool),
                    _ => {
                        return Err(ParsingError::UnexpectedToken {
                                expected: "Expected type declaration (int, float, string, bool) after ':' in player stat declaration".to_string(),
                                found: token.variant.clone(),
                                pos: token.pos,
                            });
                    }
                },
                None => {
                    return Err(ParsingError::UnexpectedEOF {
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
            "Expected '=' after stat name (and optional type declaration) in stat declaration"
        );

        let value: VariableValue = match ts.next() {
            Some(token) => match token.variant.clone() {
                TokenVariant::IntLiteral(num) => VariableValue::Int(num),
                TokenVariant::FloatLiteral(num) => VariableValue::Float(num),
                TokenVariant::StringLiteral(s) => VariableValue::String(s),
                TokenVariant::BoolLiteral(b) => VariableValue::Bool(b),
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

        // Check for type match between declared type and provided value and if no type declaration is provided, infer the type from the value.
        let datatype = if let Some(declared_type) = datatype {
            if !value.is_type(&declared_type) {
                return Err(ParsingError::SyntaxError {
                    message: format!(
                        "Type mismatch: expected {:?} literal for player stat declared as {:?}",
                        declared_type, declared_type
                    ),
                    pos,
                });
            }

            declared_type
        } else {
            value.datatype()
        };

        Ok(GameStat::new(name, datatype, value, visibility, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

    use super::*;

    #[test]
    fn test_parse_stat_no_type_decl() {
        let (mut ts, source_code) = test_token_stream!("public stat health = 100;");

        let stat = GameStat::parse(&mut ts, &source_code).unwrap();
        assert_eq!(stat.name(), "health");
        assert_eq!(stat.default(), &VariableValue::Int(100));
        assert_eq!(stat.visibility(), &GameStatVisibility::Public);
    }

    #[test]
    fn test_parse_stat_with_type_decl_int() {
        let (mut ts, source_code) = test_token_stream!("private stat mana: int = 50;");

        let stat = GameStat::parse(&mut ts, &source_code).unwrap();
        assert_eq!(stat.name(), "mana");
        assert_eq!(stat.default(), &VariableValue::Int(50));
        assert_eq!(stat.visibility(), &GameStatVisibility::Private);
    }

    #[test]
    fn test_parse_stat_with_type_decl_float() {
        let (mut ts, source_code) = test_token_stream!("hidden stat stamina: float = 75.5;");

        let stat = GameStat::parse(&mut ts, &source_code).unwrap();
        assert_eq!(stat.name(), "stamina");
        assert_eq!(stat.default(), &VariableValue::Float(75.5));
        assert_eq!(stat.visibility(), &GameStatVisibility::Hidden);
    }

    #[test]
    fn test_parse_stat_with_type_decl_string() {
        let (mut ts, source_code) = test_token_stream!("hidden stat status: string = \"healthy\";");

        let stat = GameStat::parse(&mut ts, &source_code).unwrap();
        assert_eq!(stat.name(), "status");
        assert_eq!(
            stat.default(),
            &VariableValue::String("healthy".to_string())
        );
        assert_eq!(stat.visibility(), &GameStatVisibility::Hidden);
    }

    #[test]
    fn test_parse_stat_with_type_decl_bool() {
        let (mut ts, source_code) = test_token_stream!("public stat isAlive: bool = true;");

        let stat = GameStat::parse(&mut ts, &source_code).unwrap();
        assert_eq!(stat.name(), "isAlive");
        assert_eq!(stat.default(), &VariableValue::Bool(true));
        assert_eq!(stat.visibility(), &GameStatVisibility::Public);
    }

    #[test]
    fn test_parse_stat_type_mismatch() {
        let (mut ts, source_code) =
            test_token_stream!("public stat health: int = \"not a number\";");

        let err = GameStat::parse(&mut ts, &source_code).unwrap_err();
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
