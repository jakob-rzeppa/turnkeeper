use crate::application::game::plugin::parser::abstract_syntax_tree::{
    Parsable, ParsingError, TokenStream,
    statement::{
        assignment::AssignmentStatement, variable_declaration::VariableDeclarationStatement,
    },
};

mod assignment;
mod variable_declaration;

pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
}

impl Parsable for Statement {
    fn is_next(ts: &TokenStream) -> bool {
        VariableDeclarationStatement::is_next(ts) || AssignmentStatement::is_next(ts)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        if VariableDeclarationStatement::is_next(ts) {
            Ok(Statement::VariableDeclaration(
                VariableDeclarationStatement::parse(ts)?,
            ))
        } else if AssignmentStatement::is_next(ts) {
            Ok(Statement::Assignment(AssignmentStatement::parse(ts)?))
        } else {
            Err(ParsingError::SyntaxError {
                message: "Invalid statement".to_string(),
                pos: get_pos!(ts),
            })
        }
    }
}
