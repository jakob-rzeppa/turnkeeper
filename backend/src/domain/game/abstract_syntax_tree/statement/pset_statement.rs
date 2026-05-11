use crate::domain::{
    common::position::{ Position, Positioned },
    game::abstract_syntax_tree::expression::Expression,
};

/// Represents a statement that sets a game stat for a specific player to a specific value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PSetStatement {
    stat: String,
    player: Expression,
    value: Expression,
    pos: Position,
}

impl PSetStatement {
    pub fn new(stat: String, player: Expression, value: Expression, pos: Position) -> Self {
        PSetStatement {
            stat,
            player,
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

    pub fn player(&self) -> &Expression {
        &self.player
    }
}

impl Positioned for PSetStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
