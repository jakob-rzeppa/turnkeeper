use crate::application::game::plugin::{lexer::token::Token, parser::abstract_syntax_tree::{Parse}};

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
    Atom(Literal),
}

impl Expr {

    fn parse_unary(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        let operator = match tokens.get(index) {
            Some(Token::Minus) => UnaryOperator::Neg,
            Some(Token::Not) => UnaryOperator::Not,
            _ => return Err("Expected unary operator after checking that there is one".to_string()),
        };
        index += 1; // consume operator

        if let Some(Token::LeftParen) = tokens.get(index) {
            // If the next token is a left parenthesis, we need to parse the entire parenthesized expression as the operand of the unary operator
            index += 1; // consume '('

            let (expr, new_index) = Self::pratt_parse(tokens, index, 0)?;
            index = new_index;

            expect_token!(tokens, index, Token::RightParen, "Expected ')' after parenthesized expression");

            return Ok((Expr::UnaryOperation { operator, operand: Box::new(expr) }, index));
        }

        // Expect the operand to be an atom or another unary operator (e.g. - -5)

        match tokens.get(index) {
            Some(Token::Minus) | Some(Token::Not) => {
                let (expr, new_index) = Self::parse_unary(tokens, index)?;
                index = new_index;
                return Ok((Expr::UnaryOperation { operator, operand: Box::new(expr) }, index));
            },
            _ => {
                let operand = expect_parse!(tokens, index, Literal, "Expected literal, another unary operator or '(' after unary operator");
                return Ok((Expr::UnaryOperation { operator, operand: Box::new(Expr::Atom(operand)) }, index));
            },
        }
    }

