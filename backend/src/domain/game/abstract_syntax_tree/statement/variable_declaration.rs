use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::expression::Expression,
    game::value_objects::data::Datatype,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VariableDeclarationStatement {
    name: String,
    var_type: Datatype,
    value: Expression,
    pos: Position,
}

impl VariableDeclarationStatement {
    pub fn new(
        name: String,
        var_type: Datatype,
        value: Expression,
        pos: Position,
    ) -> Self {
        VariableDeclarationStatement {
            name: name.to_string(),
            var_type,
            value,
            pos,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn datatype(&self) -> &Datatype {
        &self.var_type
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Positioned for VariableDeclarationStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
