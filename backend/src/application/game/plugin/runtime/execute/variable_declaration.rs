use crate::application::game::plugin::{parser::abstract_syntax_tree::{common::Type, expression::Expr, statement::VariableDeclaration}, runtime::{RuntimeEnvironment, memory::VariableValue}};



impl RuntimeEnvironment {
    pub fn execute_variable_declaration(&mut self, element: &VariableDeclaration) -> Result<(), String> {
        unimplemented!("Variable declaration execution not implemented yet");
    }
}