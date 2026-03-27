use crate::application::game::plugin::parser::abstract_syntax_tree::{
    Parsable, ParsingError, TokenStream,
    expression::atom::{
        function_call::FunctionCallExpressionAtom, literal::LiteralExpressionAtom,
        variable::VariableExpressionAtom,
    },
};
#[cfg(test)]
use crate::application::game::plugin::parser::abstract_syntax_tree::{
    atom::identifier::Identifier, expression::Expression,
};

pub mod function_call;
pub mod literal;
pub mod variable;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionAtom {
    Literal(LiteralExpressionAtom),
    Variable(VariableExpressionAtom),
    FunctionCall(FunctionCallExpressionAtom),
}

impl Parsable for ExpressionAtom {
    fn is_next(ts: &TokenStream) -> bool {
        LiteralExpressionAtom::is_next(ts)
            || VariableExpressionAtom::is_next(ts)
            || FunctionCallExpressionAtom::is_next(ts)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        if LiteralExpressionAtom::is_next(ts) {
            Ok(ExpressionAtom::Literal(LiteralExpressionAtom::parse(ts)?))
        } else if FunctionCallExpressionAtom::is_next(ts) {
            Ok(ExpressionAtom::FunctionCall(
                FunctionCallExpressionAtom::parse(ts)?,
            ))
        } else if VariableExpressionAtom::is_next(ts) {
            Ok(ExpressionAtom::Variable(VariableExpressionAtom::parse(ts)?))
        } else {
            Err(ParsingError::SyntaxError {
                message: "Expected literal, function call or variable in expression atom"
                    .to_string(),
                pos,
            })
        }
    }
}

#[cfg(test)]
impl ExpressionAtom {
    pub fn new_literal_int(value: i64, line: usize, first_char: usize) -> Self {
        ExpressionAtom::Literal(LiteralExpressionAtom::new_int(value, line, first_char))
    }

    pub fn new_literal_float(value: f64, line: usize, first_char: usize) -> Self {
        ExpressionAtom::Literal(LiteralExpressionAtom::new_float(value, line, first_char))
    }

    pub fn new_literal_string(value: String, line: usize, first_char: usize) -> Self {
        ExpressionAtom::Literal(LiteralExpressionAtom::new_string(value, line, first_char))
    }

    pub fn new_literal_bool(value: bool, line: usize, first_char: usize) -> Self {
        ExpressionAtom::Literal(LiteralExpressionAtom::new_bool(value, line, first_char))
    }

    pub fn new_variable(name: &str, line: usize, first_char: usize) -> Self {
        ExpressionAtom::Variable(VariableExpressionAtom::new(name, line, first_char))
    }

    pub fn new_function_call(
        identifier: Identifier,
        arguments: Vec<Expression>,
        line: usize,
        first_char: usize,
    ) -> Self {
        ExpressionAtom::FunctionCall(FunctionCallExpressionAtom::new(
            identifier, arguments, line, first_char,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::{
        lexer::token::TokenVariant,
        parser::abstract_syntax_tree::{atom::identifier::Identifier, expression::Expression},
    };

    use super::*;

    #[test]
    fn test_expression_atom_literal_parsing() {
        let mut ts = test_token_stream![TokenVariant::IntLiteral(42)];

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(atom, ExpressionAtom::new_literal_int(42, 0, 0));
    }

    #[test]
    fn test_expression_atom_variable_parsing() {
        let mut ts = test_token_stream![TokenVariant::Identifier("myVariable".to_string())];

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(atom, ExpressionAtom::new_variable("myVariable", 0, 0));
    }

    #[test]
    fn test_expression_atom_function_call_no_arguments_parsing() {
        let mut ts = test_token_stream![
            TokenVariant::Identifier("myFunction".to_string()),
            TokenVariant::LeftParen,
            TokenVariant::RightParen
        ];

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            atom,
            ExpressionAtom::new_function_call(Identifier::new("myFunction"), vec![], 0, 0)
        );
    }

    #[test]
    fn test_expression_atom_function_call_parsing() {
        let mut ts = test_token_stream![
            TokenVariant::Identifier("myFunction".to_string()),
            TokenVariant::LeftParen,
            TokenVariant::IntLiteral(42),
            TokenVariant::Comma,
            TokenVariant::StringLiteral("Hello".to_string()),
            TokenVariant::RightParen
        ];

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            atom,
            ExpressionAtom::new_function_call(
                Identifier::new("myFunction"),
                vec![
                    Expression::new_atom_literal_int(42, 2, 0),
                    Expression::new_atom_literal_string("Hello".to_string(), 4, 0)
                ],
                0,
                0
            )
        );
    }
}
