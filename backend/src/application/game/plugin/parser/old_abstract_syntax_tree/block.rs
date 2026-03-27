use crate::application::game::plugin::{common::Position, lexer::token::{Token, TokenVariant}, parser::old_abstract_syntax_tree::{Parse, statement::Statement}};

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub pos: Position,
}

impl Parse for Block {
    fn is_next(tokens: &[Token], index: usize) -> bool {
        matches!(tokens.get(index), Some(t) if t.variant == TokenVariant::LeftBrace)
    }

    fn parse(tokens: &[Token], mut index: usize) -> Result<(Self, usize), String> {
        let pos = get_pos!(tokens, index);

        expect_token!(tokens, index, TokenVariant::LeftBrace, "Expected '{' to start a block");

        let mut statements = Vec::new();
        while tokens.get(index).map(|t| &t.variant) != Some(&TokenVariant::RightBrace) {
            let statement = expect_parse!(tokens, index, Statement, "Expected a statement inside a block");
            statements.push(statement);
        }

        expect_token!(tokens, index, TokenVariant::RightBrace, "Expected '}' to end a block");

        Ok((Block { statements, pos }, index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_block() {
        let tokens = vec![
            Token::new(TokenVariant::LeftBrace, Position::new(0, 0)),
            Token::new(TokenVariant::RightBrace, Position::new(0, 1)),
        ];
        let (block, _) = Block::parse(&tokens, 0).unwrap();
        assert_eq!(block, Block { statements: Vec::new(), pos: Position::new(0, 0) });
    }
}