use crate::application::plugin::{
    parser::abstract_syntax_tree::statement::Statement,
    runtime::{RuntimeEnvironment, error::RuntimeError, execute::Executable},
};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod variable_declaration_statement;
mod while_statement;

impl Executable<()> for Statement {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        match self {
            Statement::VariableDeclaration(var_decl) => var_decl.execute(env),
            Statement::Assignment(assign) => assign.execute(env),
            Statement::If(if_stmt) => if_stmt.execute(env),
            Statement::WhileLoop(while_loop) => while_loop.execute(env),
            Statement::Expression(expr_stmt) => expr_stmt.execute(env),
        }
    }
}
