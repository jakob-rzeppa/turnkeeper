use crate::application::game::plugin::lexer::{scanner::{Lexeme, LexemeWithPosition}, token::{Token, TokenWithPosition}};

fn evaluate_text_lexeme(text: String) -> Token {
    match text.as_str() {
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "match" => Token::Match,
        "while" => Token::While,
        "do" => Token::Do,
        "for" => Token::For,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "return" => Token::Return,
        "reject" => Token::Reject,
        "throw" => Token::Throw,
        "exit" => Token::Exit,
        "exec" => Token::Exec,
        "catch" => Token::Catch,
        "fn" => Token::Fn,

        "int" => Token::IntType,
        "float" => Token::FloatType,
        "string" => Token::StringType,
        "bool" => Token::BoolType,
        
        _ if text == "true" => {
            Token::BoolLiteral(true)
        }
        _ if text == "false" => {
            Token::BoolLiteral(false)
        }

        _ => Token::Identifier(text),
        
    }
}

fn evaluate_number_lexeme(num: String) -> Token {
    let val = num.parse::<i64>().expect("valid number lexeme");
    Token::IntLiteral(val)
}

fn evaluate_number_with_dot_lexeme(num: String) -> Token {
    let val = num.parse::<f64>().expect("valid number with dot lexeme");
    Token::FloatLiteral(val)
}

fn evaluate_quote_lexeme(text: String) -> Token {
    Token::StringLiteral(text)
}

fn evaluate_symbol_lexeme(symbol: String) -> Token {
    match symbol.as_str() {
        "=" => Token::Assign,
        "+=" => Token::AddAssign,
        "-=" => Token::SubAssign,
        "*=" => Token::MulAssign,
        "/=" => Token::DivAssign,
        "%=" => Token::ModAssign,
        "^=" => Token::PowAssign,

        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Star,
        "/" => Token::Slash,
        "%" => Token::Percent,
        "^" => Token::Caret,
        "==" => Token::EqualEqual,
        "!=" => Token::NotEqual,
        "<=" => Token::LessEqual,
        ">=" => Token::GreaterEqual,
        "&&" => Token::And,
        "||" => Token::Or,
        "!" => Token::Not,

        "(" => Token::LeftParen,
        ")" => Token::RightParen,
        "{" => Token::LeftBrace,
        "}" => Token::RightBrace,
        "[" => Token::LeftBracket,
        "]" => Token::RightBracket,
        ";" => Token::Semicolon,
        ":" => Token::Colon,
        "," => Token::Comma,
        "|" => Token::Pipe,
        "_" => Token::Underscore,

        "?" => Token::Question,

        "=>" => Token::ThickArrow,
        "->" => Token::ThinArrow,

        _ => panic!("Unknown symbol lexeme: {}", symbol),
    }
}

fn evaluate_lexeme(lexeme: LexemeWithPosition) -> TokenWithPosition {
    let token = match lexeme.lexeme {
        Lexeme::Text(text) => evaluate_text_lexeme(text),
        Lexeme::Number(num) => evaluate_number_lexeme(num),
        Lexeme::NumberWithDot(num) => evaluate_number_with_dot_lexeme(num),
        Lexeme::Quote(text) => evaluate_quote_lexeme(text),
        Lexeme::Symbol(symbol) => evaluate_symbol_lexeme(symbol),
        Lexeme::DoubleSymbol(symbol) => evaluate_symbol_lexeme(symbol),
    };

    TokenWithPosition {
        token,
        line: lexeme.line,
        first_char: lexeme.first_char,
    }
}

