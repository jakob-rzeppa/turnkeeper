use crate::application::game::plugin::{
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
pub struct FunctionCallExpressionAtom {
    identifier: Identifier,
    arguments: Vec<Expression>,
    pos: Position,
}

impl Parsable for FunctionCallExpressionAtom {
    fn is_next(ts: &TokenStream) -> bool {
        Identifier::is_next(ts) && nth_is_token!(ts, 1, TokenVariant::LeftParen)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let identifier = expect_parse!(
            ts,
            Identifier,
            "function name (identifier) at the beginning of function call"
        );

        expect_token!(
            ts,
            TokenVariant::LeftParen,
            "'(' after function name in function call"
        );

        let mut arguments = Vec::new();
        if !is_token!(ts, TokenVariant::RightParen) {
            loop {
                let argument =
                    expect_parse!(ts, Expression, "expression as argument in function call");
                arguments.push(argument);

                if is_token!(ts, TokenVariant::RightParen) {
                    break;
                }

                expect_token!(
                    ts,
                    TokenVariant::Comma,
                    "',' between arguments in function call"
                );
            }
        }

        expect_token!(
            ts,
            TokenVariant::RightParen,
            "')' at the end of function call"
        );

        Ok(FunctionCallExpressionAtom {
            identifier,
            arguments,
            pos,
        })
    }
}

impl Positioned for FunctionCallExpressionAtom {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl FunctionCallExpressionAtom {
    pub fn new(
        identifier: Identifier,
        arguments: Vec<Expression>,
        line: usize,
        first_char: usize,
    ) -> Self {
        FunctionCallExpressionAtom {
            identifier,
            arguments,
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_call_parsing_no_arguments() {
        let mut ts = test_token_stream![
            TokenVariant::Identifier("myFunction".to_string()),
            TokenVariant::LeftParen,
            TokenVariant::RightParen
        ];

        assert!(FunctionCallExpressionAtom::is_next(&ts));
        let function_call = FunctionCallExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            function_call,
            FunctionCallExpressionAtom::new(Identifier::new("myFunction"), vec![], 0, 0)
        );
    }

    #[test]
    fn test_function_call_parsing_with_arguments() {
        let mut ts = test_token_stream![
            TokenVariant::Identifier("myFunction".to_string()),
            TokenVariant::LeftParen,
            TokenVariant::IntLiteral(42),
            TokenVariant::Comma,
            TokenVariant::StringLiteral("Hello".to_string()),
            TokenVariant::RightParen
        ];

        assert!(FunctionCallExpressionAtom::is_next(&ts));
        let function_call = FunctionCallExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            function_call,
            FunctionCallExpressionAtom::new(
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
