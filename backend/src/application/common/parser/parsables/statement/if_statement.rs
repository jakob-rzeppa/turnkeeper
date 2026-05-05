use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable, parsables::{expression::Expression, statement::Statement}}, domain::common::position::{Position, Positioned}};

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

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::If,
            "Expected 'if' keyword at the beginning of if statement"
        );

        let condition = Expression::parse(ts, source_code).map_err(|err|
            change_err_msg!(err, "Expected expression in if statement condition")
        )?;

        expect_token!(
            ts,
            TokenVariant::OpenBrace,
            "Expected '{' after condition in if statement"
        );

        let mut then_branch = Vec::new();
        while !is_token!(ts, TokenVariant::CloseBrace) {
            then_branch.push(Statement::parse(ts, source_code)?);
        }

        expect_token!(
            ts,
            TokenVariant::CloseBrace,
            "Expected '}' at the end of then branch in if statement"
        );

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;
        while is_token!(ts, TokenVariant::Else) {
            let else_pos = get_pos!(ts);
            ts.next(); // consume 'else' token

            if is_token!(ts, TokenVariant::If) {
                ts.next(); // consume 'if' token

                let else_if_condition = Expression::parse(ts, source_code).map_err(|err|
                    change_err_msg!(err, "Expected expression in else if branch condition")
                )?;

                expect_token!(
                    ts,
                    TokenVariant::OpenBrace,
                    "Expected '{' after condition in else if branch"
                );

                let mut else_if_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::CloseBrace) {
                    else_if_then_branch.push(Statement::parse(ts, source_code)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::CloseBrace,
                    "Expected '}' at the end of then branch in else if branch"
                );

                else_if_branches.push(ElseIfBranch {
                    condition: else_if_condition,
                    then_branch: else_if_then_branch,
                    pos: else_pos,
                });
            } else {
                expect_token!(
                    ts,
                    TokenVariant::OpenBrace,
                    "Expected '{' after 'else' keyword in else branch"
                );

                let mut else_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::CloseBrace) {
                    else_then_branch.push(Statement::parse(ts, source_code)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::CloseBrace,
                    "Expected '}' at the end of else branch"
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

    use crate::application::common::parser::{macros::test_token_stream, parsables::{expression::binary::BinaryOperator, statement::assignment::AssignmentStatement}};

use super::*;

    #[test]
    fn test_plain_if_statement() {
        let (mut ts, source_code) = test_token_stream!("if x > 0.0 { y = 1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 3),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_float(0.0, 0, 7),
                    0,
                    3
                ),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "y",
                    Expression::new_atom_literal_float(1.0, 0, 17),
                    0,
                    13
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
        let (mut ts, source_code) = test_token_stream!("if true { }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(true, 0, 3),
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
        let (mut ts, source_code) = test_token_stream!("if x > 0.0 { y = 1.0; } else { y = -1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 3),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_float(0.0, 0, 7),
                    0,
                    3
                ),
                vec![Statement::new_assignment(
                    "y",
                    Expression::new_atom_literal_float(1.0, 0, 17),
                    0,
                    13
                )],
                vec![],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "y",
                        Expression::new_unary_negation(Expression::new_atom_literal_float(1.0, 0, 36), 0, 35),
                        0,
                        31
                    )],
                    0,
                    24
                )),
                0,
                0
            )
        );
    }

    #[test]
    fn test_if_else_if_statement() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if true { x = 42; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 0, 3),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::new_atom_literal_bool(true, 0, 21),
                    vec![Statement::new_assignment(
                        "x",
                        Expression::new_atom_literal_int(42, 0, 32),
                        0,
                        28
                    )],
                    0,
                    13
                )],
                None,
                0,
                0
            )
        );
    }

    #[test]
    fn test_if_else_if_else_statement() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if x { } else { y = -1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 0, 3),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::new_atom_variable("x", 0, 21),
                    vec![],
                    0,
                    13
                )],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "y",
                        Expression::new_unary_negation(Expression::new_atom_literal_float(1.0, 0, 39), 0, 38),
                        0,
                        34
                    )],
                    0,
                    27
                )),
                0,
                0
            )
        );
    }

    #[test]
    fn test_multiple_else_if_branches() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if x { } else if y { } else { z = 42; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::new_atom_literal_bool(false, 0, 3),
                vec![],
                vec![
                    ElseIfBranch::new(Expression::new_atom_variable("x", 0, 21), vec![], 0, 13),
                    ElseIfBranch::new(Expression::new_atom_variable("y", 0, 35), vec![], 0, 27)
                ],
                Some(ElseBranch::new(
                    vec![Statement::new_assignment(
                        "z",
                        Expression::new_atom_literal_int(42, 0, 52),
                        0,
                        48
                    )],
                    0,
                    41
                )),
                0,
                0
            )
        );
    }
}
