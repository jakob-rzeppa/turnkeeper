use crate::application::game::plugin::{lexer::token::TokenType, parser::abstract_syntax_tree::{Parse, common::Block}};

use super::{common::Type, common::Identifier, expression::Expr};

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    Expression(ExprStatement),
    If(IfStatement),
    While(WhileStatement),
    Return(ReturnStatement), // Return statement with an optional expression
    Throw(ThrowStatement),
    Exit(ExitStatement),
}

impl Parse for Statement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        VariableDeclaration::is_next(tokens, index) ||
        Assignment::is_next(tokens, index) ||
        ExprStatement::is_next(tokens, index) ||
        IfStatement::is_next(tokens, index) ||
        WhileStatement::is_next(tokens, index) ||
        ReturnStatement::is_next(tokens, index) ||
        ThrowStatement::is_next(tokens, index) ||
        ExitStatement::is_next(tokens, index)
    }

    fn parse(tokens: &[TokenType], index: usize) -> Result<(Self, usize), String> {
        if VariableDeclaration::is_next(tokens, index) {
            VariableDeclaration::parse(tokens, index).map(|(decl, new_index)| (Statement::VariableDeclaration(decl), new_index))
        } else if Assignment::is_next(tokens, index) {
            Assignment::parse(tokens, index).map(|(assign, new_index)| (Statement::Assignment(assign), new_index))
        } else if IfStatement::is_next(tokens, index) {
            IfStatement::parse(tokens, index).map(|(if_stmt, new_index)| (Statement::If(if_stmt), new_index))
        } else if WhileStatement::is_next(tokens, index) {
            WhileStatement::parse(tokens, index).map(|(while_stmt, new_index)| (Statement::While(while_stmt), new_index))
        } else if ReturnStatement::is_next(tokens, index) {
            ReturnStatement::parse(tokens, index).map(|(return_stmt, new_index)| (Statement::Return(return_stmt), new_index))
        } else if ThrowStatement::is_next(tokens, index) {
            ThrowStatement::parse(tokens, index).map(|(throw_stmt, new_index)| (Statement::Throw(throw_stmt), new_index))
        } else if ExitStatement::is_next(tokens, index) {
            ExitStatement::parse(tokens, index).map(|(exit_stmt, new_index)| (Statement::Exit(exit_stmt), new_index))
        } else if ExprStatement::is_next(tokens, index) { // Needs to be last because it's the most general
            ExprStatement::parse(tokens, index).map(|(expr_stmt, new_index)| (Statement::Expression(expr_stmt), new_index))
        } else {
            Err("Expected a statement".to_string())
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub datatype: Type,
    pub value: Expr,
}

impl Parse for VariableDeclaration {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::Let))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::Let, "Expected 'let' at the beginning of variable declaration");

        let name = expect_parse!(tokens, index, Identifier, "Expected variable name after 'let'");

        expect_token!(tokens, index, TokenType::Colon, "Expected ':' after variable name in 'let' statement");

        let datatype = expect_parse!(tokens, index, Type, "Expected type after ':' in 'let' statement");

        expect_token!(tokens, index, TokenType::Assign, "Expected '=' after type in 'let' statement");

        let value = expect_parse!(tokens, index, Expr, "Expected expression after '=' in 'let' statement");

        expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' at the end of 'let' statement");

        Ok((VariableDeclaration { name, datatype, value }, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Assignment {
    pub target: Identifier,
    pub value: Expr,
}

impl Parse for Assignment {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::Identifier(_))) && tokens.get(index + 1) == Some(&TokenType::Assign)
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        let target = expect_parse!(tokens, index, Identifier, "Expected identifier on the left side of assignment");

        expect_token!(tokens, index, TokenType::Assign, "Expected '=' in assignment statement");

        let value = expect_parse!(tokens, index, Expr, "Expected expression on the right side of assignment");

        expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' at the end of assignment statement");

        Ok((Assignment { target, value }, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ExprStatement(pub Expr);

impl Parse for ExprStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        Expr::is_next(tokens, index)
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        let expr = expect_parse!(tokens, index, Expr, "Expected expression in expression statement");

        expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' at the end of expression statement");

        Ok((ExprStatement(expr), index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub then: Block,
}

impl Parse for IfStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::If))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::If, "Expected 'if' at the beginning of if statement");

        expect_token!(tokens, index, TokenType::LeftParen, "Expected '(' after 'if'");

        let condition = expect_parse!(tokens, index, Expr, "Expected condition expression in 'if' statement");
        
        expect_token!(tokens, index, TokenType::RightParen, "Expected ')' after if condition");

        let then = expect_parse!(tokens, index, Block, "Expected block after 'if' condition");

        Ok((IfStatement { condition, then }, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

impl Parse for WhileStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::While))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::While, "Expected 'while' at the beginning of while statement");

        expect_token!(tokens, index, TokenType::LeftParen, "Expected '(' after 'while'");

        let condition = expect_parse!(tokens, index, Expr, "Expected condition expression in 'while' statement");

        expect_token!(tokens, index, TokenType::RightParen, "Expected ')' after while condition");

        let body = expect_parse!(tokens, index, Block, "Expected block after 'while' condition");

        Ok((WhileStatement { condition, body }, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnStatement(pub Option<Expr>);

impl Parse for ReturnStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::Return))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::Return, "Expected 'return' at the beginning of return statement");

        if Expr::is_next(tokens, index) {
            let expr = expect_parse!(tokens, index, Expr, "Expected expression after checking there is one");

            expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' at the end of 'return' statement");

            Ok((ReturnStatement(Some(expr)), index))
        } else {
            expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' after 'return' statement with no expression");

            Ok((ReturnStatement(None), index))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ThrowStatement(pub Option<Expr>);

impl Parse for ThrowStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::Throw))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::Throw, "Expected 'throw' at the beginning of throw statement");

        if Expr::is_next(tokens, index) {
            let expr = expect_parse!(tokens, index, Expr, "Expected expression after 'throw'");

            expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' at the end of 'throw' statement");

            Ok((ThrowStatement(Some(expr)), index))
        } else {
            expect_token!(tokens, index, TokenType::Semicolon, "Expected ';' after 'throw' statement with no expression");

            Ok((ThrowStatement(None), index))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ExitStatement;

impl Parse for ExitStatement {
    fn is_next(tokens: &[TokenType], index: usize) -> bool {
        matches!(tokens.get(index), Some(TokenType::Exit))
    }

    fn parse(tokens: &[TokenType], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::Exit, "Expected 'exit' at the beginning of exit statement");

        if let Some(TokenType::Semicolon) = tokens.get(index) {
            Ok((ExitStatement, index + 1))
        } else {
            Err("Expected ';' after 'exit' statement".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::parser::abstract_syntax_tree::expression::{ExprAtom, FunctionCall, Literal};

    use super::*;
    
    #[test]
    fn test_parse_let() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Identifier("x".to_string()),
            TokenType::Colon,
            TokenType::IntType,
            TokenType::Assign,
            TokenType::IntLiteral(42),
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'let' statement");
        assert_eq!(statement, Statement::VariableDeclaration(VariableDeclaration {
            name: Identifier("x".to_string()),
            datatype: Type::Int,
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(42))),
        }));
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = vec![
            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::IntLiteral(10),
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse assignment statement");
        assert_eq!(statement, Statement::Assignment(Assignment {
            target: Identifier("x".to_string()),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(10))),
        }));
    }

    #[test]
    fn test_parse_if() {
        let tokens = vec![
            TokenType::If,
            TokenType::LeftParen,
            TokenType::BoolLiteral(true),
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::IntLiteral(0),
            TokenType::Semicolon,
            TokenType::RightBrace,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'if' statement");
        assert_eq!(statement, Statement::If(IfStatement {
            condition: Expr::Atom(ExprAtom::Literal(Literal::Bool(true))),
            then: Block(vec![
                Statement::Assignment(Assignment {
                    target: Identifier("x".to_string()),
                    value: Expr::Atom(ExprAtom::Literal(Literal::Int(0))),
                }),
            ]),
        }));
    }

    #[test]
    fn test_parse_while() {
        let tokens = vec![
            TokenType::While,
            TokenType::LeftParen,
            TokenType::BoolLiteral(false),
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::IntLiteral(0),
            TokenType::Semicolon,
            TokenType::RightBrace,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'while' statement");
        assert_eq!(statement, Statement::While(WhileStatement {
            condition: Expr::Atom(ExprAtom::Literal(Literal::Bool(false))),
            body: Block(vec![
                Statement::Assignment(Assignment {
                    target: Identifier("x".to_string()),
                    value: Expr::Atom(ExprAtom::Literal(Literal::Int(0))),
                }),
            ]),
        }));
    }

    #[test]
    fn test_parse_return_with_expr() {
        let tokens = vec![
            TokenType::Return,
            TokenType::IntLiteral(5),
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'return' statement with expression");
        assert_eq!(statement, Statement::Return(ReturnStatement(Some(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))))));
    }

    #[test]
    fn test_parse_return_without_expr() {
        let tokens = vec![
            TokenType::Return,
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'return' statement without expression");
        assert_eq!(statement, Statement::Return(ReturnStatement(None)));
    }

    #[test]
    fn test_parse_throw() {
        let tokens = vec![
            TokenType::Throw,
            TokenType::StringLiteral("Error message".to_string()),
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'throw' statement");
        assert_eq!(statement, Statement::Throw(ThrowStatement(Some(Expr::Atom(ExprAtom::Literal(Literal::String("Error message".to_string())))))));
    }

    #[test]
    fn test_parse_throw_without_expr() {
        let tokens = vec![
            TokenType::Throw,
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'throw' statement without expression");
        assert_eq!(statement, Statement::Throw(ThrowStatement(None)));
    }

    #[test]
    fn test_parse_exit() {
        let tokens = vec![
            TokenType::Exit,
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse 'exit' statement");
        assert_eq!(statement, Statement::Exit(ExitStatement));
    }

    #[test]
    fn test_parse_function_call_statement() {
        let tokens = vec![
            TokenType::Identifier("doSomething".to_string()),
            TokenType::LeftParen,
            TokenType::IntLiteral(42),
            TokenType::Comma,
            TokenType::StringLiteral("hello".to_string()),
            TokenType::RightParen,
            TokenType::Semicolon,
        ];

        let (statement, _) = Statement::parse(&tokens, 0).expect("Failed to parse function call statement");
        assert_eq!(statement, Statement::Expression(ExprStatement(Expr::Atom(ExprAtom::FunctionCall(FunctionCall {
            identifier: Identifier("doSomething".to_string()),
            arguments: vec![
                Expr::Atom(ExprAtom::Literal(Literal::Int(42))),
                Expr::Atom(ExprAtom::Literal(Literal::String("hello".to_string()))),
            ],
            catch_block: None,
        })))));
    }
}