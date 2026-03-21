use crate::application::game::plugin::{lexer::token::Token, parser::abstract_syntax_tree::{Parse}};

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    // UnaryOperation {
    //     operator: UnaryOperator,
    //     operand: Box<Expr>,
    // },
    // BinaryOperation {
    //     left: Box<Expr>,
    //     operator: BinaryOperator,
    //     right: Box<Expr>,
    // },
    // Parenthesized(Box<Expr>),
    Literal(Literal),
}

impl Parse for Expr {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        Literal::is_next(tokens, index)
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        Literal::parse(tokens, index).map(|(value, new_index)| (Expr::Literal(value), new_index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Parse for Literal {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(Token::IntLiteral(_) | Token::FloatLiteral(_) | Token::StringLiteral(_) | Token::BoolLiteral(_)))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index) {
            Some(Token::IntLiteral(value)) => Ok((Literal::Int(*value), index + 1)),
            Some(Token::FloatLiteral(value)) => Ok((Literal::Float(*value), index + 1)),
            Some(Token::StringLiteral(value)) => Ok((Literal::String(value.clone()), index + 1)),
            Some(Token::BoolLiteral(value)) => Ok((Literal::Bool(*value), index + 1)),
            _ => Err("Expected literal".to_string()),
        }
    }
}