pub fn evaluate_lexemes(lexemes: Vec<LexemeWithPosition>) -> Vec<TokenWithPosition> {
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
        let lexemes = vec![LexemeWithPosition { lexeme: Lexeme::Text("let".to_string()), line: 1, first_char: 1 }];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens, vec![TokenWithPosition { token: Token::Let, line: 1, first_char: 1 }]);
    }

    #[test]
    fn test_evaluate_lexemes_multiple() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Text("let".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Text("x".to_string()), line: 1, first_char: 5 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("=".to_string()), line: 1, first_char: 7 },
            LexemeWithPosition { lexeme: Lexeme::Number("42".to_string()), line: 1, first_char: 9 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                TokenWithPosition { token: Token::Let, line: 1, first_char: 1 },
                TokenWithPosition { token: Token::Identifier("x".to_string()), line: 1, first_char: 5 },
                TokenWithPosition { token: Token::Assign, line: 1, first_char: 7 },
                TokenWithPosition { token: Token::IntLiteral(42), line: 1, first_char: 9 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_expression() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Text("x".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("+".to_string()), line: 1, first_char: 3 },
            LexemeWithPosition { lexeme: Lexeme::Number("5".to_string()), line: 1, first_char: 5 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                TokenWithPosition { token: Token::Identifier("x".to_string()), line: 1, first_char: 1 },
                TokenWithPosition { token: Token::Plus, line: 1, first_char: 3 },
                TokenWithPosition { token: Token::IntLiteral(5), line: 1, first_char: 5 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_with_string_and_types() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Text("int".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Quote("hello".to_string()), line: 1, first_char: 5 },
            LexemeWithPosition { lexeme: Lexeme::Text("true".to_string()), line: 1, first_char: 13 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                TokenWithPosition { token: Token::IntType, line: 1, first_char: 1 },
                TokenWithPosition { token: Token::StringLiteral("hello".to_string()), line: 1, first_char: 5 },
                TokenWithPosition { token: Token::BoolLiteral(true), line: 1, first_char: 13 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_function_definition() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Text("fn".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Text("add".to_string()), line: 1, first_char: 3 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("(".to_string()), line: 1, first_char: 6 },
            LexemeWithPosition { lexeme: Lexeme::Text("a".to_string()), line: 1, first_char: 7 },
            LexemeWithPosition { lexeme: Lexeme::Symbol(":".to_string()), line: 1, first_char: 8 },
            LexemeWithPosition { lexeme: Lexeme::Text("int".to_string()), line: 1, first_char: 9 },
            LexemeWithPosition { lexeme: Lexeme::Symbol(")".to_string()), line: 1, first_char: 12 },
            LexemeWithPosition { lexeme: Lexeme::DoubleSymbol("->".to_string()), line: 1, first_char: 14 },
            LexemeWithPosition { lexeme: Lexeme::Text("int".to_string()), line: 1, first_char: 16 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("{".to_string()), line: 1, first_char: 19 },
            LexemeWithPosition { lexeme: Lexeme::Text("return".to_string()), line: 1, first_char: 21 },
            LexemeWithPosition { lexeme: Lexeme::Text("a".to_string()), line: 1, first_char: 27 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("}".to_string()), line: 1, first_char: 28 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0], TokenWithPosition { token: Token::Fn, line: 1, first_char: 1 });
        assert_eq!(tokens[1], TokenWithPosition { token: Token::Identifier("add".to_string()), line: 1, first_char: 3 });
        assert_eq!(tokens[9], TokenWithPosition { token: Token::LeftBrace, line: 1, first_char: 19 });
        assert_eq!(tokens[10], TokenWithPosition { token: Token::Return, line: 1, first_char: 21 });
        assert_eq!(tokens[12], TokenWithPosition { token: Token::RightBrace, line: 1, first_char: 28 });
    }

    #[test]
    fn test_evaluate_lexemes_conditional() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Text("if".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Text("x".to_string()), line: 1, first_char: 3 },
            LexemeWithPosition { lexeme: Lexeme::DoubleSymbol(">=".to_string()), line: 1, first_char: 5 },
            LexemeWithPosition { lexeme: Lexeme::Number("10".to_string()), line: 1, first_char: 7 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("{".to_string()), line: 1, first_char: 9 },
            LexemeWithPosition { lexeme: Lexeme::Text("break".to_string()), line: 1, first_char: 10 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("}".to_string()), line: 1, first_char: 11 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                TokenWithPosition { token: Token::If, line: 1, first_char: 1 },
                TokenWithPosition { token: Token::Identifier("x".to_string()), line: 1, first_char: 3 },
                TokenWithPosition { token: Token::GreaterEqual, line: 1, first_char: 5 },
                TokenWithPosition { token: Token::IntLiteral(10), line: 1, first_char: 7 },
                TokenWithPosition { token: Token::LeftBrace, line: 1, first_char: 9 },
                TokenWithPosition { token: Token::Break, line: 1, first_char: 10 },
                TokenWithPosition { token: Token::RightBrace, line: 1, first_char: 11 },
            ]
        );
    }

    #[test]
    fn test_evaluate_lexemes_mixed_operators() {
        let lexemes = vec![
            LexemeWithPosition { lexeme: Lexeme::Number("5".to_string()), line: 1, first_char: 1 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("+".to_string()), line: 1, first_char: 2 },
            LexemeWithPosition { lexeme: Lexeme::Number("3".to_string()), line: 1, first_char: 3 },
            LexemeWithPosition { lexeme: Lexeme::Symbol("*".to_string()), line: 1, first_char: 4 },
            LexemeWithPosition { lexeme: Lexeme::Number("2".to_string()), line: 1, first_char: 5 },
        ];
        let tokens = evaluate_lexemes(lexemes);
        assert_eq!(
            tokens,
            vec![
                TokenWithPosition { token: Token::IntLiteral(5), line: 1, first_char: 1 },
                TokenWithPosition { token: Token::Plus, line: 1, first_char: 2 },
                TokenWithPosition { token: Token::IntLiteral(3), line: 1, first_char: 3 },
                TokenWithPosition { token: Token::Star, line: 1, first_char: 4 },
                TokenWithPosition { token: Token::IntLiteral(2), line: 1, first_char: 5 },
            ]
        );
    }

    // Edge case tests
    #[test]
    fn test_evaluate_keyword_case_sensitivity() {
        let upper = evaluate_text_lexeme("Let".to_string());
        let lower = evaluate_text_lexeme("let".to_string());
        assert_ne!(upper, lower);
        assert_eq!(upper, Token::Identifier("Let".to_string()));
        assert_eq!(lower, Token::Let);
    }

    #[test]
    fn test_evaluate_bool_case_sensitivity() {
        let result_true = evaluate_text_lexeme("True".to_string());
        let result_false = evaluate_text_lexeme("False".to_string());
        assert_eq!(result_true, Token::Identifier("True".to_string()));
        assert_eq!(result_false, Token::Identifier("False".to_string()));
    }

    #[test]
    fn test_evaluate_large_number() {
        assert_eq!(
            evaluate_number_lexeme("9223372036854775807".to_string()),
            Token::IntLiteral(9223372036854775807)
        );
    }

    #[test]
    fn test_evaluate_large_float() {
        assert_eq!(
            evaluate_number_with_dot_lexeme("999999.999999".to_string()),
            Token::FloatLiteral(999999.999999)
        );
    }

    #[test]
    fn test_evaluate_identifier_all_underscores() {
        assert_eq!(evaluate_text_lexeme("___".to_string()), Token::Identifier("___".to_string()));
    }
}