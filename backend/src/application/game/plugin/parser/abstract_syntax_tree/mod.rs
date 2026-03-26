use crate::application::game::plugin::{lexer::token::TokenType, parser::abstract_syntax_tree::statement::Statement};

#[macro_use]
mod macros;

pub mod statement;
pub mod expression;
pub mod common;

pub trait Parse {
    /// Checks if the next tokens match the expected pattern for this type.
    /// 
    /// This is used to determine which parsing function to call when parsing a sequence of tokens.
    fn is_next(tokens: &[TokenType], index: usize) -> bool
    where
        Self: Sized;
    
    /// Parses the tokens starting from the given index and returns the parsed object along with the new index after parsing.
    /// 
    /// The new index should point to the next token after the parsed object, allowing for sequential parsing of multiple objects.
    fn parse(tokens: &[TokenType], index: usize) -> Result<(Self, usize), String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Root(pub Vec<Statement>);

impl Root {
    pub fn parse(mut tokens: Vec<TokenType>) -> Result<Self, String> {
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
    use super::*;

    #[test]
    fn test_parse_full_program() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Identifier("x".to_string()),
            TokenType::Colon,
            TokenType::IntType,
            TokenType::Assign,
            TokenType::IntLiteral(42),
            TokenType::Semicolon,

            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::IntLiteral(10),
            TokenType::Semicolon,

            TokenType::If,
            TokenType::LeftParen,
            TokenType::Identifier("x".to_string()),
            TokenType::Greater,
            TokenType::IntLiteral(20),
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Identifier("x".to_string()),
            TokenType::Assign,
            TokenType::IntLiteral(0),
            TokenType::Semicolon,
            TokenType::RightBrace,
        ];

        let root = Root::parse(tokens).expect("Failed to parse program");
        println!("{:#?}", root);
    }
}