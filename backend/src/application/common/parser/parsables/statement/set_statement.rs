use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{ token::{ Token, TokenVariant }, token_stream::TokenStream },
        macros::{ expect_token, get_pos, is_token },
        parsable::Parsable,
    },
    domain::game::abstract_syntax_tree::{
        expression::Expression,
        statement::set_statement::SetStatement,
    },
};

impl Parsable for SetStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Set)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(ts, TokenVariant::Set, "Expected 'set' keyword");

        let stat = if let Some(Token { variant: TokenVariant::Identifier(stat), .. }) = ts.next() {
            stat.to_string()
        } else {
            return Err(ParsingError::SyntaxError {
                message: "Expected identifier after 'set'".to_string(),
                pos,
            });
        };

        expect_token!(ts, TokenVariant::Equal, "Expected '=' after identifier in set statement");

        let value = Expression::parse(ts, source_code)?;

        expect_token!(ts, TokenVariant::Semicolon, "Expected ';' after set statement");

        Ok(SetStatement::new(stat, value, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::common::parser::macros::test_token_stream,
        domain::{
            common::position::Position,
            game::{
                abstract_syntax_tree::expression::atom::ExpressionAtom,
                value_objects::data::Value,
            },
        },
    };

    use super::*;

    #[test]
    fn test_set_statement_parsing() {
        let (mut ts, source_code) = test_token_stream!("set health = 100;");

        let set_stmt = SetStatement::parse(&mut ts, &source_code).expect(
            "Failed to parse set statement"
        );

        assert_eq!(set_stmt.stat(), "health");
        assert_eq!(
            set_stmt.value(),
            &Expression::Atom(ExpressionAtom::Literal(Value::Int(100), Position::new(0, 13)))
        );
    }
}
