use crate::application::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{
            Parsable, Positioned, TokenStream, atom::identifier::Identifier, expression::Expression,
        },
        error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    identifier: Identifier,
    value: Expression,
    pos: Position,
}

impl AssignmentStatement {
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
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
            "identifier at the beginning of assignment statement"
        );

        expect_token!(
            ts,
            TokenVariant::Assign,
            "'=' after identifier in assignment statement"
        );

        let value = expect_parse!(
            ts,
            Expression,
            "expression after '=' in assignment statement"
        );

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "';' at the end of assignment statement"
        );

        Ok(AssignmentStatement {
            identifier,
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
