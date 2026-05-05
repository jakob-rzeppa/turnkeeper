use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{
            token::{Token, TokenVariant},
            token_stream::TokenStream,
        },
        macros::{change_err_msg, expect_token, get_pos, is_token, nth_is_token}, parsable::Parsable,
    },
    domain::game::{
        entities::weak::action::Action,
        value_objects::{
            data::Datatype, execution_trigger::ExecutionTrigger, parameter::Parameter,
            visibility::ActionVisibility,
        },
    },
};

impl Parsable for Action {
    fn is_next(ts: &TokenStream) -> bool {
        nth_is_token!(ts, 1, TokenVariant::Action)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let visibility = match ts.next() {
            Some(Token {
                variant: TokenVariant::Hidden,
                ..
            }) => ActionVisibility::Hidden,
            Some(Token {
                variant: TokenVariant::Private,
                ..
            }) => ActionVisibility::Private,
            Some(Token {
                variant: TokenVariant::Public,
                ..
            }) => ActionVisibility::Public,
            Some(Token {
                variant: TokenVariant::Protected,
                ..
            }) => {
                return Err(ParsingError::SyntaxError {
                    message: "Protected visibility is not supported for actions".to_string(),
                    pos,
                });
            }
            Some(token) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected visibility modifier (hidden, private, public, protected)"
                        .to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected visibility modifier (hidden, private, public, protected)"
                        .to_string(),
                });
            }
        };

        expect_token!(ts, TokenVariant::Action, "Expected 'action' keyword");

        let name = match ts.next() {
            Some(Token {
                variant: TokenVariant::Identifier(name),
                ..
            }) => name.clone(),
            Some(token) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected action name (identifier)".to_string(),
                    found: token.variant.clone(),
                    pos: token.pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected action name (identifier)".to_string(),
                });
            }
        };

        // Now there are three possible cases:
        // 1. Action with no parameters: `public action my_action { ... }`
        // 2. Action with parameters: `public action my_action(param1: int, param2: string) { ... }`
        // 3. Action with execution trigger: `public action my_action after other_action { ... }` (can be chained with `|`)
        let mut parameters = Vec::new();
        let mut execution_triggers = Vec::new();
        if let Some(Token {
            variant: TokenVariant::OpenParen,
            ..
        }) = ts.peek()
        {
            parameters = Self::parse_parameters(ts, source_code)?;
        } else if matches!(
            ts.peek(),
            Some(Token {
                variant: TokenVariant::After,
                ..
            }) | Some(Token {
                variant: TokenVariant::Before,
                ..
            })
        ) {
            execution_triggers = Self::parse_execution_triggers(ts, source_code)?;
        }

        // The body of the action is skipped over and the source_code for the action stored in the Action struct. This is because the body of the action will be parsed separately when the action is executed.

        expect_token!(
            ts,
            TokenVariant::OpenBrace,
            "Expected '{' to start action body"
        );

        let mut brace_count = 1;
        loop {
            // Check the next token
            match ts.peek() {
                Some(Token {
                    variant: TokenVariant::OpenBrace,
                    ..
                }) => brace_count += 1,
                Some(Token {
                    variant: TokenVariant::CloseBrace,
                    ..
                }) => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        break;
                    }
                }
                Some(_) => {}
                None => {
                    return Err(ParsingError::UnexpectedEOF {
                        expected: "Expected matching '}' to close action body".to_string(),
                    });
                }
            }

            ts.next(); // Consume the token and continue
        }

        let closing_brace = expect_token!(
            ts,
            TokenVariant::CloseBrace,
            "Expected '}' to close action body"
        );

        // Extract the source code for the action body from the original source code using the positions of the opening and closing braces
        let body_start_pos = pos;
        let body_end_pos = closing_brace.pos;
        let action_source_code =
            extract_source_code_range(source_code, body_start_pos, body_end_pos)?;

        Ok(Action::new(
            name,
            parameters,
            execution_triggers,
            visibility,
            action_source_code,
            pos,
        ))
    }
}

