use crate::application::plugin::lexer::token::Token;

mod evaluator;
mod scanner;
pub mod token;

pub fn tokenize(code: &str) -> Vec<Token> {
    let lexemes = scanner::scan_source_code(code);

    evaluator::evaluate_lexemes(lexemes)
}
