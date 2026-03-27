use crate::application::game::plugin::{common::Position, lexer::token::{Token, TokenVariant}, parser::old_abstract_syntax_tree::Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Datatype {
    pub variant: DatatypeVariant,
    pub pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatatypeVariant {
    Int,
    Float,
    String,
    Bool,
}

impl Datatype {
    pub fn new(variant: DatatypeVariant, pos: Position) -> Self {
        Datatype { variant, pos }
    }
}

impl Parse for Datatype {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(t) if matches!(t.variant, TokenVariant::IntType | TokenVariant::FloatType | TokenVariant::StringType | TokenVariant::BoolType))
    }

    fn parse(tokens: &[Token], index: usize) -> Result<(Self, usize), String> {
        let pos = get_pos!(tokens, index);

        match tokens.get(index).map(|t| &t.variant) {
            Some(TokenVariant::IntType) => Ok((Datatype::new(DatatypeVariant::Int, pos), index + 1)),
            Some(TokenVariant::FloatType) => Ok((Datatype::new(DatatypeVariant::Float, pos), index + 1)),
            Some(TokenVariant::StringType) => Ok((Datatype::new(DatatypeVariant::String, pos), index + 1)),
            Some(TokenVariant::BoolType) => Ok((Datatype::new(DatatypeVariant::Bool, pos), index + 1)),
            _ => Err("Expected a type (int, float, string, bool)".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type() {
        let tokens = vec![
            Token::new(TokenVariant::IntType, Position::new(0, 0)),
        ];
        let (datatype, _) = Datatype::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Datatype::new(DatatypeVariant::Int, Position::new(0, 0)));

        let tokens = vec![
            Token::new(TokenVariant::StringType, Position::new(0, 0)),
        ];
        let (datatype, _) = Datatype::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Datatype::new(DatatypeVariant::String, Position::new(0, 0)));

        let tokens = vec![
            Token::new(TokenVariant::BoolType, Position::new(0, 0)),
        ];
        let (datatype, _) = Datatype::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Datatype::new(DatatypeVariant::Bool, Position::new(0, 0)));

        let tokens = vec![
            Token::new(TokenVariant::FloatType, Position::new(0, 0)),
        ];
        let (datatype, _) = Datatype::parse(&tokens, 0).unwrap();
        assert_eq!(datatype, Datatype::new(DatatypeVariant::Float, Position::new(0, 0)));
    }
}