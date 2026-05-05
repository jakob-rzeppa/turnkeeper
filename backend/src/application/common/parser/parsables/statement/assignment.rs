use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token, nth_is_token}, parsable::Parsable, parsables::expression::Expression}, domain::common::position::{Position, Positioned}};



#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    name: String,
    value: Expression,
    pos: Position,
}

impl AssignmentStatement {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Parsable for AssignmentStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Identifier(_)) && nth_is_token!(ts, 1, TokenVariant::Equal)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let name = match ts.next() {
            Some(t) if matches!(t.variant, TokenVariant::Identifier(_)) => {
                if let TokenVariant::Identifier(name) = &t.variant {
                    name.clone()
                } else {
                    unreachable!("Token wasn't an Identifier after checking it was one.")
                }
            }
            Some(t) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected identifier at the beginning of assignment statement".to_string(),
                    found: t.variant.clone(),
                    pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected identifier at the beginning of assignment statement".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Equal,
            "'=' after identifier in assignment statement"
        );

        let value = Expression::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression after '=' in assignment statement")
        )?;

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "';' at the end of assignment statement"
        );

        Ok(AssignmentStatement {
            name,
            value,
            pos,
        })
    }
}

impl Positioned for AssignmentStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl AssignmentStatement {
    pub fn new(name: &str, value: Expression, line: usize, first_char: usize) -> Self {
        Self {
            name: name.to_string(),
            value,
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

use super::*;

    #[test]
    fn test_assignment_statement_parsing() {
        let (mut ts, source_code) = test_token_stream!("myVariable = 42;");

        assert!(AssignmentStatement::is_next(&ts));
        let assignment = AssignmentStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            assignment,
            AssignmentStatement::new(
                "myVariable",
                Expression::new_atom_literal_int(42, 0, 13),
                0,
                0
            )
        );
    }

    #[test]
    fn test_assignment_statement_no_semicolon() {
        let (mut ts, source_code) = test_token_stream!("myVariable = 42");

        let result = AssignmentStatement::parse(&mut ts, &source_code);
        assert!(result.is_err());
    }
}
