use crate::application::game::plugin::{parser::abstract_syntax_tree::{statement::Statement}, runtime::RuntimeEnvironment};

mod expression_statement;
mod if_statement;
mod while_statement;
mod variable_declaration_statement;
mod assignment_statement;

impl RuntimeEnvironment {
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.execute_variable_declaration_statement(var_decl)?,
            Statement::Assignment(assign) => self.execute_assignment_statement(assign)?,
            Statement::Expression(expr) => self.execute_expression_statement(expr)?,
            Statement::If(if_stmt) => self.execute_if_statement(if_stmt)?,
            Statement::While(while_stmt) => self.execute_while_statement(while_stmt)?,
             _ => return Err(format!("Unsupported statement type: {:?}", stmt)),
        }
        Ok(())
    }
}