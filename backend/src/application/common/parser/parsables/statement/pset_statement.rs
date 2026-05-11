use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{ token::{ Token, TokenVariant }, token_stream::TokenStream },
        macros::{ change_err_msg, expect_token, get_pos, is_token },
        parsable::Parsable,
    },
    domain::game::abstract_syntax_tree::{
        expression::Expression,
        statement::pset_statement::PSetStatement,
    },
};

impl Parsable for PSetStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Pset)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(ts, TokenVariant::Pset, "Expected 'pset' keyword");

        let stat = if let Some(Token { variant: TokenVariant::Identifier(stat), .. }) = ts.next() {
            stat.to_string()
        } else {
            return Err(ParsingError::SyntaxError {
                message: "Expected identifier after 'pset'".to_string(),
                pos,
            });
        };

        expect_token!(ts, TokenVariant::For, "Expected 'for' after identifier in pset statement");

        let player = Expression::parse(ts, source_code).map_err(|err| {
            change_err_msg!(
                err,
                "Expected expression for the player name after for keyword in pset statement."
            )
        })?;

        expect_token!(ts, TokenVariant::Equal, "Expected '=' after player in pset statement");

        let value = Expression::parse(ts, source_code)?;

        expect_token!(ts, TokenVariant::Semicolon, "Expected ';' after set statement");

        Ok(PSetStatement::new(stat, player, value, pos))
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
    fn test_pset_statement_parsing() {
        let (mut ts, source_code) = test_token_stream!("pset health for player = 100;");

        let pset_stmt = PSetStatement::parse(&mut ts, &source_code).expect(
            "Failed to parse pset statement"
        );

        assert_eq!(pset_stmt.stat(), "health");
        assert_eq!(
            pset_stmt.value(),
            &Expression::Atom(ExpressionAtom::Literal(Value::Int(100), Position::new(0, 13)))
        );
    }
}
