use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::expression::Expression,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AssignmentStatement {
    name: String,
    value: Expression,
    pos: Position,
}

impl AssignmentStatement {
    pub fn new(name: String, value: Expression, pos: Position) -> Self {
        AssignmentStatement {
            name: name,
            value,
            pos,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Positioned for AssignmentStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
