use crate::{
    application::common::parser::lexer::lexeme::{Lexeme, LexemeVariant},
    domain::common::position::Position,
};

pub fn scan_source_code(source: &str) -> Vec<Lexeme> {
    let mut lexemes: Vec<Lexeme> = Vec::new();

    let mut scanner = ScannerTransverser::new();
    for char in source.chars() {
        // Continue the current scanner FSM
        let lexeme = scanner.step(char);
        if let Some(lexeme) = lexeme {
            lexemes.push(lexeme);
        }
    }

    // Handle the final lexeme after processing all characters
    if let Some(lexeme) = scanner.last_step() {
        lexemes.push(lexeme);
    }

    lexemes
}

struct ScannerTransverser {
    state: ScannerTransverserState,
    buffer: String,
    current_line: usize,
    current_column: usize,
    lexeme_start_line: usize,
    lexeme_start_column: usize,
}

enum ScannerTransverserState {
    None,

    Text,

    Number,
    DecimalNumber,

    Quote,
    ClosedQuote,

    Symbol,
    DoubleSymbol,
}

impl ScannerTransverser {
    fn new() -> Self {
        ScannerTransverser {
            state: ScannerTransverserState::None,
            buffer: String::new(),
            current_line: 0,
            current_column: 0,
            lexeme_start_line: 0,
            lexeme_start_column: 0,
        }
    }

    fn step_next_lexeme(&mut self, char: char) -> Option<LexemeVariant> {
        // Record starting position for the new lexeme at current character position
        self.lexeme_start_column = self.current_column;
        self.lexeme_start_line = self.current_line;

        // Reset state and buffer for the next lexeme
        self.state = ScannerTransverserState::None;
        self.buffer.clear();

        if char.is_whitespace() {
            None
        } else if char.is_numeric() {
            self.state = ScannerTransverserState::Number;
            self.buffer.push(char);
            None
        } else if char.is_alphabetic() || char == '_' {
            self.state = ScannerTransverserState::Text;
            self.buffer.push(char);
            None
        } else if char == '"' {
            self.state = ScannerTransverserState::Quote;
            None
        } else if Lexeme::char_is_symbol(char) {
            self.state = ScannerTransverserState::Symbol;
            self.buffer.push(char);
            None
        } else {
            // Handle unexpected characters (could be an error or ignored)
            None
        }
    }

