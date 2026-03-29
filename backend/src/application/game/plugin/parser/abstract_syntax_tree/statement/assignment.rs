use crate::application::game::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::abstract_syntax_tree::{
        Parsable, ParsingError, TokenStream, atom::identifier::Identifier, expression::Expression,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    identifier: Identifier,
    value: Expression,
    pos: Position,
}

impl Parsable for AssignmentStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Identifier(_)) && nth_is_token!(ts, 1, TokenVariant::Assign)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let identifier = expect_parse!(
            ts,
            Identifier,
            "Expected identifier at the beginning of assignment statement"
        );

        expect_token!(
            ts,
            TokenVariant::Assign,
            "Expected '=' after identifier in assignment statement"
        );

        let value = expect_parse!(
            ts,
            Expression,
            "Expected expression after '=' in assignment statement"
        );

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "Expected ';' at the end of assignment statement"
        );

        Ok(AssignmentStatement {
            identifier,
            value,
            pos,
        })
    }
}

#[cfg(test)]
impl AssignmentStatement {
    pub fn new(name: &str, value: Expression, line: usize, first_char: usize) -> Self {
        Self {
            identifier: Identifier::new(name),
            value,
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_statement_parsing() {
        let mut ts = test_token_stream!(
            TokenVariant::Identifier("myVariable".to_string()),
            TokenVariant::Assign,
            TokenVariant::IntLiteral(42),
            TokenVariant::Semicolon
        );

        assert!(AssignmentStatement::is_next(&ts));
        let assignment = AssignmentStatement::parse(&mut ts).unwrap();
        assert_eq!(
            assignment,
            AssignmentStatement::new(
                "myVariable",
                Expression::new_atom_literal_int(42, 2, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_assignment_statement_no_semicolon() {
        let mut ts = test_token_stream!(
            TokenVariant::Identifier("myVariable".to_string()),
            TokenVariant::Assign,
            TokenVariant::IntLiteral(42)
        );

        let result = AssignmentStatement::parse(&mut ts);
        assert!(result.is_err());
    }
}
