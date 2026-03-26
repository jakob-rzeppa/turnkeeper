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

    Token {
        token,
        line: lexeme.line,
        first_char: lexeme.first_char,
    }
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
        let lexemes = vec![Lexeme { lexeme: LexemeType::Text("let".to_string()), line: 1, first_char: 1 }];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens, vec![Token { token: TokenType::Let, line: 1, first_char: 1 }]);
    }

    #[test]
    fn test_evaluate_lexemes_multiple() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Text("let".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Text("x".to_string()), line: 1, first_char: 5 },
            Lexeme { lexeme: LexemeType::Symbol("=".to_string()), line: 1, first_char: 7 },
            Lexeme { lexeme: LexemeType::Number("42".to_string()), line: 1, first_char: 9 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token { token: TokenType::Let, line: 1, first_char: 1 },
                Token { token: TokenType::Identifier("x".to_string()), line: 1, first_char: 5 },
                Token { token: TokenType::Assign, line: 1, first_char: 7 },
                Token { token: TokenType::IntLiteral(42), line: 1, first_char: 9 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_expression() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Text("x".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Symbol("+".to_string()), line: 1, first_char: 3 },
            Lexeme { lexeme: LexemeType::Number("5".to_string()), line: 1, first_char: 5 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token { token: TokenType::Identifier("x".to_string()), line: 1, first_char: 1 },
                Token { token: TokenType::Plus, line: 1, first_char: 3 },
                Token { token: TokenType::IntLiteral(5), line: 1, first_char: 5 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_with_string_and_types() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Text("int".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Quote("hello".to_string()), line: 1, first_char: 5 },
            Lexeme { lexeme: LexemeType::Text("true".to_string()), line: 1, first_char: 13 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token { token: TokenType::IntType, line: 1, first_char: 1 },
                Token { token: TokenType::StringLiteral("hello".to_string()), line: 1, first_char: 5 },
                Token { token: TokenType::BoolLiteral(true), line: 1, first_char: 13 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_function_definition() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Text("fn".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Text("add".to_string()), line: 1, first_char: 3 },
            Lexeme { lexeme: LexemeType::Symbol("(".to_string()), line: 1, first_char: 6 },
            Lexeme { lexeme: LexemeType::Text("a".to_string()), line: 1, first_char: 7 },
            Lexeme { lexeme: LexemeType::Symbol(":".to_string()), line: 1, first_char: 8 },
            Lexeme { lexeme: LexemeType::Text("int".to_string()), line: 1, first_char: 9 },
            Lexeme { lexeme: LexemeType::Symbol(")".to_string()), line: 1, first_char: 12 },
            Lexeme { lexeme: LexemeType::DoubleSymbol("->".to_string()), line: 1, first_char: 14 },
            Lexeme { lexeme: LexemeType::Text("int".to_string()), line: 1, first_char: 16 },
            Lexeme { lexeme: LexemeType::Symbol("{".to_string()), line: 1, first_char: 19 },
            Lexeme { lexeme: LexemeType::Text("return".to_string()), line: 1, first_char: 21 },
            Lexeme { lexeme: LexemeType::Text("a".to_string()), line: 1, first_char: 27 },
            Lexeme { lexeme: LexemeType::Symbol("}".to_string()), line: 1, first_char: 28 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], Token { token: TokenType::Fn, line: 1, first_char: 1 });
        assert_eq!(tokens[1], Token { token: TokenType::Identifier("add".to_string()), line: 1, first_char: 3 });
        assert_eq!(tokens[9], Token { token: TokenType::LeftBrace, line: 1, first_char: 19 });
        assert_eq!(tokens[10], Token { token: TokenType::Return, line: 1, first_char: 21 });
        assert_eq!(tokens[12], Token { token: TokenType::RightBrace, line: 1, first_char: 28 });
    }

    #[test]
    fn test_evaluate_lexemes_conditional() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Text("if".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Text("x".to_string()), line: 1, first_char: 3 },
            Lexeme { lexeme: LexemeType::DoubleSymbol(">=".to_string()), line: 1, first_char: 5 },
            Lexeme { lexeme: LexemeType::Number("10".to_string()), line: 1, first_char: 7 },
            Lexeme { lexeme: LexemeType::Symbol("{".to_string()), line: 1, first_char: 9 },
            Lexeme { lexeme: LexemeType::Text("break".to_string()), line: 1, first_char: 10 },
            Lexeme { lexeme: LexemeType::Symbol("}".to_string()), line: 1, first_char: 11 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token { token: TokenType::If, line: 1, first_char: 1 },
                Token { token: TokenType::Identifier("x".to_string()), line: 1, first_char: 3 },
                Token { token: TokenType::GreaterEqual, line: 1, first_char: 5 },
                Token { token: TokenType::IntLiteral(10), line: 1, first_char: 7 },
                Token { token: TokenType::LeftBrace, line: 1, first_char: 9 },
                Token { token: TokenType::Break, line: 1, first_char: 10 },
                Token { token: TokenType::RightBrace, line: 1, first_char: 11 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_mixed_operators() {
        let lexemes = vec![
            Lexeme { lexeme: LexemeType::Number("5".to_string()), line: 1, first_char: 1 },
            Lexeme { lexeme: LexemeType::Symbol("+".to_string()), line: 1, first_char: 2 },
            Lexeme { lexeme: LexemeType::Number("3".to_string()), line: 1, first_char: 3 },
            Lexeme { lexeme: LexemeType::Symbol("*".to_string()), line: 1, first_char: 4 },
            Lexeme { lexeme: LexemeType::Number("2".to_string()), line: 1, first_char: 5 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                Token { token: TokenType::IntLiteral(5), line: 1, first_char: 1 },
                Token { token: TokenType::Plus, line: 1, first_char: 2 },
                Token { token: TokenType::IntLiteral(3), line: 1, first_char: 3 },
                Token { token: TokenType::Star, line: 1, first_char: 4 },
                Token { token: TokenType::IntLiteral(2), line: 1, first_char: 5 },
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