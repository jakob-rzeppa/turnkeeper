use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos}, parsable::Parsable, parsables::expression::Expression}, domain::common::position::{Position, Positioned}};



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExpressionStatement {
    expression: Expression,
    pos: Position,
}

impl ExpressionStatement {
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

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

        Ok(ExpressionStatement { expression, pos })
    }
}

impl Positioned for ExpressionStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl ExpressionStatement {
    pub fn new(expression: Expression, line: usize, column: usize) -> Self {
        Self {
            expression,
            pos: Position::new(line, column),
        }
    }
}