impl Action {
    /// Parse the parameter list of an action and return a vector of (param_name, param_type)
    fn parse_parameters(
        ts: &mut TokenStream,
        source_code: &str,
    ) -> Result<Vec<Parameter>, ParsingError> {
        expect_token!(
            ts,
            TokenVariant::OpenParen,
            "Expected '(' to start parameter list"
        );

        let mut parameters = Vec::new();

        loop {
            if ts.peek().is_none() {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected parameter name (identifier) or ')' to close parameter list"
                        .to_string(),
                });
            }

            if is_token!(ts, TokenVariant::CloseParen) {
                ts.next(); // Consume the closing parenthesis
                break;
            }

            let param_name = if let Token {
                variant: TokenVariant::Identifier(param_name),
                ..
            } = expect_token!(
                ts,
                TokenVariant::Identifier(_),
                "Expected parameter name (identifier)"
            ) {
                param_name.clone()
            } else {
                unreachable!(
                    "Expected parameter name (identifier) token after checking with is_token! macro"
                );
            };

            expect_token!(ts, TokenVariant::Colon, "Expected ':' after parameter name");

            let param_type = Datatype::parse(ts, source_code).map_err(|err| {
                change_err_msg!(err, "Expected parameter type (int, float, string, bool)")
            })?;

            parameters.push(Parameter::new(param_name, param_type));

            // If the next token is a comma, consume it and continue parsing the next parameter
            if is_token!(ts, TokenVariant::Comma) {
                ts.next(); // Consume the comma and continue
                continue;
            }

            // If the next token is a closing parenthesis, consume it and break the loop
            if is_token!(ts, TokenVariant::CloseParen) {
                ts.next(); // Consume the closing parenthesis
                break;
            }

            // If the next token is neither a comma nor a closing parenthesis, it's a syntax error
            match ts.peek() {
                Some(token) => {
                    return Err(ParsingError::UnexpectedToken {
                        expected: "Expected ',' between parameters or ')' to close parameter list"
                            .to_string(),
                        found: token.variant.clone(),
                        pos: token.pos,
                    });
                }
                None => {
                    return Err(ParsingError::UnexpectedEOF {
                        expected: "Expected ',' between parameters or ')' to close parameter list"
                            .to_string(),
                    });
                }
            }
        }

        Ok(parameters)
    }

    /// Parse the execution trigger list of an action and return a vector of action names that trigger this action
    fn parse_execution_triggers(
        ts: &mut TokenStream,
        _source_code: &str,
    ) -> Result<Vec<ExecutionTrigger>, ParsingError> {
        let mut triggers = Vec::new();

        loop {
            let trigger_type_token = expect_token!(
                ts,
                TokenVariant::After | TokenVariant::Before,
                "Expected 'after' or 'before' to start execution trigger"
            );

            let trigger = match expect_token!(
                ts,
                TokenVariant::Identifier(_)
                    | TokenVariant::TurnAdvance
                    | TokenVariant::RoundAdvance,
                "Expected action name (identifier) or turn/round advance after 'after' or 'before'"
            ) {
                Token {
                    variant: TokenVariant::Identifier(action_name),
                    ..
                } => match trigger_type_token.variant {
                    TokenVariant::After => ExecutionTrigger::AfterAction(action_name.clone()),
                    TokenVariant::Before => ExecutionTrigger::BeforeAction(action_name.clone()),
                    _ => unreachable!(
                        "Expected 'after' or 'before' token after checking with expect_token! macro"
                    ),
                },
                Token {
                    variant: TokenVariant::TurnAdvance,
                    ..
                } => match trigger_type_token.variant {
                    TokenVariant::After => ExecutionTrigger::AfterTurnAdvance,
                    TokenVariant::Before => ExecutionTrigger::BeforeTurnAdvance,
                    _ => unreachable!(
                        "Expected 'after' or 'before' token after checking with expect_token! macro"
                    ),
                },
                Token {
                    variant: TokenVariant::RoundAdvance,
                    ..
                } => match trigger_type_token.variant {
                    TokenVariant::After => ExecutionTrigger::AfterRoundAdvance,
                    TokenVariant::Before => ExecutionTrigger::BeforeRoundAdvance,
                    _ => unreachable!(
                        "Expected 'after' or 'before' token after checking with expect_token! macro"
                    ),
                },
                Token { variant, pos } => {
                    return Err(ParsingError::UnexpectedToken {
                        expected: "Expected action name (identifier) or turn/round advance after 'after' or 'before'"
                            .to_string(),
                        found: variant.clone(),
                        pos: pos.clone(),
                    });
                }
            };

            triggers.push(trigger);

            // If the next token is a |, consume it and continue parsing the next parameter
            if is_token!(ts, TokenVariant::Pipe) {
                ts.next(); // Consume the pipe and continue
                continue;
            } else {
                // If the next token isn't a | we are done parsing the execution triggers, so break the loop
                break;
            }
        }

        Ok(triggers)
    }
}

