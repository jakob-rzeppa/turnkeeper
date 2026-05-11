use crate::domain::{
    common::position::{ Position, Positioned },
    game::abstract_syntax_tree::expression::Expression,
};

/// Represents a statement that sets a game stat to a specific value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetStatement {
    stat: String,
    value: Expression,
    pos: Position,
}

impl SetStatement {
    pub fn new(stat: String, value: Expression, pos: Position) -> Self {
        SetStatement {
            stat,
            value,
            pos,
        }
    }

    pub fn stat(&self) -> &str {
        &self.stat
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl Positioned for SetStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
