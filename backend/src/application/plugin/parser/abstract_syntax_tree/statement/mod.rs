#[cfg(test)]
use crate::application::plugin::parser::abstract_syntax_tree::{
    atom::datatype::Datatype,
    expression::Expression,
    statement::if_statement::{ElseBranch, ElseIfBranch},
};
use crate::application::plugin::parser::{
    abstract_syntax_tree::{
        Parsable, TokenStream,
        statement::{
            assignment::AssignmentStatement, expression::ExpressionStatement,
            if_statement::IfStatement, variable_declaration::VariableDeclarationStatement,
            while_loop::WhileLoopStatement,
        },
    },
    error::ParsingError,
};

pub mod assignment;
pub mod expression;
pub mod if_statement;
pub mod variable_declaration;
pub mod while_loop;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    If(IfStatement),
    WhileLoop(WhileLoopStatement),
    Expression(ExpressionStatement),
}

impl Parsable for Statement {
    fn is_next(ts: &TokenStream) -> bool {
        VariableDeclarationStatement::is_next(ts)
            || AssignmentStatement::is_next(ts)
            || IfStatement::is_next(ts)
            || WhileLoopStatement::is_next(ts)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        if IfStatement::is_next(ts) {
            Ok(Statement::If(IfStatement::parse(ts)?))
        } else if WhileLoopStatement::is_next(ts) {
            Ok(Statement::WhileLoop(WhileLoopStatement::parse(ts)?))
        } else if VariableDeclarationStatement::is_next(ts) {
            Ok(Statement::VariableDeclaration(
                VariableDeclarationStatement::parse(ts)?,
            ))
        } else if AssignmentStatement::is_next(ts) {
            Ok(Statement::Assignment(AssignmentStatement::parse(ts)?))
        } else if ExpressionStatement::is_next(ts) {
            Ok(Statement::Expression(ExpressionStatement::parse(ts)?))
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

    pub fn new_while_loop(
        condition: Expression,
        body: Vec<Statement>,
        line: usize,
        column: usize,
    ) -> Self {
        Statement::WhileLoop(WhileLoopStatement::new_while_loop(
            condition, body, line, column,
        ))
    }

    pub fn new_expression_statement(expression: Expression, line: usize, column: usize) -> Self {
        Statement::Expression(ExpressionStatement::new(expression, line, column))
    }
}
