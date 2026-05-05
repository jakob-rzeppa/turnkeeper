use crate::domain::common::position::{Position, Positioned};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ElseBranch {
    then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
    pos: Position,
}

impl ElseBranch {
    pub fn new(
        then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
        pos: Position,
    ) -> Self {
        ElseBranch {
            then_branch,
            pos,
        }
    }

    pub fn then_statements(&self) -> &[crate::domain::game::abstract_syntax_tree::statement::Statement] {
        &self.then_branch
    }
}

impl Positioned for ElseBranch {
    fn position(&self) -> Position {
        self.pos
    }
}
