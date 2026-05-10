use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::abstract_syntax_tree::statement::Statement,
};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod variable_declaration_statement;
mod while_statement;

impl Executable<()> for Statement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        match self {
            Statement::VariableDeclaration(var_decl) => var_decl.execute(env).await,
            Statement::Assignment(assign) => assign.execute(env).await,
            Statement::If(if_stmt) => if_stmt.execute(env).await,
            Statement::WhileLoop(while_loop) => while_loop.execute(env).await,
            Statement::Expression(expr_stmt) => expr_stmt.execute(env).await,
        }
    }
}
