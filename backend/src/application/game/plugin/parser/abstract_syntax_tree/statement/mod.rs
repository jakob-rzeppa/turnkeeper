use crate::application::game::plugin::parser::abstract_syntax_tree::statement::variable_declaration::{VariableDeclarationStatement};

mod variable_declaration;

pub enum Statement {
    VariableDeclaration(VariableDeclarationStatement),
}