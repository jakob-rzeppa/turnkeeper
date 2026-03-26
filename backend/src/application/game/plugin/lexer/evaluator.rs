use crate::application::game::plugin::lexer::{scanner::{LexemeType, Lexeme}, token::{TokenType, Token}};

fn evaluate_text_lexeme(text: String) -> TokenType {
    match text.as_str() {
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "match" => TokenType::Match,
        "while" => TokenType::While,
        "do" => TokenType::Do,
        "for" => TokenType::For,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        "reject" => TokenType::Reject,
        "throw" => TokenType::Throw,
        "exit" => TokenType::Exit,
        "exec" => TokenType::Exec,
        "catch" => TokenType::Catch,
        "fn" => TokenType::Fn,

        "int" => TokenType::IntType,
        "float" => TokenType::FloatType,
        "string" => TokenType::StringType,
        "bool" => TokenType::BoolType,
        
        _ if text == "true" => {
            TokenType::BoolLiteral(true)
        }
        _ if text == "false" => {
            TokenType::BoolLiteral(false)
        }

        _ => TokenType::Identifier(text),
        
    }
}

fn evaluate_number_lexeme(num: String) -> TokenType {
    let val = num.parse::<i64>().expect("valid number lexeme");
    TokenType::IntLiteral(val)
}

fn evaluate_number_with_dot_lexeme(num: String) -> TokenType {
    let val = num.parse::<f64>().expect("valid number with dot lexeme");
    TokenType::FloatLiteral(val)
}

fn evaluate_quote_lexeme(text: String) -> TokenType {
    TokenType::StringLiteral(text)
}

fn evaluate_symbol_lexeme(symbol: String) -> TokenType {
    match symbol.as_str() {
        "=" => TokenType::Assign,
        "+=" => TokenType::AddAssign,
        "-=" => TokenType::SubAssign,
        "*=" => TokenType::MulAssign,
        "/=" => TokenType::DivAssign,
        "%=" => TokenType::ModAssign,
        "^=" => TokenType::PowAssign,

        "+" => TokenType::Plus,
        "-" => TokenType::Minus,
        "*" => TokenType::Star,
        "/" => TokenType::Slash,
        "%" => TokenType::Percent,
        "^" => TokenType::Caret,
        "==" => TokenType::EqualEqual,
        "!=" => TokenType::NotEqual,
        "<=" => TokenType::LessEqual,
        ">=" => TokenType::GreaterEqual,
        "&&" => TokenType::And,
        "||" => TokenType::Or,
        "!" => TokenType::Not,

        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,
        ";" => TokenType::Semicolon,
        ":" => TokenType::Colon,
        "," => TokenType::Comma,
        "|" => TokenType::Pipe,
        "_" => TokenType::Underscore,

        "?" => TokenType::Question,

        "=>" => TokenType::ThickArrow,
        "->" => TokenType::ThinArrow,

        _ => panic!("Unknown symbol lexeme: {}", symbol),
    }
}

fn evaluate_lexeme(lexeme: Lexeme) -> Token {
    let token = match lexeme.lexeme {
        LexemeType::Text(text) => evaluate_text_lexeme(text),
        LexemeType::Number(num) => evaluate_number_lexeme(num),
        LexemeType::NumberWithDot(num) => evaluate_number_with_dot_lexeme(num),
        LexemeType::Quote(text) => evaluate_quote_lexeme(text),
        LexemeType::Symbol(symbol) => evaluate_symbol_lexeme(symbol),
        LexemeType::DoubleSymbol(symbol) => evaluate_symbol_lexeme(symbol),
    };

    Token::new(token, lexeme.pos.line, lexeme.pos.first_char)
}

