use crate::application::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{
            Parsable, Positioned, TokenStream, expression::Expression, statement::Statement,
        },
        error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    condition: Expression,
    then_branch: Vec<Statement>,
    else_if_branches: Vec<ElseIfBranch>,
    else_branch: Option<ElseBranch>,
    pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch {
    then_branch: Vec<Statement>,
    pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfBranch {
    condition: Expression,
    then_branch: Vec<Statement>,
    pos: Position,
}

impl IfStatement {
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then_statements(&self) -> &[Statement] {
        &self.then_branch
    }

    pub fn else_if_branches(&self) -> &[ElseIfBranch] {
        &self.else_if_branches
    }

    pub fn else_branch(&self) -> Option<&ElseBranch> {
        self.else_branch.as_ref()
    }
}

impl ElseIfBranch {
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then_statements(&self) -> &[Statement] {
        &self.then_branch
    }
}

impl ElseBranch {
    pub fn then_statements(&self) -> &[Statement] {
        &self.then_branch
    }
}

impl Parsable for IfStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::If)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::If,
            "'if' keyword at the beginning of if statement"
        );

        let condition = expect_parse!(
            ts,
            Expression,
            "expression after 'if' keyword in if statement"
        );

        expect_token!(
            ts,
            TokenVariant::LeftBrace,
            "'{' after condition in if statement"
        );

        let mut then_branch = Vec::new();
        while !is_token!(ts, TokenVariant::RightBrace) {
            then_branch.push(Statement::parse(ts)?);
        }

        expect_token!(
            ts,
            TokenVariant::RightBrace,
            "'}' at the end of then branch in if statement"
        );

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;
        while is_token!(ts, TokenVariant::Else) {
            let else_pos = get_pos!(ts);
            ts.next(); // consume 'else' token

            if is_token!(ts, TokenVariant::If) {
                ts.next(); // consume 'if' token

                let else_if_condition = expect_parse!(
                    ts,
                    Expression,
                    "expression after 'if' keyword in else if branch"
                );

                expect_token!(
                    ts,
                    TokenVariant::LeftBrace,
                    "'{' after condition in else if branch"
                );

                let mut else_if_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::RightBrace) {
                    else_if_then_branch.push(Statement::parse(ts)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::RightBrace,
                    "'}' at the end of then branch in else if branch"
                );

                else_if_branches.push(ElseIfBranch {
                    condition: else_if_condition,
                    then_branch: else_if_then_branch,
                    pos: else_pos,
                });
            } else {
                expect_token!(
                    ts,
                    TokenVariant::LeftBrace,
                    "'{' after 'else' keyword in else branch"
                );

                let mut else_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::RightBrace) {
                    else_then_branch.push(Statement::parse(ts)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::RightBrace,
                    "'}' at the end of else branch"
                );

                else_branch = Some(ElseBranch {
                    then_branch: else_then_branch,
                    pos: else_pos,
                });
            }
        }

        Ok(IfStatement {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            pos,
        })
    }
}

impl Positioned for IfStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

impl Positioned for ElseBranch {
    fn position(&self) -> Position {
        self.pos
    }
}

impl Positioned for ElseIfBranch {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl IfStatement {
    pub fn new(
        condition: Expression,
        then_branch: Vec<Statement>,
        else_if_branches: Vec<ElseIfBranch>,
        else_branch: Option<ElseBranch>,
        line: usize,
        column: usize,
    ) -> Self {
        IfStatement {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            pos: Position::new(line, column),
        }
    }
}

#[cfg(test)]
impl ElseBranch {
    pub fn new(then_branch: Vec<Statement>, line: usize, column: usize) -> Self {
        ElseBranch {
            then_branch,
            pos: Position::new(line, column),
        }
    }
}

#[cfg(test)]
impl ElseIfBranch {
    pub fn new(
        condition: Expression,
        then_branch: Vec<Statement>,
        line: usize,
        column: usize,
    ) -> Self {
        ElseIfBranch {
            condition,
            then_branch,
            pos: Position::new(line, column),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::parser::abstract_syntax_tree::{
        expression::binary::BinaryOperator, statement::assignment::AssignmentStatement,
    };

    use super::*;

    #[test]
    fn test_plain_if_statement() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Greater,
            TokenVariant::FloatLiteral(0.0),
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::FloatLiteral(1.0),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 1, 0),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_float(0.0, 3, 0),
                    1,
                    0
                ),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "y",
                    Expression::new_atom_literal_float(1.0, 7, 0),
                    5,
                    0
                ))],
                vec![],
                None,
                0,
                0
            )
        );
    }

    #[test]
    fn test_empty_if_statement() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::BoolLiteral(true),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(true, 1, 0),
                vec![],
                vec![],
                None,
                0,
                0
            )
        );
    }

    #[test]
    fn test_if_else_statement() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Greater,
            TokenVariant::FloatLiteral(0.0),
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::FloatLiteral(1.0),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::FloatLiteral(-1.0),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 1, 0),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_float(0.0, 3, 0),
                    1,
                    0
                ),
                vec![Statement::new_assignment(
                    "y",
                    Expression::new_atom_literal_float(1.0, 7, 0),
                    5,
                    0
                )],
                vec![],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "y",
                        Expression::new_atom_literal_float(-1.0, 14, 0),
                        12,
                        0
                    )],
                    10,
                    0
                )),
                0,
                0
            )
        );
    }

    #[test]
    fn test_if_else_if_statement() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::BoolLiteral(false),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::If,
            TokenVariant::BoolLiteral(true),
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::IntLiteral(42),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 1, 0),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::new_atom_literal_bool(true, 6, 0),
                    vec![Statement::new_assignment(
                        "x",
                        Expression::new_atom_literal_int(42, 10, 0),
                        8,
                        0
                    )],
                    4,
                    0
                )],
                None,
                0,
                0
            )
        );
    }

    #[test]
    fn test_if_else_if_else_statement() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::BoolLiteral(false),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::If,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::FloatLiteral(-1.0),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 1, 0),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::new_atom_variable("x", 6, 0),
                    vec![],
                    4,
                    0
                )],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "y",
                        Expression::new_atom_literal_float(-1.0, 13, 0),
                        11,
                        0
                    )],
                    9,
                    0
                )),
                0,
                0
            )
        );
    }

    #[test]
    fn test_multiple_else_if_branches() {
        let mut ts = test_token_stream!(
            TokenVariant::If,
            TokenVariant::BoolLiteral(false),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::If,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::If,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace,
            TokenVariant::Else,
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("z".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::IntLiteral(42),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let if_stmt = IfStatement::parse(&mut ts).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 1, 0),
                vec![],
                vec![
                    ElseIfBranch::new(Expression::new_atom_variable("x", 6, 0), vec![], 4, 0),
                    ElseIfBranch::new(Expression::new_atom_variable("y", 11, 0), vec![], 9, 0)
                ],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "z",
                        Expression::new_atom_literal_int(42, 18, 0),
                        16,
                        0
                    )],
                    14,
                    0
                )),
                0,
                0
            )
        );
    }
}
