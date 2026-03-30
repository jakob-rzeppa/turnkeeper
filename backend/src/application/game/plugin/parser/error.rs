use std::{error::Error, fmt::Display};

use crate::application::game::plugin::{common::Position, lexer::token::TokenVariant};

#[derive(Debug)]
pub enum ParsingError {
    SyntaxError {
        message: String,
        pos: Position,
    },
    UnexpectedToken {
        expected: String,
        found: TokenVariant,
        pos: Position,
    },
    UnexpectedEOF {
        expected: String,
    },
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ParsingError {
    pub fn message(&self) -> String {
        match self {
            ParsingError::SyntaxError { message, .. } => format!("Syntax error: {}", message),
            ParsingError::UnexpectedToken {
                expected, found, ..
            } => {
                format!(
                    "Unexpected token: Expected {}, but found {}",
                    expected, found
                )
            }
            ParsingError::UnexpectedEOF { expected } => {
                format!("Unexpected end of file. Expected {}", expected)
            }
        }
    }

    pub fn context_message(&self, source_code: &str) -> String {
        let lines: Vec<&str> = source_code.lines().collect();

        match self {
            ParsingError::SyntaxError { pos, .. } | ParsingError::UnexpectedToken { pos, .. } => {
                let line_num = pos.line() as usize;
                let col_num = pos.column() as usize;

                let mut result = String::new();

                // Previous 2 lines
                let start_idx = line_num.saturating_sub(2);
                for i in start_idx..line_num {
                    if i < lines.len() {
                        result.push_str(lines[i]);
                        result.push('\n');
                    }
                }

                // Separator
                result.push_str(&"=".repeat(80));
                result.push('\n');

                // Line with error
                result.push_str(lines[line_num]);
                result.push('\n');

                // Indicator
                result.push_str(&" ".repeat(col_num));
                result.push_str("^");
                result.push('\n');

                // Error message
                result.push_str(&self.message());
                result.push('\n');

                // Separator
                result.push_str(&"=".repeat(80));
                result.push('\n');

                // Next 2 lines
                for i in (line_num + 1)..=(line_num + 2) {
                    if i < lines.len() {
                        result.push_str(lines[i]);
                        result.push('\n');
                    }
                }

                result
            }
            ParsingError::UnexpectedEOF { .. } => "Unexpected end of file\n".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_error_message() {
        let source_code = "let x: ? = 42\nx = x + 1;";

        let error = ParsingError::UnexpectedToken {
            expected: "datatype in variable declaration".to_string(),
            found: TokenVariant::Question,
            pos: Position::new(0, 7),
        };

        println!("{}", error.context_message(source_code));
    }
}
