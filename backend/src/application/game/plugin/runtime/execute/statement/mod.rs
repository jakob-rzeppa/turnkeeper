use crate::application::game::plugin::{
    parser::abstract_syntax_tree::statement::Statement,
    runtime::{RuntimeEnvironment, RuntimeError},
};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod variable_declaration_statement;
mod while_statement;

impl RuntimeEnvironment {
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                self.execute_variable_declaration_statement(var_decl)
            }
            Statement::Assignment(assign) => self.execute_assignment_statement(assign),
            Statement::If(if_stmt) => self.execute_if_statement(if_stmt),
            Statement::WhileLoop(while_loop) => self.execute_while_statement(while_loop),
            Statement::Expression(expr_stmt) => self.execute_expression_statement(expr_stmt),
        }
    }
}
