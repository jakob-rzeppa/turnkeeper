use crate::application::game::plugin::{lexer::token::{Token}, parser::abstract_syntax_tree::statement::Statement};

#[macro_use]
mod macros;

pub mod statement;
pub mod expression;
pub mod common;

pub trait Parse {
    /// Checks if the next tokens match the expected pattern for this type.
    /// 
    /// This is used to determine which parsing function to call when parsing a sequence of tokens.
    fn is_next(tokens: &[Token], index: usize) -> bool
    where
        Self: Sized;
    
    /// Parses the tokens starting from the given index and returns the parsed object along with the new index after parsing.
    /// 
    /// The new index should point to the next token after the parsed object, allowing for sequential parsing of multiple objects.
    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Root(pub Vec<Statement>);

impl Root {
    pub fn parse(mut tokens: Vec<Token>) -> Result<Self, String> {
        let mut elements = Vec::new();
        
        let mut index = 0;
        while index < tokens.len() {
            let (element, new_index) = Statement::parse(&mut tokens, index)?;
            elements.push(element);
            index = new_index; // Consume the parsed tokens and move to the next token
        }

        Ok(Root(elements))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::lexer::token::TokenType;

    use super::*;

    #[test]
    fn test_parse_full_program() {
        let tokens = vec![
            Token::new(TokenType::Let, 0, 0),
            Token::new(TokenType::Identifier("x".to_string()), 0, 0),
            Token::new(TokenType::Colon, 0, 0),
            Token::new(TokenType::IntType, 0, 0),
            Token::new(TokenType::Assign, 0, 0),
            Token::new(TokenType::IntLiteral(42), 0, 0),
            Token::new(TokenType::Semicolon, 0, 0),

            Token::new(TokenType::Identifier("x".to_string()), 0, 0),
            Token::new(TokenType::Assign, 0, 0),
            Token::new(TokenType::IntLiteral(10), 0, 0),
            Token::new(TokenType::Semicolon, 0, 0),

            Token::new(TokenType::If, 0, 0),
            Token::new(TokenType::LeftParen, 0, 0),
            Token::new(TokenType::Identifier("x".to_string()), 0, 0),
            Token::new(TokenType::Greater, 0, 0),
            Token::new(TokenType::IntLiteral(20), 0, 0),
            Token::new(TokenType::RightParen, 0, 0),
            Token::new(TokenType::LeftBrace, 0, 0),
            Token::new(TokenType::Identifier("x".to_string()), 0, 0),
            Token::new(TokenType::Assign, 0, 0),
            Token::new(TokenType::IntLiteral(0), 0, 0),
            Token::new(TokenType::Semicolon, 0, 0),
            Token::new(TokenType::RightBrace, 0, 0),
        ];

        let root = Root::parse(tokens).expect("Failed to parse program");
        println!("{:#?}", root);
    }
}