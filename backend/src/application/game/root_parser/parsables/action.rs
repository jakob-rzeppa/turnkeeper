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
        entities::weak::action::Action,
        value_objects::{
            data::VariableType, execution_trigger::ExecutionTrigger, visibility::ActionVisibility,
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
            parameters = Self::parse_parameters(ts)?;
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
            execution_triggers = Self::parse_execution_triggers(ts)?;
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
    fn parse_parameters(ts: &mut TokenStream) -> Result<Vec<(String, VariableType)>, ParsingError> {
        // This function should parse the parameter list of an action and return a vector of (param_name, param_type)
        // For example, for `my_action(param1: int, param2: string)`, it should return vec![("param1", "int"), ("param2", "string")]
        unimplemented!()
    }

    fn parse_execution_triggers(
        ts: &mut TokenStream,
    ) -> Result<Vec<ExecutionTrigger>, ParsingError> {
        // This function should parse the execution trigger list of an action and return a vector of action names that trigger this action
        // For example, for `my_action after other_action | another_action`, it should return vec!["other_action", "another_action"]
        unimplemented!()
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
        if start_pos.column() <= end_pos.column() && end_pos.column() <= line.len() {
            result.push_str(&line[start_pos.column()..end_pos.column()]);
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
        if end_pos.column() <= end_line.len() {
            result.push_str(&end_line[0..end_pos.column()]);
        }
    }

    Ok(result)
}
