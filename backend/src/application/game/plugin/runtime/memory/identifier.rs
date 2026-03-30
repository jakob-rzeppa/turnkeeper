use std::fmt::Display;

use crate::application::game::plugin::parser::abstract_syntax_tree;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&abstract_syntax_tree::atom::identifier::Identifier> for Identifier {
    fn from(ast_id: &abstract_syntax_tree::atom::identifier::Identifier) -> Self {
        Identifier {
            name: ast_id.name().to_string(),
        }
    }
}
