mod assignment;
mod else_branch;
mod else_if_branch;
mod expression_statement;
mod if_statement;
mod variable_declaration;
mod while_loop;

pub use assignment::AssignmentStatement;
pub use else_branch::ElseBranch;
pub use else_if_branch::ElseIfBranch;
pub use expression_statement::ExpressionStatement;
pub use if_statement::IfStatement;
pub use variable_declaration::VariableDeclarationStatement;
pub use while_loop::WhileLoopStatement;

use crate::domain::common::position::{Position, Positioned};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    If(IfStatement),
    WhileLoop(WhileLoopStatement),
    Expression(ExpressionStatement),
}

impl Positioned for Statement {
    fn position(&self) -> Position {
        match self {
            Statement::VariableDeclaration(var_decl) => var_decl.position(),
            Statement::Assignment(assign) => assign.position(),
            Statement::If(if_stmt) => if_stmt.position(),
            Statement::WhileLoop(while_loop) => while_loop.position(),
            Statement::Expression(expr_stmt) => expr_stmt.position(),
        }
    }
}
