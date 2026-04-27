use crate::application::common::parser::lexer::token::Token;

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.get(self.index).map(|token| {
            self.index += 1;
            token
        })
    }
}