pub fn evaluate_lexemes(lexemes: Vec<Lexeme>) -> Vec<Token> {
    lexemes.into_iter().map(evaluate_lexeme).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_lexemes_empty() {
        assert_eq!(evaluate_lexemes(vec![]), vec![]);
    }

    #[test]
    fn test_evaluate_lexemes_single() {
        let lexemes = vec![Lexeme::new(LexemeType::Text("let".to_string()), 1, 1)];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens, vec![Token::new(TokenType::Let, 1, 1)]);
    }

    #[test]
    fn test_evaluate_lexemes_multiple() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("let".to_string()), 1, 1),
            Lexeme::new(LexemeType::Text("x".to_string()), 1, 5),
            Lexeme::new(LexemeType::Symbol("=".to_string()), 1, 7),
            Lexeme::new(LexemeType::Number("42".to_string()), 1, 9),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Let, 1, 1),
                Token::new(TokenType::Identifier("x".to_string()), 1, 5),
                Token::new(TokenType::Assign, 1, 7),
                Token::new(TokenType::IntLiteral(42), 1, 9),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_expression() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("x".to_string()), 1, 1),
            Lexeme::new(LexemeType::Symbol("+".to_string()), 1, 3),
            Lexeme::new(LexemeType::Number("5".to_string()), 1, 5),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Identifier("x".to_string()), 1, 1),
                Token::new(TokenType::Plus, 1, 3),
                Token::new(TokenType::IntLiteral(5), 1, 5),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_with_string_and_types() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("int".to_string()), 1, 1),
            Lexeme::new(LexemeType::Quote("hello".to_string()), 1, 5),
            Lexeme::new(LexemeType::Text("true".to_string()), 1, 13),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::IntType, 1, 1),
                Token::new(TokenType::StringLiteral("hello".to_string()), 1, 5),
                Token::new(TokenType::BoolLiteral(true), 1, 13),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_function_definition() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("fn".to_string()), 1, 1),
            Lexeme::new(LexemeType::Text("add".to_string()), 1, 3),
            Lexeme::new(LexemeType::Symbol("(".to_string()), 1, 6),
            Lexeme::new(LexemeType::Text("a".to_string()), 1, 7),
            Lexeme::new(LexemeType::Symbol(":".to_string()), 1, 8),
            Lexeme::new(LexemeType::Text("int".to_string()), 1, 9),
            Lexeme::new(LexemeType::Symbol(")".to_string()), 1, 12),
            Lexeme::new(LexemeType::DoubleSymbol("->".to_string()), 1, 14),
            Lexeme::new(LexemeType::Text("int".to_string()), 1, 16),
            Lexeme::new(LexemeType::Symbol("{".to_string()), 1, 19),
            Lexeme::new(LexemeType::Text("return".to_string()), 1, 21),
            Lexeme::new(LexemeType::Text("a".to_string()), 1, 27),
            Lexeme::new(LexemeType::Symbol("}".to_string()), 1, 28),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], Token::new(TokenType::Fn, 1, 1));
        assert_eq!(tokens[1], Token::new(TokenType::Identifier("add".to_string()), 1, 3));
        assert_eq!(tokens[9], Token::new(TokenType::LeftBrace, 1, 19));
        assert_eq!(tokens[10], Token::new(TokenType::Return, 1, 21));
        assert_eq!(tokens[12], Token::new(TokenType::RightBrace, 1, 28));
    }

    #[test]
    fn test_evaluate_lexemes_conditional() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Text("if".to_string()), 1, 1),
            Lexeme::new(LexemeType::Text("x".to_string()), 1, 3),
            Lexeme::new(LexemeType::DoubleSymbol(">=".to_string()), 1, 5),
            Lexeme::new(LexemeType::Number("10".to_string()), 1, 7),
            Lexeme::new(LexemeType::Symbol("{".to_string()), 1, 9),
            Lexeme::new(LexemeType::Text("break".to_string()), 1, 10),
            Lexeme::new(LexemeType::Symbol("}".to_string()), 1, 11),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::If, 1, 1),
                Token::new(TokenType::Identifier("x".to_string()), 1, 3),
                Token::new(TokenType::GreaterEqual, 1, 5),
                Token::new(TokenType::IntLiteral(10), 1, 7),
                Token::new(TokenType::LeftBrace, 1, 9),
                Token::new(TokenType::Break, 1, 10),
                Token::new(TokenType::RightBrace, 1, 11),
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_mixed_operators() {
        let lexemes = vec![
            Lexeme::new(LexemeType::Number("5".to_string()), 1, 1),
            Lexeme::new(LexemeType::Symbol("+".to_string()), 1, 2),
            Lexeme::new(LexemeType::Number("3".to_string()), 1, 3),
            Lexeme::new(LexemeType::Symbol("*".to_string()), 1, 4),
            Lexeme::new(LexemeType::Number("2".to_string()), 1, 5),
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::IntLiteral(5), 1, 1),
                Token::new(TokenType::Plus, 1, 2),
                Token::new(TokenType::IntLiteral(3), 1, 3),
                Token::new(TokenType::Star, 1, 4),
                Token::new(TokenType::IntLiteral(2), 1, 5),
            ]
        );
    }

    // Edge case tests
    #[test]
    fn test_evaluate_keyword_case_sensitivity() {
        let upper = evaluate_text_lexeme("Let".to_string());
        let lower = evaluate_text_lexeme("let".to_string());
        assert_ne!(upper, lower);
        assert_eq!(upper, TokenType::Identifier("Let".to_string()));
        assert_eq!(lower, TokenType::Let);
    }

    #[test]
    fn test_evaluate_bool_case_sensitivity() {
        let result_true = evaluate_text_lexeme("True".to_string());
        let result_false = evaluate_text_lexeme("False".to_string());
        assert_eq!(result_true, TokenType::Identifier("True".to_string()));
        assert_eq!(result_false, TokenType::Identifier("False".to_string()));
    }

    #[test]
    fn test_evaluate_large_number() {
        assert_eq!(
            evaluate_number_lexeme("9223372036854775807".to_string()),
            TokenType::IntLiteral(9223372036854775807)
        );
    }

    #[test]
    fn test_evaluate_large_float() {
        assert_eq!(
            evaluate_number_with_dot_lexeme("999999.999999".to_string()),
            TokenType::FloatLiteral(999999.999999)
        );
    }

    #[test]
    fn test_evaluate_identifier_all_underscores() {
        assert_eq!(evaluate_text_lexeme("___".to_string()), TokenType::Identifier("___".to_string()));
    }
}