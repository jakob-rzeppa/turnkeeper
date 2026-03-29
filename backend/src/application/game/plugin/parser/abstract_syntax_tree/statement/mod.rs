use crate::application::game::plugin::parser::abstract_syntax_tree::{
    Parsable, ParsingError, TokenStream,
    statement::{
        assignment_statement::AssignmentStatement, if_statement::IfStatement,
        variable_declaration_statement::VariableDeclarationStatement,
    },
};
#[cfg(test)]
use crate::application::game::plugin::parser::abstract_syntax_tree::{
    atom::datatype::Datatype,
    expression::Expression,
    statement::if_statement::{ElseBranch, ElseIfBranch},
};

mod assignment_statement;
mod if_statement;
mod variable_declaration_statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    If(IfStatement),
}

impl Parsable for Statement {
    fn is_next(ts: &TokenStream) -> bool {
        VariableDeclarationStatement::is_next(ts)
            || AssignmentStatement::is_next(ts)
            || IfStatement::is_next(ts)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        if IfStatement::is_next(ts) {
            Ok(Statement::If(IfStatement::parse(ts)?))
        } else if VariableDeclarationStatement::is_next(ts) {
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

#[cfg(test)]
impl Statement {
    pub fn new_variable_declaration(
        name: &str,
        var_type: Datatype,
        value: Expression,
        line: usize,
        column: usize,
    ) -> Self {
        Statement::VariableDeclaration(VariableDeclarationStatement::new(
            name, var_type, value, line, column,
        ))
    }

    pub fn new_assignment(
        variable_name: &str,
        value: Expression,
        line: usize,
        column: usize,
    ) -> Self {
        Statement::Assignment(AssignmentStatement::new(variable_name, value, line, column))
    }

    pub fn new_if(
        condition: Expression,
        then_branch: Vec<Statement>,
        else_if_branches: Vec<ElseIfBranch>,
        else_branch: Option<ElseBranch>,
        line: usize,
        column: usize,
    ) -> Self {
        Statement::If(IfStatement::new(
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            line,
            column,
        ))
    }
}
