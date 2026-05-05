use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos}, parsable::Parsable}, domain::{common::position::{Position, Positioned}, game::abstract_syntax_tree::{statement::ExpressionStatement, expression::Expression}}};



impl Parsable for ExpressionStatement {
    fn is_next(ts: &TokenStream) -> bool {
        Expression::is_next(ts)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let expression = Expression::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression in expression statement")
        )?;

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "';' after expression statement"
        );

        Ok(ExpressionStatement::new(expression, pos))
    }
}