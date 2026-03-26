use crate::application::game::plugin::{lexer::token::{Token, TokenType}, parser::abstract_syntax_tree::{Parse, statement::Statement}};

#[derive(Clone, PartialEq, Debug)]
pub struct Block(pub Vec<Statement>);

impl Parse for Block {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(t) if t.token == TokenType::LeftBrace)
    }

    fn parse(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        expect_token!(tokens, index, TokenType::LeftBrace, "Expected '{' to start a block");

        let mut statements = Vec::new();
        while tokens.get(index).map(|t| &t.token) != Some(&TokenType::RightBrace) {
            let statement = expect_parse!(tokens, index, Statement, "Expected a statement inside a block");
            statements.push(statement);
        }

        expect_token!(tokens, index, TokenType::RightBrace, "Expected '}' to end a block");

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
        matches!(tokens.get(index), Some(t) if matches!(t.token, TokenType::IntType | TokenType::FloatType | TokenType::StringType | TokenType::BoolType))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index).map(|t| &t.token) {
            Some(TokenType::IntType) => Ok((Type::Int, index + 1)),
            Some(TokenType::FloatType) => Ok((Type::Float, index + 1)),
            Some(TokenType::StringType) => Ok((Type::String, index + 1)),
            Some(TokenType::BoolType) => Ok((Type::Bool, index + 1)),
            _ => Err("Expected a type (int, float, string, bool)".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(t) if matches!(t.token, TokenType::Identifier(_)))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        match tokens.get(index).map(|t| &t.token) {
            Some(TokenType::Identifier(name)) => Ok((Identifier(name.clone()), index + 1)),
            _ => Err("Expected an identifier".to_string()),
        }
    }
}

impl Identifier {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block() {
        let tokens = vec![
            Token { token: TokenType::LeftBrace, line: 0, first_char: 0 },
            Token { token: TokenType::RightBrace, line: 0, first_char: 0 },
        ];
        let (block, _) = Block::parse(&tokens, 0).unwrap();
        assert_eq!(block, Block(Vec::new()));
    }

    #[test]
    fn test_parse_type() {
        let tokens = vec![
            Token { token: TokenType::IntType, line: 0, first_char: 0 },
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Int);

        let tokens = vec![
            Token { token: TokenType::StringType, line: 0, first_char: 0 },
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::String);

        let tokens = vec![
            Token { token: TokenType::BoolType, line: 0, first_char: 0 },
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Bool);

        let tokens = vec![
            Token { token: TokenType::FloatType, line: 0, first_char: 0 },
        ];
        let (datatype, _) = Type::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Type::Float);
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![
            Token { token: TokenType::Identifier("x".to_string()), line: 0, first_char: 0 },
        ];
        let (identifier, _) = Identifier::parse(&tokens, 0).unwrap();
        assert_eq!(identifier, Identifier("x".to_string()));
    }
}