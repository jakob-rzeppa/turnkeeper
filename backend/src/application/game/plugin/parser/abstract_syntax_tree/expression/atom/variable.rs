use crate::application::game::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::abstract_syntax_tree::{
        Parsable, Positioned, TokenStream, atom::identifier::Identifier, error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpressionAtom {
    identifier: Identifier,
    pos: Position,
}

impl VariableExpressionAtom {
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Parsable for VariableExpressionAtom {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Identifier(_))
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let identifier = expect_parse!(ts, Identifier, "identifier");

        Ok(VariableExpressionAtom { identifier, pos })
    }
}

impl Positioned for VariableExpressionAtom {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl VariableExpressionAtom {
    pub fn new(name: &str, line: usize, first_char: usize) -> Self {
        Self {
            identifier: Identifier::new(name),
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_expression_atom_parsing() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("myVariable".to_string()));

        assert!(VariableExpressionAtom::is_next(&ts));
        let variable_atom = VariableExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            variable_atom,
            VariableExpressionAtom::new("myVariable", 0, 0)
        );
    }
}
