use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::token_stream::TokenStream,
        macros::get_pos,
        parsable::Parsable,
    },
    domain::game::abstract_syntax_tree::statement::{
        Statement,
        assignment::AssignmentStatement,
        expression_statement::ExpressionStatement,
        if_statement::IfStatement,
        pset_statement::PSetStatement,
        set_statement::SetStatement,
        variable_declaration::VariableDeclarationStatement,
        while_loop::WhileLoopStatement,
    },
};

pub mod assignment;
pub mod expression;
pub mod if_statement;
pub mod variable_declaration;
pub mod while_loop;
pub mod set_statement;
pub mod pset_statement;

impl Parsable for Statement {
    fn is_next(ts: &TokenStream) -> bool {
        VariableDeclarationStatement::is_next(ts) ||
            AssignmentStatement::is_next(ts) ||
            IfStatement::is_next(ts) ||
            WhileLoopStatement::is_next(ts) ||
            ExpressionStatement::is_next(ts) ||
            SetStatement::is_next(ts) ||
            PSetStatement::is_next(ts)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        if IfStatement::is_next(ts) {
            Ok(Statement::If(IfStatement::parse(ts, source_code)?))
        } else if WhileLoopStatement::is_next(ts) {
            Ok(Statement::WhileLoop(WhileLoopStatement::parse(ts, source_code)?))
        } else if VariableDeclarationStatement::is_next(ts) {
            Ok(
                Statement::VariableDeclaration(
                    VariableDeclarationStatement::parse(ts, source_code)?
                )
            )
        } else if SetStatement::is_next(ts) {
            Ok(Statement::Set(SetStatement::parse(ts, source_code)?))
        } else if PSetStatement::is_next(ts) {
            Ok(Statement::PSet(PSetStatement::parse(ts, source_code)?))
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
