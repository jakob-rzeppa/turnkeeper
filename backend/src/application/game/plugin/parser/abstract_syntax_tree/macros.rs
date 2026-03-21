
/// Macro to expect and parse a specific type
/// 
/// Usage: `let name = expect_parse!(tokens, index, Identifier, "Expected variable name")`
macro_rules! expect_parse {
    ($tokens:expr, $index:expr, $expected_parse:ty, $error:expr) => {
        {
            let (parsed, new_index) = <$expected_parse>::parse($tokens, $index)?;
            $index = new_index;
            parsed
        }
    };
}

/// Macro to expect and consume a specific token
/// 
/// Usage: `expect_token!(tokens, index, Token::Semicolon, "Expected ';'")`
macro_rules! expect_token {
    ($tokens:expr, $index:expr, $expected:expr, $error:expr) => {
        {
            if $tokens.get($index) != Some(&$expected) {
                return Err($error.to_string());
            }
            $index += 1;
        }
    };
}