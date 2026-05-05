use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::expression::Expression,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ElseIfBranch {
    condition: Expression,
    then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
    pos: Position,
}

impl ElseIfBranch {
    pub fn new(
        condition: Expression,
        then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
        pos: Position,
    ) -> Self {
        ElseIfBranch {
            condition,
            then_branch,
            pos,
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then_statements(&self) -> &[crate::domain::game::abstract_syntax_tree::statement::Statement] {
        &self.then_branch
    }
}

impl Positioned for ElseIfBranch {
    fn position(&self) -> Position {
        self.pos
    }
}
