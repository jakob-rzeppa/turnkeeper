use crate::application::game::plugin::{lexer::token::{Token, TokenType}, parser::abstract_syntax_tree::{Parse, common::{Block, Identifier}}};

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    BinaryOperation {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Atom(ExprAtom),
}

impl Expr {
    fn parse_unary(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        let operator = match tokens.get(index) {
            Some(t) if t.token == TokenType::Minus => UnaryOperator::Neg,
            Some(t) if t.token == TokenType::Not => UnaryOperator::Not,
            _ => return Err("Expected unary operator after checking that there is one".to_string()),
        };
        index += 1; // consume operator

        if let Some(t) = tokens.get(index) {
            if t.token == TokenType::LeftParen {
                // If the next token is a left parenthesis, we need to parse the entire parenthesized expression as the operand of the unary operator
                index += 1; // consume '('

                let (expr, new_index) = Self::pratt_parse(tokens, index, 0)?;
                index = new_index;

                expect_token!(tokens, index, TokenType::RightParen, "Expected ')' after parenthesized expression");

                return Ok((Expr::UnaryOperation { operator, operand: Box::new(expr) }, index));
            }
        }

        // Expect the operand to be an atom or another unary operator (e.g. - -5)

        match tokens.get(index) {
            Some(t) if matches!(t.token, TokenType::Minus | TokenType::Not) => {
                let (expr, new_index) = Self::parse_unary(tokens, index)?;
                index = new_index;
                return Ok((Expr::UnaryOperation { operator, operand: Box::new(expr) }, index));
            },
            _ => {
                let operand = expect_parse!(tokens, index, Literal, "Expected literal, another unary operator or '(' after unary operator");
                return Ok((Expr::UnaryOperation { operator, operand: Box::new(Expr::Atom(ExprAtom::Literal(operand))) }, index));
            },
        }
    }

    /// Pratt parser for expressions with operator precedence and associativity
    /// 
    /// This function will parse expressions by their precedence. It will handle parentheses to override precedence and associativity. 
    /// It will also handle unary operators (highest precedence).
    fn pratt_parse(tokens: &[Token], mut index: usize, min_bp: u8) -> Result<(Self, usize), String> {
        let mut left = match tokens.get(index) {
            Some(t) if t.token == TokenType::LeftParen => {
                index += 1; // consume '('

                let (expr, new_index) = Self::pratt_parse(tokens, index, 0)?;
                index = new_index;

                expect_token!(tokens, index, TokenType::RightParen, "Expected ')' after parenthesized expression");

                expr
            },
            Some(t) if matches!(t.token, TokenType::Minus | TokenType::Not) => {
                let (expr, new_index) = Self::parse_unary(tokens, index)?;
                index = new_index;
                expr
            },
            _ => {
                if Literal::is_next(tokens, index) {
                    Expr::Atom(ExprAtom::Literal(expect_parse!(tokens, index, Literal, "Expected literal after checking that there is one")))
                } else if FunctionCall::is_next(tokens, index) {
                    Expr::Atom(ExprAtom::FunctionCall(expect_parse!(tokens, index, FunctionCall, "Expected function call after checking that there is one")))
                } else if Identifier::is_next(tokens, index) {
                    Expr::Atom(ExprAtom::Identifier(expect_parse!(tokens, index, Identifier, "Expected identifier after checking that there is one")))
                } else {
                    return Err("Expected expression atom".to_string());
                }
            },
        };

        loop {
            let operator = match tokens.get(index) {
                Some(t) if t.token == TokenType::Plus => BinaryOperator::Addition,
                Some(t) if t.token == TokenType::Minus => BinaryOperator::Subtraction,
                Some(t) if t.token == TokenType::Star => BinaryOperator::Multiplication,
                Some(t) if t.token == TokenType::Slash => BinaryOperator::Division,
                Some(t) if t.token == TokenType::Percent => BinaryOperator::Modulo,
                Some(t) if t.token == TokenType::Caret => BinaryOperator::Power,
                Some(t) if t.token == TokenType::EqualEqual => BinaryOperator::Equal,
                Some(t) if t.token == TokenType::NotEqual => BinaryOperator::NotEqual,
                Some(t) if t.token == TokenType::Less => BinaryOperator::Less,
                Some(t) if t.token == TokenType::Greater => BinaryOperator::Greater,
                Some(t) if t.token == TokenType::LessEqual => BinaryOperator::LessEqual,
                Some(t) if t.token == TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                Some(t) if t.token == TokenType::And => BinaryOperator::And,
                Some(t) if t.token == TokenType::Or => BinaryOperator::Or,

                Some(t) if t.token == TokenType::RightParen => break, // Don't consume ')', let the caller handle it
                _ => break,
            };

            let (l_bp, r_bp) = operator.binding_power();
            if l_bp < min_bp {
                break;
            }
            index += 1; // consume operator

            let (right, new_index) = Self::pratt_parse(tokens, index, r_bp)?;
            index = new_index;

            left = Expr::BinaryOperation { left: Box::new(left), operator, right: Box::new(right) };
        }

        Ok((left, index))
    }
}