    fn step_text(&mut self, char: char) -> Option<LexemeVariant> {
        if char.is_alphanumeric() || char == '_' || char == '-' {
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeVariant::Text(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_number(&mut self, char: char) -> Option<LexemeVariant> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else if char == '.' {
            self.state = ScannerTransverserState::DecimalNumber;
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeVariant::Number(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_decimal_number(&mut self, char: char) -> Option<LexemeVariant> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeVariant::DecimalNumber(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_quote(&mut self, char: char) -> Option<LexemeVariant> {
        if char != '"' {
            self.buffer.push(char);
            None
        } else {
            self.state = ScannerTransverserState::ClosedQuote;
            None
        }
    }

    fn step_closed_quote(&mut self, char: char) -> Option<LexemeVariant> {
        let lexeme = LexemeVariant::Quote(self.buffer.clone());
        self.step_next_lexeme(char);
        Some(lexeme)
    }

    fn step_symbol(&mut self, char: char) -> Option<LexemeVariant> {
        if Lexeme::chars_are_double_symbol(self.buffer.chars().last().unwrap_or('\0'), char) {
            self.state = ScannerTransverserState::DoubleSymbol;
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeVariant::Symbol(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_double_symbol(&mut self, char: char) -> Option<LexemeVariant> {
        let lexeme = LexemeVariant::DoubleSymbol(self.buffer.clone());
        self.step_next_lexeme(char);
        Some(lexeme)
    }

    fn step(&mut self, char: char) -> Option<Lexeme> {
        // Save current lexeme position before it might be modified
        let lexeme_line = self.lexeme_start_line;
        let lexeme_col = self.lexeme_start_column;
        let is_newline = char == '\n';

        let lexeme = if is_newline {
            // Handle newline: finalize current lexeme if any
            let result = match self.state {
                ScannerTransverserState::Text => Some(LexemeVariant::Text(self.buffer.clone())),
                ScannerTransverserState::Number => Some(LexemeVariant::Number(self.buffer.clone())),
                ScannerTransverserState::DecimalNumber => {
                    Some(LexemeVariant::DecimalNumber(self.buffer.clone()))
                }
                ScannerTransverserState::Quote => Some(LexemeVariant::Quote(self.buffer.clone())),
                ScannerTransverserState::ClosedQuote => {
                    Some(LexemeVariant::Quote(self.buffer.clone()))
                }
                ScannerTransverserState::Symbol => Some(LexemeVariant::Symbol(self.buffer.clone())),
                ScannerTransverserState::DoubleSymbol => {
                    Some(LexemeVariant::DoubleSymbol(self.buffer.clone()))
                }
                _ => None,
            };

            // Reset for next line (column stays at next position for newline)
            self.state = ScannerTransverserState::None;
            self.buffer.clear();
            self.current_line += 1;
            self.current_column = 0; // Next character on new line starts at 0

            result
        } else {
            match self.state {
                ScannerTransverserState::None => self.step_next_lexeme(char),
                ScannerTransverserState::Text => self.step_text(char),
                ScannerTransverserState::Number => self.step_number(char),
                ScannerTransverserState::DecimalNumber => self.step_decimal_number(char),
                ScannerTransverserState::Quote => self.step_quote(char),
                ScannerTransverserState::ClosedQuote => self.step_closed_quote(char),
                ScannerTransverserState::Symbol => self.step_symbol(char),
                ScannerTransverserState::DoubleSymbol => self.step_double_symbol(char),
            }
        };

        // Build result with position (use saved position from before potential modification)
        if let Some(lex) = lexeme {
            // Only increment column for non-newline characters
            if !is_newline {
                self.current_column += 1;
            }
            Some(Lexeme::new(lex, Position::new(lexeme_line, lexeme_col)))
        } else {
            // Only increment column for non-newline characters
            if !is_newline {
                self.current_column += 1;
            }
            None
        }
    }

    fn last_step(&mut self) -> Option<Lexeme> {
        let lexeme_type = match self.state {
            ScannerTransverserState::Text => Some(LexemeVariant::Text(self.buffer.clone())),
            ScannerTransverserState::Number => Some(LexemeVariant::Number(self.buffer.clone())),
            ScannerTransverserState::DecimalNumber => {
                Some(LexemeVariant::DecimalNumber(self.buffer.clone()))
            }
            ScannerTransverserState::Quote => Some(LexemeVariant::Quote(self.buffer.clone())),
            ScannerTransverserState::ClosedQuote => Some(LexemeVariant::Quote(self.buffer.clone())),
            ScannerTransverserState::Symbol => Some(LexemeVariant::Symbol(self.buffer.clone())),
            ScannerTransverserState::DoubleSymbol => {
                Some(LexemeVariant::DoubleSymbol(self.buffer.clone()))
            }
            _ => None,
        };

        Some(Lexeme::new(
            lexeme_type?,
            Position::new(self.lexeme_start_line, self.lexeme_start_column),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic Text Tests
    #[test]
    fn test_single_word() {
        let result = scan_source_code("hello");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("hello".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multiple_words() {
        let result = scan_source_code("hello world");
        assert_eq!(
            result,
            vec![
                Lexeme::new(
                    LexemeVariant::Text("hello".to_string()),
                    Position::new(0, 0)
                ),
                Lexeme::new(
                    LexemeVariant::Text("world".to_string()),
                    Position::new(0, 6)
                ),
            ]
        );
    }

    #[test]
    fn test_text_with_underscores() {
        let result = scan_source_code("hello_world");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("hello_world".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_text_with_hyphens() {
        let result = scan_source_code("hello-world");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("hello-world".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_text_with_numbers() {
        let result = scan_source_code("test123");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("test123".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    // Number Tests
    #[test]
    fn test_single_digit() {
        let result = scan_source_code("5");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Number("5".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multi_digit_number() {
        let result = scan_source_code("12345");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Number("12345".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_decimal_number() {
        let result = scan_source_code("123.45");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::DecimalNumber("123.45".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multiple_numbers() {
        let result = scan_source_code("1 2 3");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Number("1".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Number("3".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_zero_decimal() {
        let result = scan_source_code("0.0");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::DecimalNumber("0.0".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    // Quote Tests
    #[test]
    fn test_simple_quoted_string() {
        let result = scan_source_code("\"hello\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Quote("hello".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_quoted_string_with_spaces() {
        let result = scan_source_code("\"hello world\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Quote("hello world".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_quoted_string_with_numbers() {
        let result = scan_source_code("\"test 123\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Quote("test 123".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_quoted_string_with_symbols() {
        let result = scan_source_code("\"a+b\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Quote("a+b".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multiple_quoted_strings() {
        let result = scan_source_code("\"first\" \"second\"");
        assert_eq!(
            result,
            vec![
                Lexeme::new(
                    LexemeVariant::Quote("first".to_string()),
                    Position::new(0, 0)
                ),
                Lexeme::new(
                    LexemeVariant::Quote("second".to_string()),
                    Position::new(0, 8)
                ),
            ]
        );
    }

    #[test]
    fn test_empty_quoted_string() {
        let result = scan_source_code("\"\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Quote("".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    // Symbol Tests
    #[test]
    fn test_single_symbol() {
        let result = scan_source_code("+");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Symbol("+".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_equals_symbol() {
        let result = scan_source_code("=");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Symbol("=".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multiple_single_symbols() {
        let result = scan_source_code("+ - *");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("-".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Symbol("*".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_parentheses() {
        let result = scan_source_code("( )");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_parentheses_no_spaces() {
        let result = scan_source_code("()");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 1)),
            ]
        );
    }

    #[test]
    fn test_brackets() {
        let result = scan_source_code("[ ]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("]".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_brackets_no_spaces() {
        let result = scan_source_code("[]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("]".to_string()), Position::new(0, 1)),
            ]
        );
    }

    #[test]
    fn test_braces() {
        let result = scan_source_code("{ }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("{".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("}".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_braces_no_spaces() {
        let result = scan_source_code("{}");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("{".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("}".to_string()), Position::new(0, 1)),
            ]
        );
    }

    // Double Symbol Tests
    #[test]
    fn test_double_equals() {
        let result = scan_source_code("==");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::DoubleSymbol("==".to_string()),
                Position::new(0, 0)
            ),]
        );
    }

    #[test]
    fn test_plus_equals() {
        let result = scan_source_code("+=");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::DoubleSymbol("+=".to_string()),
                Position::new(0, 0)
            ),]
        );
    }

    // Whitespace Tests
    #[test]
    fn test_empty_string() {
        let result = scan_source_code("");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_only_spaces() {
        let result = scan_source_code("   ");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_only_tabs() {
        let result = scan_source_code("\t\t");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_only_newlines() {
        let result = scan_source_code("\n\n");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_mixed_whitespace() {
        let result = scan_source_code(" \t\n ");
        assert_eq!(result, vec![]);
    }

    // Complex Expression Tests
    #[test]
    fn test_arithmetic_expression() {
        let result = scan_source_code("10 + 5");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Number("10".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeVariant::Number("5".to_string()), Position::new(0, 5)),
            ]
        );
    }

    #[test]
    fn test_equation() {
        let result = scan_source_code("x = 10");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("x".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("=".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Number("10".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let result = scan_source_code("func(1, 2)");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("func".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Number("1".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 9)),
            ]
        );
    }

    #[test]
    fn test_function_call_no_spaces() {
        let result = scan_source_code("func(1,2)");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("func".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Number("1".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 8)),
            ]
        );
    }

    #[test]
    fn test_complex_comparison() {
        let result = scan_source_code("if x >= 5 && y <= 10");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("if".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Text("x".to_string()), Position::new(0, 3)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol(">=".to_string()),
                    Position::new(0, 5)
                ),
                Lexeme::new(LexemeVariant::Number("5".to_string()), Position::new(0, 8)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("&&".to_string()),
                    Position::new(0, 10)
                ),
                Lexeme::new(LexemeVariant::Text("y".to_string()), Position::new(0, 13)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("<=".to_string()),
                    Position::new(0, 15)
                ),
                Lexeme::new(
                    LexemeVariant::Number("10".to_string()),
                    Position::new(0, 18)
                ),
            ]
        );
    }

    #[test]
    fn test_mathematical_expression() {
        let result = scan_source_code("( a + b ) * c");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Symbol("*".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeVariant::Text("c".to_string()), Position::new(0, 12)),
            ]
        );
    }

    #[test]
    fn test_string_with_text_and_numbers() {
        let result = scan_source_code("price \"$100\"");
        assert_eq!(
            result,
            vec![
                Lexeme::new(
                    LexemeVariant::Text("price".to_string()),
                    Position::new(0, 0)
                ),
                Lexeme::new(
                    LexemeVariant::Quote("$100".to_string()),
                    Position::new(0, 6)
                ),
            ]
        );
    }

    #[test]
    fn test_assignment_with_decimal() {
        let result = scan_source_code("value = 3.14");
        assert_eq!(
            result,
            vec![
                Lexeme::new(
                    LexemeVariant::Text("value".to_string()),
                    Position::new(0, 0)
                ),
                Lexeme::new(LexemeVariant::Symbol("=".to_string()), Position::new(0, 6)),
                Lexeme::new(
                    LexemeVariant::DecimalNumber("3.14".to_string()),
                    Position::new(0, 8)
                ),
            ]
        );
    }

    #[test]
    fn test_array_initialization() {
        let result = scan_source_code("[1,2,3]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Number("1".to_string()), Position::new(0, 1)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Number("3".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeVariant::Symbol("]".to_string()), Position::new(0, 6)),
            ]
        );
    }

    #[test]
    fn test_array_with_spaces() {
        let result = scan_source_code("[ 1 , 2 , 3 ]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Number("1".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Number("3".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeVariant::Symbol("]".to_string()), Position::new(0, 12)),
            ]
        );
    }

    #[test]
    fn test_all_operator_types() {
        let result = scan_source_code("+ - * / % ^ ! &");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol("-".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Symbol("*".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeVariant::Symbol("/".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Symbol("%".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Symbol("^".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeVariant::Symbol("!".to_string()), Position::new(0, 12)),
                Lexeme::new(LexemeVariant::Symbol("&".to_string()), Position::new(0, 14)),
            ]
        );
    }

    #[test]
    fn test_not_equals() {
        let result = scan_source_code("!=");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::DoubleSymbol("!=".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    // Edge Cases
    #[test]
    fn test_number_followed_by_text() {
        // Once scanner enters Number state, it can't transition to Text
        // This is expected behavior in the current implementation
        let result = scan_source_code("123abc");
        assert_eq!(
            result,
            vec![
                Lexeme::new(
                    LexemeVariant::Number("123".to_string()),
                    Position::new(0, 0)
                ),
                Lexeme::new(LexemeVariant::Text("abc".to_string()), Position::new(0, 3)),
            ]
        );
    }

    #[test]
    fn test_underscore_only() {
        let result = scan_source_code("_");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("_".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_multiple_underscores() {
        let result = scan_source_code("___");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("___".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_text_starts_with_number_like_char() {
        let result = scan_source_code("_123text");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("_123text".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_dot_after_non_number_becomes_token() {
        // Dot is now recognized as a symbol
        let result = scan_source_code("text.method");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("text".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(".".to_string()), Position::new(0, 4)),
                Lexeme::new(
                    LexemeVariant::Text("method".to_string()),
                    Position::new(0, 5)
                ),
            ]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = scan_source_code("a ; b");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_semicolon_separator_no_spaces() {
        let result = scan_source_code("a;b");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 1)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_semicolon_end_of_line() {
        let result = scan_source_code("a;\nhello;");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 1)),
                Lexeme::new(
                    LexemeVariant::Text("hello".to_string()),
                    Position::new(1, 0)
                ),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(1, 5)),
            ]
        );
    }

    #[test]
    fn test_mixed_operators() {
        let result = scan_source_code("a += 5; b -= 3; c *= 2; d /= 4; e %= 3; f ^= 2;");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("+=".to_string()),
                    Position::new(0, 2)
                ),
                Lexeme::new(LexemeVariant::Number("5".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 8)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("-=".to_string()),
                    Position::new(0, 10)
                ),
                Lexeme::new(LexemeVariant::Number("3".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 14)),
                Lexeme::new(LexemeVariant::Text("c".to_string()), Position::new(0, 16)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("*=".to_string()),
                    Position::new(0, 18)
                ),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 21)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 22)),
                Lexeme::new(LexemeVariant::Text("d".to_string()), Position::new(0, 24)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("/=".to_string()),
                    Position::new(0, 26)
                ),
                Lexeme::new(LexemeVariant::Number("4".to_string()), Position::new(0, 29)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 30)),
                Lexeme::new(LexemeVariant::Text("e".to_string()), Position::new(0, 32)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("%=".to_string()),
                    Position::new(0, 34)
                ),
                Lexeme::new(LexemeVariant::Number("3".to_string()), Position::new(0, 37)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 38)),
                Lexeme::new(LexemeVariant::Text("f".to_string()), Position::new(0, 40)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("^=".to_string()),
                    Position::new(0, 42)
                ),
                Lexeme::new(LexemeVariant::Number("2".to_string()), Position::new(0, 45)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 46)),
            ]
        );
    }

    #[test]
    fn test_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int { return a + b; }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("fn".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Text("add".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeVariant::Symbol(":".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 15)),
                Lexeme::new(LexemeVariant::Symbol(":".to_string()), Position::new(0, 16)),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 18)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 21)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("->".to_string()),
                    Position::new(0, 23)
                ),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 26)),
                Lexeme::new(LexemeVariant::Symbol("{".to_string()), Position::new(0, 30)),
                Lexeme::new(
                    LexemeVariant::Text("return".to_string()),
                    Position::new(0, 32)
                ),
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 39)),
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 41)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 43)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 44)),
                Lexeme::new(LexemeVariant::Symbol("}".to_string()), Position::new(0, 46)),
            ]
        );
    }

    #[test]
    fn test_reject_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int? { return a + b; }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeVariant::Text("fn".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeVariant::Text("add".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeVariant::Symbol("(".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeVariant::Symbol(":".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeVariant::Symbol(",".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 15)),
                Lexeme::new(LexemeVariant::Symbol(":".to_string()), Position::new(0, 16)),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 18)),
                Lexeme::new(LexemeVariant::Symbol(")".to_string()), Position::new(0, 21)),
                Lexeme::new(
                    LexemeVariant::DoubleSymbol("->".to_string()),
                    Position::new(0, 23)
                ),
                Lexeme::new(LexemeVariant::Text("int".to_string()), Position::new(0, 26)),
                Lexeme::new(LexemeVariant::Symbol("?".to_string()), Position::new(0, 29)),
                Lexeme::new(LexemeVariant::Symbol("{".to_string()), Position::new(0, 31)),
                Lexeme::new(
                    LexemeVariant::Text("return".to_string()),
                    Position::new(0, 33)
                ),
                Lexeme::new(LexemeVariant::Text("a".to_string()), Position::new(0, 40)),
                Lexeme::new(LexemeVariant::Symbol("+".to_string()), Position::new(0, 42)),
                Lexeme::new(LexemeVariant::Text("b".to_string()), Position::new(0, 44)),
                Lexeme::new(LexemeVariant::Symbol(";".to_string()), Position::new(0, 45)),
                Lexeme::new(LexemeVariant::Symbol("}".to_string()), Position::new(0, 47)),
            ]
        );
    }

    #[test]
    fn test_ignores_spaces_before_text() {
        let result = scan_source_code("   hello");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("hello".to_string()),
                Position::new(0, 3)
            )]
        );
    }

    #[test]
    fn test_ignores_spaces_after_text() {
        let result = scan_source_code("hello   ");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Text("hello".to_string()),
                Position::new(0, 0)
            )]
        );
    }

    #[test]
    fn test_ignores_spaces_around_symbol() {
        let result = scan_source_code("   +   ");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeVariant::Symbol("+".to_string()),
                Position::new(0, 3)
            )]
        );
    }
}
