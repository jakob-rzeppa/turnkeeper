use crate::application::game::plugin::lexer::token::Token;

pub mod token;
mod scanner;
mod evaluator;

pub fn tokenize(code: &str) -> Vec<Token> {
    let lexemes = scanner::scan_source_code(code);

    evaluator::evaluate_lexemes(lexemes.into_iter().map(|lexeme| lexeme.lexeme).collect())
}