use crate::application::game::plugin::{parser::abstract_syntax_tree::{common::Block, statement::Statement}, runtime::RuntimeEnvironment};



impl RuntimeEnvironment {
    pub fn execute_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VariableDeclaration(var_decl) => self.execute_variable_declaration(var_decl)?,
            Statement::Assignment(assign) => self.execute_assignment(assign)?,
            Statement::Expression(expr) => self.execute_expression_stmt(expr)?,
            Statement::If(if_stmt) => self.execute_if_stmt(if_stmt)?,
            Statement::While(while_stmt) => self.execute_while_stmt(while_stmt)?,
             _ => return Err(format!("Unsupported statement type: {:?}", stmt)),
        }
        Ok(())
    }
}