    /// Pratt parser for expressions with operator precedence and associativity
    /// 
    /// This function will parse expressions by their precedence. It will handle parentheses to override precedence and associativity. 
    /// It will also handle unary operators (highest precedence).
    fn pratt_parse(tokens: &[Token], mut index: usize, min_bp: u8) -> Result<(Self, usize), String> {
        let mut left = match tokens.get(index) {
            Some(Token::LeftParen) => {
                index += 1; // consume '('

                let (expr, new_index) = Self::pratt_parse(tokens, index, 0)?;
                index = new_index;

                expect_token!(tokens, index, Token::RightParen, "Expected ')' after parenthesized expression");

                expr
            },
            Some(Token::Minus) | Some(Token::Not) => {
                let (expr, new_index) = Self::parse_unary(tokens, index)?;
                index = new_index;
                expr
            },
            _ => Expr::Atom(expect_parse!(tokens, index, Literal, "Expected literal or '(' at beginning of expression")),
        };

        loop {
            let operator = match tokens.get(index) {
                Some(Token::Plus) => BinaryOperator::Plus,
                Some(Token::Minus) => BinaryOperator::Minus,
                Some(Token::Star) => BinaryOperator::Multiply,
                Some(Token::Slash) => BinaryOperator::Divide,
                Some(Token::Percent) => BinaryOperator::Modulo,
                Some(Token::Caret) => BinaryOperator::Power,
                Some(Token::EqualEqual) => BinaryOperator::Equal,
                Some(Token::NotEqual) => BinaryOperator::NotEqual,
                Some(Token::Less) => BinaryOperator::Less,
                Some(Token::Greater) => BinaryOperator::Greater,
                Some(Token::LessEqual) => BinaryOperator::LessEqual,
                Some(Token::GreaterEqual) => BinaryOperator::GreaterEqual,
                Some(Token::And) => BinaryOperator::And,
                Some(Token::Or) => BinaryOperator::Or,

                Some(Token::RightParen) => break, // Don't consume ')', let the caller handle it
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
        matches!(tokens.get(index), Some(Token::LeftParen) | Some(Token::Minus) | Some(Token::Not))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        Self::pratt_parse(tokens, index, 0)
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

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Neg, // negation, e.g. -5
    Not, // logical NOT, e.g. !true
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
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

            BinaryOperator::Plus | BinaryOperator::Minus => (80, 81),

            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => (90, 91),
            BinaryOperator::Power => (101, 100),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_operator() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(3))), operator: BinaryOperator::Plus, right: Box::new(Expr::Atom(Literal::Int(4))) });
    }

    #[test]
    fn test_star_operator() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(3))), operator: BinaryOperator::Multiply, right: Box::new(Expr::Atom(Literal::Int(4))) });
    }

    #[test]
    fn test_plus_then_star() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::Star,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(4))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(5))) 
            }) 
        });
    }

    #[test]
    fn test_star_then_plus() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
            Token::Plus,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_eval_first_plus_before_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::Plus,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_eval_first_minus_before_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Minus,
            Token::IntLiteral(4),
            Token::Minus,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Minus, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Minus, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_eval_first_multiply_before_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
            Token::Star,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Multiply, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_eval_first_divide_before_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Slash,
            Token::IntLiteral(4),
            Token::Slash,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Divide, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Divide, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_complex_expression() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::Star,
            Token::IntLiteral(5),
            Token::Minus,
            Token::IntLiteral(6),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::BinaryOperation { 
                    left: Box::new(Expr::Atom(Literal::Int(4))), 
                    operator: BinaryOperator::Multiply, 
                    right: Box::new(Expr::Atom(Literal::Int(5))) 
                })
            }), 
            operator: BinaryOperator::Minus, 
            right: Box::new(Expr::Atom(Literal::Int(6))) });
    }

    #[test]
    fn test_parenthesized_literal() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(42),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::Atom(Literal::Int(42)));
    }

    #[test]
    fn test_parenthesized_expression() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(4))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_higher_precedence_first() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::RightParen,
            Token::Star,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Multiply, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_higher_precedence_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::LeftParen,
            Token::IntLiteral(4),
            Token::Plus,
            Token::IntLiteral(5),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Multiply, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(4))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(5))) 
            }) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_lower_precedence_first() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
            Token::RightParen,
            Token::Plus,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_lower_precedence_last() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::LeftParen,
            Token::IntLiteral(4),
            Token::Star,
            Token::IntLiteral(5),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(4))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(5))) 
            }) 
        });
    }

    #[test]
    fn test_parenthesized_expression_with_nested_parentheses() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(3),
            Token::Star,
            Token::LeftParen,
            Token::IntLiteral(4),
            Token::Plus,
            Token::IntLiteral(5),
            Token::RightParen,
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Multiply, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(4))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(5))) 
            }) 
        });
    }

    #[test]
    fn test_complex_nested_parenthesized_expression() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::LeftParen,
            Token::IntLiteral(4),
            Token::Plus,
            Token::LeftParen,
            Token::IntLiteral(5),
            Token::Star,
            Token::IntLiteral(6),
            Token::RightParen,
            Token::Slash,
            Token::IntLiteral(6),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Multiply, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(4))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::BinaryOperation { 
                    left: Box::new(Expr::BinaryOperation { 
                        left: Box::new(Expr::Atom(Literal::Int(5))), 
                        operator: BinaryOperator::Multiply, 
                        right: Box::new(Expr::Atom(Literal::Int(6))) 
                    }), 
                    operator: BinaryOperator::Divide, 
                    right: Box::new(Expr::Atom(Literal::Int(6))) 
                }),
            }),
        });
    }

    #[test]
    fn test_unary_minus() {
        let tokens = vec![
            Token::Minus,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(Literal::Int(5))) });
    }

    #[test]
    fn test_unary_not() {
        let tokens = vec![
            Token::Not,
            Token::BoolLiteral(true),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { operator: UnaryOperator::Not, operand: Box::new(Expr::Atom(Literal::Bool(true))) });
    }

    #[test]
    fn test_unary_minus_with_binary_operation() {
        let tokens = vec![
            Token::Minus,
            Token::IntLiteral(5),
            Token::Plus,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(Literal::Int(5))) }), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(3))) 
        });
    }

    #[test]
    fn test_unary_not_with_binary_operation() {
        let tokens = vec![
            Token::Minus,
            Token::IntLiteral(1),
            Token::Plus,
            Token::IntLiteral(0),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(Literal::Int(1))) }), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::Atom(Literal::Int(0))) 
        });
    }

    #[test]
    fn test_unary_in_binary_operation() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::Minus,
            Token::IntLiteral(4),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(3))), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::UnaryOperation { operator: UnaryOperator::Neg, operand: Box::new(Expr::Atom(Literal::Int(4))) }) 
        });
    }

    #[test]
    fn test_multiple_unary_operators() {
        let tokens = vec![
            Token::Not,
            Token::Not,
            Token::BoolLiteral(true),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::UnaryOperation { 
                operator: UnaryOperator::Not, 
                operand: Box::new(Expr::Atom(Literal::Bool(true))) 
            }) 
        });
    }

    #[test]
    fn test_unary_operators_with_parentheses() {
        let tokens = vec![
            Token::Not,
            Token::LeftParen,
            Token::BoolLiteral(true),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::Atom(Literal::Bool(true))) 
        });
    }

    #[test]
    fn test_unary_operators_with_expr_in_parentheses() {
        let tokens = vec![
            Token::Minus,
            Token::LeftParen,
            Token::IntLiteral(5),
            Token::Plus,
            Token::IntLiteral(3),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Neg, 
            operand: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(5))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(3))) 
            }) 
        });
    }

    #[test]
    fn test_unary_operators_with_nested_parentheses() {
        let tokens = vec![
            Token::Not,
            Token::LeftParen,
            Token::Not,
            Token::LeftParen,
            Token::BoolLiteral(true),
            Token::RightParen,
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::UnaryOperation { 
            operator: UnaryOperator::Not, 
            operand: Box::new(Expr::UnaryOperation { 
                operator: UnaryOperator::Not, 
                operand: Box::new(Expr::Atom(Literal::Bool(true))) 
            }) 
        });
    }

    #[test]
    fn test_expr_end_is_at_semicolon() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::Semicolon,
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, index) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(3))), operator: BinaryOperator::Plus, right: Box::new(Expr::Atom(Literal::Int(4))) });
        assert_eq!(index, 3); // The expression should end at the semicolon
    }

    #[test]
    fn test_expr_end_is_at_right_paren() {
        let tokens = vec![
            Token::LeftParen,
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::RightParen,
        ];

        assert!(Expr::is_next(&tokens, 1));
        let (expr, index) = Expr::parse(&tokens, 1).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(3))), operator: BinaryOperator::Plus, right: Box::new(Expr::Atom(Literal::Int(4))) });
        assert_eq!(index, 4); // The expression should end at the right parenthesis
    }

    #[test]
    fn test_modulo_operator() {
        let tokens = vec![
            Token::IntLiteral(10),
            Token::Percent,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(10))), operator: BinaryOperator::Modulo, right: Box::new(Expr::Atom(Literal::Int(3))) });
    }

    #[test]
    fn test_power_operator() {
        let tokens = vec![
            Token::IntLiteral(2),
            Token::Caret,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(2))), operator: BinaryOperator::Power, right: Box::new(Expr::Atom(Literal::Int(3))) });
    }

    #[test]
    fn test_power_is_right_associative() {
        let tokens = vec![
            Token::IntLiteral(2),
            Token::Caret,
            Token::IntLiteral(3),
            Token::Caret,
            Token::IntLiteral(2),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        // Power should be right-associative: 2^3^2 = 2^(3^2)
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(2))), 
            operator: BinaryOperator::Power, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Power, 
                right: Box::new(Expr::Atom(Literal::Int(2))) 
            }) 
        });
    }

    #[test]
    fn test_multiply_then_modulo() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
            Token::Percent,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Multiply, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Modulo, 
            right: Box::new(Expr::Atom(Literal::Int(5))) 
        });
    }

    #[test]
    fn test_equal_operator() {
        let tokens = vec![
            Token::IntLiteral(5),
            Token::EqualEqual,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(5))), operator: BinaryOperator::Equal, right: Box::new(Expr::Atom(Literal::Int(5))) });
    }

    #[test]
    fn test_not_equal_operator() {
        let tokens = vec![
            Token::IntLiteral(5),
            Token::NotEqual,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(5))), operator: BinaryOperator::NotEqual, right: Box::new(Expr::Atom(Literal::Int(3))) });
    }

    #[test]
    fn test_less_operator() {
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Less,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(3))), operator: BinaryOperator::Less, right: Box::new(Expr::Atom(Literal::Int(5))) });
    }

    #[test]
    fn test_greater_operator() {
        let tokens = vec![
            Token::IntLiteral(5),
            Token::Greater,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(5))), operator: BinaryOperator::Greater, right: Box::new(Expr::Atom(Literal::Int(3))) });
    }

    #[test]
    fn test_less_equal_operator() {
        let tokens = vec![
            Token::IntLiteral(5),
            Token::LessEqual,
            Token::IntLiteral(5),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(5))), operator: BinaryOperator::LessEqual, right: Box::new(Expr::Atom(Literal::Int(5))) });
    }

    #[test]
    fn test_greater_equal_operator() {
        let tokens = vec![
            Token::IntLiteral(5),
            Token::GreaterEqual,
            Token::IntLiteral(3),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Int(5))), operator: BinaryOperator::GreaterEqual, right: Box::new(Expr::Atom(Literal::Int(3))) });
    }

    #[test]
    fn test_and_operator() {
        let tokens = vec![
            Token::BoolLiteral(true),
            Token::And,
            Token::BoolLiteral(false),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Bool(true))), operator: BinaryOperator::And, right: Box::new(Expr::Atom(Literal::Bool(false))) });
    }

    #[test]
    fn test_or_operator() {
        let tokens = vec![
            Token::BoolLiteral(false),
            Token::Or,
            Token::BoolLiteral(true),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { left: Box::new(Expr::Atom(Literal::Bool(false))), operator: BinaryOperator::Or, right: Box::new(Expr::Atom(Literal::Bool(true))) });
    }

    #[test]
    fn test_and_or_precedence() {
        // OR has lower precedence than AND, so: false && true || true => (false && true) || true
        let tokens = vec![
            Token::BoolLiteral(false),
            Token::And,
            Token::BoolLiteral(true),
            Token::Or,
            Token::BoolLiteral(true),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Bool(false))), 
                operator: BinaryOperator::And, 
                right: Box::new(Expr::Atom(Literal::Bool(true))) 
            }), 
            operator: BinaryOperator::Or, 
            right: Box::new(Expr::Atom(Literal::Bool(true))) 
        });
    }

    #[test]
    fn test_comparison_lower_precedence_than_arithmetic() {
        // 3 + 4 == 7 should be (3 + 4) == 7
        let tokens = vec![
            Token::IntLiteral(3),
            Token::Plus,
            Token::IntLiteral(4),
            Token::EqualEqual,
            Token::IntLiteral(7),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Plus, 
                right: Box::new(Expr::Atom(Literal::Int(4))) 
            }), 
            operator: BinaryOperator::Equal, 
            right: Box::new(Expr::Atom(Literal::Int(7))) 
        });
    }

    #[test]
    fn test_arithmetic_lower_precedence_than_power() {
        // 2 + 3 ** 2 should be 2 + (3 ** 2)
        let tokens = vec![
            Token::IntLiteral(2),
            Token::Plus,
            Token::IntLiteral(3),
            Token::Caret,
            Token::IntLiteral(2),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation { 
            left: Box::new(Expr::Atom(Literal::Int(2))), 
            operator: BinaryOperator::Plus, 
            right: Box::new(Expr::BinaryOperation { 
                left: Box::new(Expr::Atom(Literal::Int(3))), 
                operator: BinaryOperator::Power, 
                right: Box::new(Expr::Atom(Literal::Int(2))) 
            }) 
        });
    }

    #[test]
    fn test_complex_mixed_operators() {
        // -3 * 4 + 5 < 10 && true
        let tokens = vec![
            Token::Minus,
            Token::IntLiteral(3),
            Token::Star,
            Token::IntLiteral(4),
            Token::Plus,
            Token::IntLiteral(5),
            Token::Less,
            Token::IntLiteral(10),
            Token::And,
            Token::BoolLiteral(true),
        ];

        assert!(Expr::is_next(&tokens, 0));
        let (expr, _) = Expr::parse(&tokens, 0).unwrap();
        assert_eq!(expr, Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::BinaryOperation {
                    left: Box::new(Expr::BinaryOperation {
                        left: Box::new(Expr::UnaryOperation {
                            operator: UnaryOperator::Neg,
                            operand: Box::new(Expr::Atom(Literal::Int(3)))
                        }),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(Expr::Atom(Literal::Int(4)))
                    }),
                    operator: BinaryOperator::Plus,
                    right: Box::new(Expr::Atom(Literal::Int(5)))
                }),
                operator: BinaryOperator::Less,
                right: Box::new(Expr::Atom(Literal::Int(10)))
            }), 
            operator: BinaryOperator::And, 
            right: Box::new(Expr::Atom(Literal::Bool(true))) 
        });
    }
}