use crate::application::interpreter::parser::lexer::token::{Token, TokenVariant};

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn evaluate_lexemes(&self, lexemes: Vec<Lexeme>) -> Vec<Token> {
        lexemes
            .into_iter()
            .map(|lexeme| self.evaluate_lexeme(lexeme))
            .collect()
    }

    fn evaluate_lexeme(&self, lexeme: Lexeme) -> Token {
        let token = match lexeme.lexeme {
            LexemeType::Text(text) => self.evaluate_text_lexeme(text),
            LexemeType::Number(num) => self.evaluate_number_lexeme(num),
            LexemeType::NumberWithDot(num) => self.evaluate_number_with_dot_lexeme(num),
            LexemeType::Quote(text) => self.evaluate_quote_lexeme(text),
            LexemeType::Symbol(symbol) => self.evaluate_symbol_lexeme(symbol),
            LexemeType::DoubleSymbol(symbol) => self.evaluate_symbol_lexeme(symbol),
        };

        Token::new(token, lexeme.pos)
    }

    fn evaluate_text_lexeme(&self, text: String) -> TokenVariant {
        match text.as_str() {
            "let" => TokenVariant::Let,
            "if" => TokenVariant::If,
            "else" => TokenVariant::Else,
            "match" => TokenVariant::Match,
            "while" => TokenVariant::While,
            "do" => TokenVariant::Do,
            "for" => TokenVariant::For,
            "break" => TokenVariant::Break,
            "continue" => TokenVariant::Continue,
            "return" => TokenVariant::Return,
            "reject" => TokenVariant::Reject,
            "throw" => TokenVariant::Throw,
            "exit" => TokenVariant::Exit,
            "exec" => TokenVariant::Exec,
            "catch" => TokenVariant::Catch,
            "fn" => TokenVariant::Fn,

            "int" => TokenVariant::IntType,
            "float" => TokenVariant::FloatType,
            "string" => TokenVariant::StringType,
            "bool" => TokenVariant::BoolType,

            _ if text == "true" => TokenVariant::BoolLiteral(true),
            _ if text == "false" => TokenVariant::BoolLiteral(false),

            _ => TokenVariant::Identifier(text),
        }
    }

    fn evaluate_number_lexeme(&self, num: String) -> TokenVariant {
        let val = num.parse::<i64>().expect("valid number lexeme");
        TokenVariant::IntLiteral(val)
    }

    fn evaluate_number_with_dot_lexeme(&self, num: String) -> TokenVariant {
        let val = num.parse::<f64>().expect("valid number with dot lexeme");
        TokenVariant::FloatLiteral(val)
    }

    fn evaluate_quote_lexeme(&self, text: String) -> TokenVariant {
        TokenVariant::StringLiteral(text)
    }

    fn evaluate_symbol_lexeme(&self, symbol: String) -> TokenVariant {
        match symbol.as_str() {
            "=" => TokenVariant::Assign,
            "+=" => TokenVariant::AddAssign,
            "-=" => TokenVariant::SubAssign,
            "*=" => TokenVariant::MulAssign,
            "/=" => TokenVariant::DivAssign,
            "%=" => TokenVariant::ModAssign,
            "^=" => TokenVariant::PowAssign,

            "+" => TokenVariant::Plus,
            "-" => TokenVariant::Minus,
            "*" => TokenVariant::Star,
            "/" => TokenVariant::Slash,
            "%" => TokenVariant::Percent,
            "^" => TokenVariant::Caret,
            "==" => TokenVariant::EqualEqual,
            "!=" => TokenVariant::NotEqual,
            "<=" => TokenVariant::LessEqual,
            ">=" => TokenVariant::GreaterEqual,
            "&&" => TokenVariant::And,
            "||" => TokenVariant::Or,
            "!" => TokenVariant::Not,

            "(" => TokenVariant::LeftParen,
            ")" => TokenVariant::RightParen,
            "{" => TokenVariant::LeftBrace,
            "}" => TokenVariant::RightBrace,
            "[" => TokenVariant::LeftBracket,
            "]" => TokenVariant::RightBracket,
            ";" => TokenVariant::Semicolon,
            ":" => TokenVariant::Colon,
            "," => TokenVariant::Comma,
            "|" => TokenVariant::Pipe,
            "_" => TokenVariant::Underscore,

            "?" => TokenVariant::Question,

            "=>" => TokenVariant::ThickArrow,
            "->" => TokenVariant::ThinArrow,

            _ => panic!("Unknown symbol lexeme: {}", symbol),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::application::interpreter::position::Position;

    use super::*;

    #[test]
    fn test_evaluate_lexemes_empty() {
        let evaluator = Evaluator::new();
        assert_eq!(evaluator.evaluate_lexemes(vec![]), vec![]);
    }

    #[test]
    fn test_evaluate_lexemes_single() {
        let evaluator = Evaluator::new();
        let lexemes = vec![Lexeme::new(
            LexemeType::Text("let".to_string()),
            Position::new(1, 1),
        )];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![Token::new(TokenVariant::Let, Position::new(1, 1))]
        );
    }

    #[test]
    fn test_evaluate_lexemes_multiple() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("let".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Text("x".to_string()), Position::new(1, 5)),
            Lexeme::new(LexemeType::Symbol("=".to_string()), Position::new(1, 7)),
            Lexeme::new(LexemeType::Number("42".to_string()), Position::new(1, 9)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenVariant::Let, Position::new(1, 1)),
                Token::new(
                    TokenVariant::Identifier("x".to_string()),
                    Position::new(1, 5)
                ),
                Token::new(TokenVariant::Assign, Position::new(1, 7)),
                Token::new(TokenVariant::IntLiteral(42), Position::new(1, 9)),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_expression() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("x".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(1, 3)),
            Lexeme::new(LexemeType::Number("5".to_string()), Position::new(1, 5)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenVariant::Identifier("x".to_string()),
                    Position::new(1, 1)
                ),
                Token::new(TokenVariant::Plus, Position::new(1, 3)),
                Token::new(TokenVariant::IntLiteral(5), Position::new(1, 5)),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_with_string_and_types() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("int".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Quote("hello".to_string()), Position::new(1, 5)),
            Lexeme::new(LexemeType::Text("true".to_string()), Position::new(1, 13)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenVariant::IntType, Position::new(1, 1)),
                Token::new(
                    TokenVariant::StringLiteral("hello".to_string()),
                    Position::new(1, 5)
                ),
                Token::new(TokenVariant::BoolLiteral(true), Position::new(1, 13)),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_function_definition() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("fn".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Text("add".to_string()), Position::new(1, 3)),
            Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(1, 6)),
            Lexeme::new(LexemeType::Text("a".to_string()), Position::new(1, 7)),
            Lexeme::new(LexemeType::Symbol(":".to_string()), Position::new(1, 8)),
            Lexeme::new(LexemeType::Text("int".to_string()), Position::new(1, 9)),
            Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(1, 12)),
            Lexeme::new(
                LexemeType::DoubleSymbol("->".to_string()),
                Position::new(1, 14),
            ),
            Lexeme::new(LexemeType::Text("int".to_string()), Position::new(1, 16)),
            Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(1, 19)),
            Lexeme::new(LexemeType::Text("return".to_string()), Position::new(1, 21)),
            Lexeme::new(LexemeType::Text("a".to_string()), Position::new(1, 27)),
            Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(1, 28)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], Token::new(TokenVariant::Fn, Position::new(1, 1)));
        assert_eq!(
            tokens[1],
            Token::new(
                TokenVariant::Identifier("add".to_string()),
                Position::new(1, 3)
            )
        );
        assert_eq!(
            tokens[9],
            Token::new(TokenVariant::LeftBrace, Position::new(1, 19))
        );
        assert_eq!(
            tokens[10],
            Token::new(TokenVariant::Return, Position::new(1, 21))
        );
        assert_eq!(
            tokens[12],
            Token::new(TokenVariant::RightBrace, Position::new(1, 28))
        );
    }

    #[test]
    fn test_evaluate_lexemes_conditional() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("if".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Text("x".to_string()), Position::new(1, 3)),
            Lexeme::new(
                LexemeType::DoubleSymbol(">=".to_string()),
                Position::new(1, 5),
            ),
            Lexeme::new(LexemeType::Number("10".to_string()), Position::new(1, 7)),
            Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(1, 9)),
            Lexeme::new(LexemeType::Text("break".to_string()), Position::new(1, 10)),
            Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(1, 11)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenVariant::If, Position::new(1, 1)),
                Token::new(
                    TokenVariant::Identifier("x".to_string()),
                    Position::new(1, 3)
                ),
                Token::new(TokenVariant::GreaterEqual, Position::new(1, 5)),
                Token::new(TokenVariant::IntLiteral(10), Position::new(1, 7)),
                Token::new(TokenVariant::LeftBrace, Position::new(1, 9)),
                Token::new(TokenVariant::Break, Position::new(1, 10)),
                Token::new(TokenVariant::RightBrace, Position::new(1, 11)),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_mixed_operators() {
        let evaluator = Evaluator::new();
        let lexemes = vec![
            Lexeme::new(LexemeType::Number("5".to_string()), Position::new(1, 1)),
            Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(1, 2)),
            Lexeme::new(LexemeType::Number("3".to_string()), Position::new(1, 3)),
            Lexeme::new(LexemeType::Symbol("*".to_string()), Position::new(1, 4)),
            Lexeme::new(LexemeType::Number("2".to_string()), Position::new(1, 5)),
        ];
        let tokens = evaluator.evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenVariant::IntLiteral(5), Position::new(1, 1)),
                Token::new(TokenVariant::Plus, Position::new(1, 2)),
                Token::new(TokenVariant::IntLiteral(3), Position::new(1, 3)),
                Token::new(TokenVariant::Star, Position::new(1, 4)),
                Token::new(TokenVariant::IntLiteral(2), Position::new(1, 5)),
            ]
        );
    }

    // Edge case tests
    #[test]
    fn test_evaluate_keyword_case_sensitivity() {
        let evaluator = Evaluator::new();
        let upper = evaluator.evaluate_text_lexeme("Let".to_string());
        let lower = evaluator.evaluate_text_lexeme("let".to_string());
        assert_ne!(upper, lower);
        assert_eq!(upper, TokenVariant::Identifier("Let".to_string()));
        assert_eq!(lower, TokenVariant::Let);
    }

    #[test]
    fn test_evaluate_bool_case_sensitivity() {
        let evaluator = Evaluator::new();
        let result_true = evaluator.evaluate_text_lexeme("True".to_string());
        let result_false = evaluator.evaluate_text_lexeme("False".to_string());
        assert_eq!(result_true, TokenVariant::Identifier("True".to_string()));
        assert_eq!(result_false, TokenVariant::Identifier("False".to_string()));
    }

    #[test]
    fn test_evaluate_large_number() {
        let evaluator = Evaluator::new();
        assert_eq!(
            evaluator.evaluate_number_lexeme("9223372036854775807".to_string()),
            TokenVariant::IntLiteral(9223372036854775807)
        );
    }

    #[test]
    fn test_evaluate_large_float() {
        let evaluator = Evaluator::new();
        assert_eq!(
            evaluator.evaluate_number_with_dot_lexeme("999999.999999".to_string()),
            TokenVariant::FloatLiteral(999999.999999)
        );
    }

    #[test]
    fn test_evaluate_identifier_all_underscores() {
        let evaluator = Evaluator::new();
        assert_eq!(
            evaluator.evaluate_text_lexeme("___".to_string()),
            TokenVariant::Identifier("___".to_string())
        );
    }
}
