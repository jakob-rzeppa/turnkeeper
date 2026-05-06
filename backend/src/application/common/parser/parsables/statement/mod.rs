use crate::{application::common::parser::{error::ParsingError, lexer::token_stream::TokenStream, macros::get_pos, parsable::Parsable}, domain::{game::abstract_syntax_tree::{statement::{Statement, AssignmentStatement, IfStatement, VariableDeclarationStatement, WhileLoopStatement, ExpressionStatement}}}};

pub mod assignment;
pub mod expression;
pub mod if_statement;
pub mod variable_declaration;
pub mod while_loop;

impl Parsable for Statement {
    fn is_next(ts: &TokenStream) -> bool {
        VariableDeclarationStatement::is_next(ts)
            || AssignmentStatement::is_next(ts)
            || IfStatement::is_next(ts)
            || WhileLoopStatement::is_next(ts)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        if IfStatement::is_next(ts) {
            Ok(Statement::If(IfStatement::parse(ts, source_code)?))
        } else if WhileLoopStatement::is_next(ts) {
            Ok(Statement::WhileLoop(WhileLoopStatement::parse(ts, source_code)?))
        } else if VariableDeclarationStatement::is_next(ts) {
            Ok(Statement::VariableDeclaration(
                VariableDeclarationStatement::parse(ts, source_code)?,
            ))
        } else if AssignmentStatement::is_next(ts) {
            Ok(Statement::Assignment(AssignmentStatement::parse(ts, source_code)?))
        } else if ExpressionStatement::is_next(ts) {
            Ok(Statement::Expression(ExpressionStatement::parse(ts, source_code)?))
        } else {
            Err(ParsingError::SyntaxError {
                message: "Invalid statement".to_string(),
                pos: get_pos!(ts),
            })
        }
    }
}
