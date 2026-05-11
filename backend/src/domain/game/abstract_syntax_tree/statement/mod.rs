pub mod assignment;
pub mod else_branch;
pub mod else_if_branch;
pub mod expression_statement;
pub mod if_statement;
pub mod variable_declaration;
pub mod while_loop;
pub mod set_statement;
pub mod pset_statement;

use crate::domain::{
    common::position::{ Position, Positioned },
    game::abstract_syntax_tree::statement::{
        assignment::AssignmentStatement,
        expression_statement::ExpressionStatement,
        if_statement::IfStatement,
        pset_statement::PSetStatement,
        set_statement::SetStatement,
        variable_declaration::VariableDeclarationStatement,
        while_loop::WhileLoopStatement,
    },
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    If(IfStatement),
    WhileLoop(WhileLoopStatement),
    Expression(ExpressionStatement),
    Set(SetStatement),
    PSet(PSetStatement),
}

impl Positioned for Statement {
    fn position(&self) -> Position {
        match self {
            Statement::VariableDeclaration(var_decl) => var_decl.position(),
            Statement::Assignment(assign) => assign.position(),
            Statement::If(if_stmt) => if_stmt.position(),
            Statement::WhileLoop(while_loop) => while_loop.position(),
            Statement::Expression(expr_stmt) => expr_stmt.position(),
            Statement::Set(set_stmt) => set_stmt.position(),
            Statement::PSet(pset_stmt) => pset_stmt.position(),
        }
    }
}
