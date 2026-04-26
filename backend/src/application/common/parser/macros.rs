/// The macro is used to check if the next token matches the expected pattern.
/// It simplifies the code by reducing the boilerplate of checking for the presence of a token and comparing it to an expected value.
///
/// Example usage: `is_token!(tokens, TokenType::Let)` checks if the next token in `tokens` is of type `TokenType::Let`.
///
/// It return true if the token matches the expected type, and false otherwise. If there is no token, it also returns false.
macro_rules! is_token {
    ($tokenstream:expr, $expected:pat) => {
        match $tokenstream.peek() {
            Some(token) => {
                matches!(&token.variant, $expected)
            }
            None => false,
        }
    };
}
pub(crate) use is_token;

/// The macro is used to check if the nth token matches the expected pattern.
/// It simplifies the code by reducing the boilerplate of checking for the presence of a token and comparing it to an expected value.
///
/// Example usage: `nth_is_token!(tokens, index, TokenType::Let)` checks if the token at `index` in `tokens` is of type `TokenType::Let`.
///
/// It return true if the token matches the expected type, and false otherwise. If there is no token at the given index, it also returns false.
macro_rules! nth_is_token {
    ($tokenstream:expr, $n:expr, $expected:pat) => {
        match $tokenstream.peek_nth($n) {
            Some(token) => {
                matches!(&token.variant, $expected)
            }
            None => false,
        }
    };
}
pub(crate) use nth_is_token;

/// The macro is used to expect a specific token from the token stream. It checks if the next token matches the expected pattern and returns it if it does.
///
/// Example usage: `expect_token!(tokens, TokenVariant::Let, "'let' keyword")` checks if the next token is of type `TokenVariant::Let` and returns it. If the token does not match, it returns a `Self::Error::UnexpectedToken` with a message and the position of the error. If there are no more tokens, it returns a `Self::Error::UnexpectedEOF` with a message.
macro_rules! expect_token {
    ($tokenstream:expr, $expected:pat, $expected_msg:expr) => {
        match $tokenstream.next() {
            Some(token) => {
                if matches!(&token.variant, $expected) {
                    token
                } else {
                    return Err(
                        crate::application::common::parser::error::ParsingError::UnexpectedToken {
                            expected: $expected_msg.to_string(),
                            found: token.variant.clone(),
                            pos: token.pos.clone(),
                        },
                    );
                }
            }
            None => {
                return Err(
                    crate::application::common::parser::error::ParsingError::UnexpectedEOF {
                        expected: $expected_msg.to_string(),
                    },
                )
            }
        }
    };
}
pub(crate) use expect_token;

/// Macro to expect and parse a specific type
///
/// Usage: `let name = expect_parse!(tokens, index, Identifier, "variable name")`
macro_rules! expect_parse {
    ($tokenstream:expr, $expected_parse:ty, $expected_msg:expr) => {{
        <$expected_parse>::parse($tokenstream).map_err(|err| match err {
            crate::application::common::parser::error::ParsingError::UnexpectedToken {
                found,
                pos,
                ..
            } => crate::application::common::parser::error::ParsingError::UnexpectedToken {
                expected: $expected_msg.to_string(),
                found,
                pos,
            },
            crate::application::common::parser::error::ParsingError::UnexpectedEOF { .. } => {
                crate::application::common::parser::error::ParsingError::UnexpectedEOF {
                    expected: $expected_msg.to_string(),
                }
            }
            crate::application::common::parser::error::ParsingError::SyntaxError {
                message,
                pos,
            } => crate::application::common::parser::error::ParsingError::SyntaxError {
                message,
                pos,
            },
        })?
    }};
}
pub(crate) use expect_parse;

/// Macro to get the position of the current token for error reporting
///
/// Usage: `let pos = get_pos!(tokens, index)`
macro_rules! get_pos {
    ($tokenstream:expr) => {
        $tokenstream.peek().map(|t| t.pos).ok_or_else(|| {
            crate::application::common::parser::error::ParsingError::UnexpectedEOF {
                expected: "Unexpected EOF".to_string(),
            }
        })?
    };
}
pub(crate) use get_pos;

/// Create a token stream from TokenVariant arguments for testing purposes
///
/// Usage: `let tokens = test_token_stream!(TokenVariant::Let, TokenVariant::Identifier("x".to_string()))`
///
/// The position (line) of the tokens will be incremented for each token, starting from 0. This is useful for testing the parser without having to go through the lexer.
/// The first_char (column) of the tokens will be set to 0 for simplicity.
macro_rules! test_token_stream {
    ($($variant:expr),*) => {
        {
            let tokens = vec![$($variant),*]
                .into_iter()
                .enumerate()
                .map(|(i, variant)| Token {
                    variant,
                    pos: crate::domain::common::position::Position::new(i, 0),
                })
                .collect::<Vec<_>>();

            crate::application::common::parser::lexer::token_stream::TokenStream::new(tokens)
        }
    };
}
pub(crate) use test_token_stream;