impl Parse for Expr {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        Literal::is_next(tokens, index) || 
        FunctionCall::is_next(tokens, index) ||
        Identifier::is_next(tokens, index) ||
        matches!(tokens.get(index), Some(t) if matches!(t.token, TokenType::LeftParen | TokenType::Minus | TokenType::Not))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        Self::pratt_parse(tokens, index, 0)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExprAtom {
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
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
        matches!(tokens.get(index), Some(t) if matches!(t.token, TokenType::IntLiteral(_) | TokenType::FloatLiteral(_) | TokenType::StringLiteral(_) | TokenType::BoolLiteral(_)))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index).map(|t| &t.token) {
            Some(TokenType::IntLiteral(value)) => Ok((Literal::Int(*value), index + 1)),
            Some(TokenType::FloatLiteral(value)) => Ok((Literal::Float(*value), index + 1)),
            Some(TokenType::StringLiteral(value)) => Ok((Literal::String(value.clone()), index + 1)),
            Some(TokenType::BoolLiteral(value)) => Ok((Literal::Bool(*value), index + 1)),
            _ => Err("Expected literal".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub arguments: Vec<Expr>,
    pub catch_block: Option<Block>,
}

impl Parse for FunctionCall {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        Identifier::is_next(tokens, index) && 
        matches!(tokens.get(index + 1), Some(t) if t.token == TokenType::LeftParen)
    }

    fn parse(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        let identifier = expect_parse!(tokens, index, Identifier, "Expected function name at beginning of function call");

        expect_token!(tokens, index, TokenType::LeftParen, "Expected '(' after function name in function call");

        let mut arguments = Vec::new();
        if !matches!(tokens.get(index), Some(t) if t.token == TokenType::RightParen) {
            loop {
                let arg = expect_parse!(tokens, index, Expr, "Expected expression as function argument");
                arguments.push(arg);

                // If there is a comma, consume it and continue parsing the next argument. Otherwise, break the loop
                if matches!(tokens.get(index), Some(t) if t.token == TokenType::Comma) {
                    index += 1; // consume ','
                } else {
                    break;
                }
            }
        }

        // After the arguments, we should have a right parenthesis. Consume it!
        expect_token!(tokens, index, TokenType::RightParen, "Expected ')' after function arguments in function call");

        let catch_block = if matches!(tokens.get(index), Some(t) if t.token == TokenType::Catch) {
            index += 1; // consume 'catch'
            let (block, new_index) = Block::parse(tokens, index)?;
            index = new_index;
            Some(block)
        } else {
            None
        };

        Ok((FunctionCall { identifier, arguments, catch_block }, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Neg, // negation, e.g. -5
    Not, // logical NOT, e.g. !true
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Power,

    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    And,
    Or,
}

impl BinaryOperator {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            BinaryOperator::Or => (50, 51),
            BinaryOperator::And => (60, 61),
            
            BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::Less | BinaryOperator::Greater | BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => (70, 71),

            BinaryOperator::Addition | BinaryOperator::Subtraction => (80, 81),

            BinaryOperator::Multiplication | BinaryOperator::Division | BinaryOperator::Modulo => (90, 91),
            BinaryOperator::Power => (101, 100),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::parser::abstract_syntax_tree::statement::{ReturnStatement, Statement};

    use super::*;

    #[test]
    fn test_plus_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
        });
    }

    #[test]
    fn test_star_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), operator: BinaryOperator::Multiplication, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) });
    }

    #[test]
    fn test_plus_then_star() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
            }) 
        });
    }

    #[test]
    fn test_star_then_plus() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_eval_first_plus_before_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_eval_first_minus_before_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Subtraction, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Subtraction, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_eval_first_multiply_before_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_eval_first_divide_before_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Slash, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Slash, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Division, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Division, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_complex_expression() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(6), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::BinaryOperation { 
                    left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                    operator: BinaryOperator::Multiplication, 
                    right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
                })
            }), 
            operator: BinaryOperator::Subtraction, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(6)))) 
        });
    }

    #[test]
    fn test_parenthesized_literal() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(42), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::Atom(ExprAtom::Literal(Literal::Int(42))));
    }

    #[test]
    fn test_parenthesized_expression() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_higher_precedence_first() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_higher_precedence_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
            }) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_lower_precedence_first() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_lower_precedence_last() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
            }) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_nested_parentheses() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
            }) 
        });
    }

    #[test]
    fn test_complex_nested_parenthesized_expression() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(6), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::Slash, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(6), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::BinaryOperation { 
                    left: Box::new(Expr::BinaryOperation { 
                        left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), 
                        operator: BinaryOperator::Multiplication, 
                        right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(6)))) 
                    }), 
                    operator: BinaryOperator::Division, 
                    right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(6)))) 
                }),
            }),
        });
    }

    #[test]
    fn test_unary_minus() {
        let tokens = vec![
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) });
    }

    #[test]
    fn test_unary_not() {
        let tokens = vec![
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { operator: UnaryOperator::Not, operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) });
    }

    #[test]
    fn test_unary_minus_with_binary_operation() {
        let tokens = vec![
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) }), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) 
        });
    }

    #[test]
    fn test_unary_not_with_binary_operation() {
        let tokens = vec![
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(1), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(0), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1)))) }), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(0)))) 
        });
    }

    #[test]
    fn test_unary_in_binary_operation() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) }) 
        });
    }

    #[test]
    fn test_multiple_unary_operators() {
        let tokens = vec![
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::UnaryOperation { 
                operator: UnaryOperator::Not, 
                operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
            }) 
        });
    }

    #[test]
    fn test_unary_operators_with_parentheses() {
        let tokens = vec![
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
        });
    }

    #[test]
    fn test_unary_operators_with_expr_in_parentheses() {
        let tokens = vec![
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Neg, 
            operand: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) 
            }) 
        });
    }

    #[test]
    fn test_unary_operators_with_nested_parentheses() {
        let tokens = vec![
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::Not, line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::UnaryOperation { 
                operator: UnaryOperator::Not, 
                operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
            }) 
        });
    }

    #[test]
    fn test_expr_end_is_at_semicolon() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Semicolon, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, index) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), operator: BinaryOperator::Addition, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) });
        assert_eq!(index, 3); // The expression should end at the semicolon
    }

    #[test]
    fn test_expr_end_is_at_right_paren() {
        let tokens = vec![
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 1));
        let (expr, index) = Expr::parse(&tokens, 1).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), operator: BinaryOperator::Addition, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) });
        assert_eq!(index, 4); // The expression should end at the right parenthesis
    }

    #[test]
    fn test_modulo_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(10), line: 0, first_char: 0 },
            Token { token: TokenType::Percent, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))), operator: BinaryOperator::Modulo, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) });
    }

    #[test]
    fn test_power_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(2), line: 0, first_char: 0 },
            Token { token: TokenType::Caret, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))), operator: BinaryOperator::Power, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) });
    }

    #[test]
    fn test_power_is_right_associative() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(2), line: 0, first_char: 0 },
            Token { token: TokenType::Caret, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Caret, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(2), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        // Power should be right-associative: 2^3^2 = 2^(3^2)
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))), 
            operator: BinaryOperator::Power, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Power, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))) 
            }) 
        });
    }

    #[test]
    fn test_multiply_then_modulo() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Percent, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Multiplication, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Modulo, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_equal_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::EqualEqual, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), operator: BinaryOperator::Equal, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) });
    }

    #[test]
    fn test_not_equal_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::NotEqual, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), operator: BinaryOperator::NotEqual, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) });
    }

    #[test]
    fn test_less_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Less, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), operator: BinaryOperator::Less, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) });
    }

    #[test]
    fn test_greater_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Greater, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), operator: BinaryOperator::Greater, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) });
    }

    #[test]
    fn test_less_equal_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::LessEqual, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), operator: BinaryOperator::LessEqual, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) });
    }

    #[test]
    fn test_greater_equal_operator() {
        let tokens = vec![
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::GreaterEqual, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))), operator: BinaryOperator::GreaterEqual, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))) });
    }

    #[test]
    fn test_and_operator() {
        let tokens = vec![
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
            Token { token: TokenType::And, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(false), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))), operator: BinaryOperator::And, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))) });
    }

    #[test]
    fn test_or_operator() {
        let tokens = vec![
            Token { token: TokenType::BoolLiteral(false), line: 0, first_char: 0 },
            Token { token: TokenType::Or, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))), operator: BinaryOperator::Or, right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) });
    }

    #[test]
    fn test_and_or_precedence() {
        // OR has lower precedence than AND, so: false && true || true => (false && true) || true
        let tokens = vec![
            Token { token: TokenType::BoolLiteral(false), line: 0, first_char: 0 },
            Token { token: TokenType::And, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
            Token { token: TokenType::Or, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))), 
                operator: BinaryOperator::And, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
            }), 
            operator: BinaryOperator::Or, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
        });
    }

    #[test]
    fn test_comparison_lower_precedence_than_arithmetic() {
        // 3 + 4 == 7 should be (3 + 4) == 7
        let tokens = vec![
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::EqualEqual, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(7), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Addition, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
            }), 
            operator: BinaryOperator::Equal, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(7)))) 
        });
    }

    #[test]
    fn test_arithmetic_lower_precedence_than_power() {
        // 2 + 3 ** 2 should be 2 + (3 ** 2)
        let tokens = vec![
            Token { token: TokenType::IntLiteral(2), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Caret, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(2), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))), 
                operator: BinaryOperator::Power, 
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))) 
            }) 
        });
    }

    #[test]
    fn test_complex_mixed_operators() {
        // -3 * 4 + 5 < 10 && true
        let tokens = vec![
            Token { token: TokenType::Minus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
            Token { token: TokenType::Less, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(10), line: 0, first_char: 0 },
            Token { token: TokenType::And, line: 0, first_char: 0 },
            Token { token: TokenType::BoolLiteral(true), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::BinaryOperation {
                    left: Box::new(Expr::BinaryOperation {
                        left: Box::new(Expr::UnaryOperation {
                            operator: UnaryOperator::Neg,
                            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3))))
                        }),
                        operator: BinaryOperator::Multiplication,
                        right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4))))
                    }),
                    operator: BinaryOperator::Addition,
                    right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5))))
                }),
                operator: BinaryOperator::Less,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10))))
            }), 
            operator: BinaryOperator::And, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))) 
        });
    }

    #[test]
    fn test_expr_with_variable() {
        let tokens = vec![
            Token { token: TokenType::Identifier("x".to_string()), line: 0, first_char: 0 },
            Token { token: TokenType::Plus, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(5), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))), 
            operator: BinaryOperator::Addition, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))) 
        });
    }

    #[test]
    fn test_expr_with_function_call() {
        let tokens = vec![
            Token { token: TokenType::Identifier("f".to_string()), line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::FunctionCall(FunctionCall { 
                identifier: Identifier("f".to_string()),
                arguments: vec![Expr::Atom(ExprAtom::Literal(Literal::Int(3)))],
                catch_block: None,
            }))), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
        });
    }

    #[test]
    fn test_expr_with_nested_function_calls() {
        let tokens = vec![
            Token { token: TokenType::Identifier("f".to_string()), line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::Identifier("g".to_string()), line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::Atom(ExprAtom::FunctionCall(FunctionCall { 
            identifier: Identifier("f".to_string()),
            arguments: vec![Expr::Atom(ExprAtom::FunctionCall(FunctionCall { 
                identifier: Identifier("g".to_string()),
                arguments: vec![Expr::Atom(ExprAtom::Literal(Literal::Int(3)))],
                catch_block: None,
            }))],
            catch_block: None,
        })));
    }

    #[test]
    fn test_expr_with_function_call_and_catch_block() {
        let tokens = vec![
            Token { token: TokenType::Identifier("f".to_string()), line: 0, first_char: 0 },
            Token { token: TokenType::LeftParen, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(3), line: 0, first_char: 0 },
            Token { token: TokenType::RightParen, line: 0, first_char: 0 },
            Token { token: TokenType::Catch, line: 0, first_char: 0 },
            Token { token: TokenType::LeftBrace, line: 0, first_char: 0 },
            Token { token: TokenType::Return, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(0), line: 0, first_char: 0 },
            Token { token: TokenType::Semicolon, line: 0, first_char: 0 },
            Token { token: TokenType::RightBrace, line: 0, first_char: 0 },
            Token { token: TokenType::Star, line: 0, first_char: 0 },
            Token { token: TokenType::IntLiteral(4), line: 0, first_char: 0 },
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(ExprAtom::FunctionCall(FunctionCall { 
                identifier: Identifier("f".to_string()),
                arguments: vec![Expr::Atom(ExprAtom::Literal(Literal::Int(3)))],
                catch_block: Some(Block(vec![Statement::Return(ReturnStatement(Some(Expr::Atom(ExprAtom::Literal(Literal::Int(0))))))])),
            }))), 
            operator: BinaryOperator::Multiplication, 
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))) 
        });
    }
}