/// Extracts source code from a range defined by line/column positions
fn extract_source_code_range(
    source_code: &str,
    start_pos: crate::domain::common::position::Position,
    end_pos: crate::domain::common::position::Position,
) -> Result<String, ParsingError> {
    let lines: Vec<&str> = source_code.lines().collect();

    // Validate positions are within bounds
    if start_pos.line() >= lines.len() || end_pos.line() >= lines.len() {
        return Err(ParsingError::SyntaxError {
            message: "Position out of bounds".to_string(),
            pos: start_pos,
        });
    }

    let mut result = String::new();

    if start_pos.line() == end_pos.line() {
        // Single line extraction
        let line = lines[start_pos.line()];
        if start_pos.column() <= end_pos.column() + 1 && end_pos.column() + 1 <= line.len() {
            result.push_str(&line[start_pos.column()..end_pos.column() + 1]);
        }
    } else {
        // Multi-line extraction
        // Add the remainder of the start line
        let start_line = lines[start_pos.line()];
        if start_pos.column() <= start_line.len() {
            result.push_str(&start_line[start_pos.column()..]);
            result.push('\n');
        }

        // Add all lines between start and end
        for i in (start_pos.line() + 1)..end_pos.line() {
            result.push_str(lines[i]);
            result.push('\n');
        }

        // Add the beginning of the end line
        let end_line = lines[end_pos.line()];
        if end_pos.column() + 1 <= end_line.len() {
            result.push_str(&end_line[0..end_pos.column() + 1]);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

    use super::*;

    #[test]
    fn test_simple_action() {
        let (mut ts, source_code) =
            test_token_stream!("public action my_action { let x: int = 5; }");

        let action = Action::parse(&mut ts, &source_code).unwrap();

        assert_eq!(action.name(), "my_action");
        assert_eq!(action.visibility(), &ActionVisibility::Public);
        assert!(action.parameters().is_empty());
        assert!(action.execution_triggers().is_empty());
        assert_eq!(action.source_code(), source_code);
    }

    #[test]
    fn test_action_with_parameters() {
        let (mut ts, source_code) = test_token_stream!(
            "private action my_action(param1: int, param2: string) { let x: int = 5; }"
        );

        let action = Action::parse(&mut ts, &source_code).unwrap();

        assert_eq!(action.name(), "my_action");
        assert_eq!(action.visibility(), &ActionVisibility::Private);
        assert_eq!(action.parameters().len(), 2);
        assert_eq!(action.parameters()[0].name(), "param1");
        assert_eq!(action.parameters()[0].datatype(), &Datatype::Int);
        assert_eq!(action.parameters()[1].name(), "param2");
        assert_eq!(action.parameters()[1].datatype(), &Datatype::String);
        assert!(action.execution_triggers().is_empty());
        assert_eq!(action.source_code(), source_code);
    }

    #[test]
    fn test_action_with_execution_triggers() {
        let (mut ts, source_code) = test_token_stream!(
            "public action my_action after my_trigger | before my_other_trigger { let x: int = 5; }"
        );

        let action = Action::parse(&mut ts, &source_code).unwrap();

        assert_eq!(action.name(), "my_action");
        assert_eq!(action.visibility(), &ActionVisibility::Public);
        assert!(action.parameters().is_empty());
        assert_eq!(action.execution_triggers().len(), 2);
        assert_eq!(
            action.execution_triggers()[0],
            ExecutionTrigger::AfterAction("my_trigger".into())
        );
        assert_eq!(
            action.execution_triggers()[1],
            ExecutionTrigger::BeforeAction("my_other_trigger".into())
        );
        assert_eq!(action.source_code(), source_code);
    }

    #[test]
    fn test_action_with_turn_and_round_triggers() {
        let (mut ts, source_code) = test_token_stream!(
            r#"public action my_action 
                before TurnAdvance | 
                after TurnAdvance | 
                before RoundAdvance | 
                after RoundAdvance
            { 
                let x: int = 5; 
            }"#
        );

        let action = Action::parse(&mut ts, &source_code).unwrap();

        assert_eq!(action.name(), "my_action");
        assert_eq!(action.visibility(), &ActionVisibility::Public);
        assert!(action.parameters().is_empty());
        assert_eq!(action.execution_triggers().len(), 4);
        assert_eq!(
            action.execution_triggers()[0],
            ExecutionTrigger::BeforeTurnAdvance
        );
        assert_eq!(
            action.execution_triggers()[1],
            ExecutionTrigger::AfterTurnAdvance
        );
        assert_eq!(
            action.execution_triggers()[2],
            ExecutionTrigger::BeforeRoundAdvance
        );
        assert_eq!(
            action.execution_triggers()[3],
            ExecutionTrigger::AfterRoundAdvance
        );
        assert_eq!(action.source_code(), source_code);
    }

    #[test]
    fn test_action_with_no_body_fails() {
        let (mut ts, source_code) = test_token_stream!("public action my_action");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_unclosed_body_fails() {
        let (mut ts, source_code) =
            test_token_stream!("public action my_action { let x: int = 5; ");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_params_and_triggers_fails() {
        let (mut ts, source_code) = test_token_stream!(
            "public action my_action(param1: int) after my_trigger { let x: int = 5; }"
        );
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_invalid_trigger_fails() {
        let (mut ts, source_code) =
            test_token_stream!("public action my_action after 123 { let x: int = 5; }");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_invalid_param_fails() {
        let (mut ts, source_code) =
            test_token_stream!("public action my_action(param1: invalid_type) { let x: int = 5; }");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_protected_visibility_fails() {
        let (mut ts, source_code) =
            test_token_stream!("protected action my_action { let x: int = 5; }");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_missing_visibility_fails() {
        let (mut ts, source_code) = test_token_stream!("action my_action { let x: int = 5; }");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_missing_name_fails() {
        let (mut ts, source_code) = test_token_stream!("public action { let x: int = 5; }");
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_invalid_trigger_type_fails() {
        let (mut ts, source_code) = test_token_stream!(
            "public action my_action trigger invalid_trigger { let x: int = 5; }"
        );
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_missing_parameter_comma_fails() {
        let (mut ts, source_code) = test_token_stream!(
            "public action my_action(param1: int param2: string) { let x: int = 5; }"
        );
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }

    #[test]
    fn test_action_with_missing_parameter_colon_fails() {
        let (mut ts, source_code) = test_token_stream!(
            "public action my_action(param1 int, param2: string) { let x: int = 5; }"
        );
        assert!(Action::parse(&mut ts, &source_code).is_err());
    }
}
