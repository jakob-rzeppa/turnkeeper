use crate::application::game::plugin::{common::Position, lexer::token::{Token, TokenType}, parser::abstract_syntax_tree::Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub pos: Position,
}

impl Parse for Identifier {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(t) if matches!(t.token, TokenType::Identifier(_)))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        let pos = get_pos!(tokens, index);

        match tokens.get(index).map(|t| &t.token) {
            Some(TokenType::Identifier(name)) => Ok((Identifier { name: name.clone(), pos }, index + 1)),
            _ => Err("Expected an identifier".to_string()),
        }
    }
}

impl Identifier {
    pub fn new(name: String, pos: Position) -> Self {
        Identifier { name, pos }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::common::Position;

    use super::*;

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![
            Token::new(TokenType::Identifier("x".to_string()), Position::new(0, 0)),
        ];
        let (identifier, _) = Identifier::parse(&tokens, 0).unwrap();
        assert_eq!(identifier, Identifier { name: "x".to_string(), pos: Position::new(0, 0) });
    }
}