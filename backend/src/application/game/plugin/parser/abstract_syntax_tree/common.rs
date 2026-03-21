use crate::application::game::plugin::{lexer::token::Token, parser::abstract_syntax_tree::{Parse, statement::Statement}};

#[derive(Clone, PartialEq, Debug)]
pub struct Block(pub Vec<Statement>);

impl Parse for Block {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(&Token::LeftBrace))
    }

    fn parse(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, Token::LeftBrace, "Expected '{' to start a block");

        let mut statements = Vec::new();
        while tokens.get(index) != Some(&Token::RightBrace) {
            let statement = expect_parse!(tokens, index, Statement, "Expected a statement inside a block");
            statements.push(statement);
        }

        expect_token!(tokens, index, Token::RightBrace, "Expected '}' to end a block");

        Ok((Block(statements), index))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

impl Parse for Type {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(Token::IntType | Token::FloatType | Token::StringType | Token::BoolType))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index) {
            Some(Token::IntType) => Ok((Type::Int, index + 1)),
            Some(Token::FloatType) => Ok((Type::Float, index + 1)),
            Some(Token::StringType) => Ok((Type::String, index + 1)),
            Some(Token::BoolType) => Ok((Type::Bool, index + 1)),
            _ => Err("Expected a type (int, float, string, bool)".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(Token::Identifier(_)))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index) {
            Some(Token::Identifier(name)) => Ok((Identifier(name.clone()), index + 1)),
            _ => Err("Expected an identifier".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block() {
        let tokens = vec![
            Token::LeftBrace,
            Token::RightBrace,
        ];
        let (block, _) = Block::parse(&tokens, 0).unwrap();
        assert_eq!(block, Block(Vec::new()));
    }

    #[test]
    fn test_parse_type() {
        let tokens = vec![
            Token::IntType,
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Int);

        let tokens = vec![
            Token::StringType,
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::String);

        let tokens = vec![
            Token::BoolType,
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Bool);

        let tokens = vec![
            Token::FloatType,
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Float);
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
        ];
        let (identifier, _) = Identifier::parse(&tokens, 0).unwrap();
        assert_eq!(identifier, Identifier("x".to_string()));
    }
}