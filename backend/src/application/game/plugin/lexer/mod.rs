use crate::application::game::plugin::lexer::token::{TokenWithPosition};

pub mod token;
mod scanner;
mod evaluator;

pub fn tokenize(code: &str) -> Vec<TokenWithPosition> {
    let lexemes = scanner::scan_source_code(code);

    evaluator::evaluate_lexemes(lexemes)
}