
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
/// Usage: `expect_token!(tokens, index, TokenType::Semicolon, "Expected ';'")`
macro_rules! expect_token {
    ($tokens:expr, $index:expr, $expected:expr, $error:expr) => {
        {
            if $tokens.get($index).map(|t| &t.token) != Some(&$expected) {
                return Err($error.to_string());
            }
            $index += 1;
        }
    };
}

/// Macro to get the position of the current token for error reporting
/// 
/// Usage: `let pos = get_pos!(tokens, index)`
macro_rules! get_pos {
    ($tokens:expr, $index:expr) => {
        $tokens.get($index).map(|t| t.pos).ok_or_else(|| "Unexpected EOF".to_string())?
    };
}