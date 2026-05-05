use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::expression::Expression,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WhileLoopStatement {
    condition: Expression,
    body: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
    pos: Position,
}

impl WhileLoopStatement {
    pub fn new(
        condition: Expression,
        body: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
        pos: Position,
    ) -> Self {
        WhileLoopStatement {
            condition,
            body,
            pos,
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn body(&self) -> &[crate::domain::game::abstract_syntax_tree::statement::Statement] {
        &self.body
    }
}

impl Positioned for WhileLoopStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
