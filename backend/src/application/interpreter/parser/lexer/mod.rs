// mod evaluator;
// pub mod token;

// pub struct Lexer {
//     scanner: Scanner,
//     evaluator: Evaluator,
// }

// impl Lexer {
//     pub fn new() -> Self {
//         Lexer {
//             scanner: Scanner::new(),
//             evaluator: Evaluator::new(),
//         }
//     }

//     #[allow(dead_code)]
//     pub fn tokenize(&self, source_code: &str) -> Vec<Token> {
//         let lexemes = self.scanner.scan_source_code(source_code);

//         self.evaluator.evaluate_lexemes(lexemes)
//     }
// }
