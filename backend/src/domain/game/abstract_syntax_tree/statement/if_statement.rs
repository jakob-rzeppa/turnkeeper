use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::statement::{else_branch::ElseBranch, else_if_branch::ElseIfBranch},
    game::abstract_syntax_tree::expression::Expression,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IfStatement {
    condition: Expression,
    then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
    else_if_branches: Vec<ElseIfBranch>,
    else_branch: Option<ElseBranch>,
    pos: Position,
}

impl IfStatement {
    pub fn new(
        condition: Expression,
        then_branch: Vec<crate::domain::game::abstract_syntax_tree::statement::Statement>,
        else_if_branches: Vec<ElseIfBranch>,
        else_branch: Option<ElseBranch>,
        pos: Position,
    ) -> Self {
        IfStatement {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            pos,
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then_statements(&self) -> &[crate::domain::game::abstract_syntax_tree::statement::Statement] {
        &self.then_branch
    }

    pub fn else_if_branches(&self) -> &[ElseIfBranch] {
        &self.else_if_branches
    }

    pub fn else_branch(&self) -> Option<&ElseBranch> {
        self.else_branch.as_ref()
    }
}

impl Positioned for IfStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
