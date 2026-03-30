use crate::application::plugin::common::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub lexeme: LexemeType,
    pub pos: Position,
}

impl Lexeme {
    pub fn new(lexeme: LexemeType, pos: Position) -> Self {
        Lexeme { lexeme, pos }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexemeType {
    Text(String),
    Number(String),
    NumberWithDot(String),
    Quote(String),
    Symbol(String),
    DoubleSymbol(String),
}

fn is_symbol_char(char: char) -> bool {
    let symbols = [
        '=', '+', '-', '*', '/', '%', '^', '!', '&', '|', '<', '>', ';', '(', ')', '{', '}', '[',
        ']', ',', '.', ':', '?',
    ];
    symbols.contains(&char)
}

fn is_double_symbol(first: &str, second: char) -> bool {
    let double_symbols = [
        "==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "^=", "=>", "->",
    ];
    let candidate = format!("{}{}", first, second);
    double_symbols.contains(&candidate.as_str())
}

struct Scanner {
    state: ScannerState,
    buffer: String,
    current_line: usize,
    current_column: usize,
    lexeme_start_line: usize,
    lexeme_start_column: usize,
}

enum ScannerState {
    None,

    Text,

    Number,
    NumberWithDot,

    Quote,
    ClosedQuote,

    Symbol,
    DoubleSymbol,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            state: ScannerState::None,
            buffer: String::new(),
            current_line: 0,
            current_column: 0,
            lexeme_start_line: 0,
            lexeme_start_column: 0,
        }
    }

    fn step_next_lexeme(&mut self, char: char) -> Option<LexemeType> {
        // Record starting position for the new lexeme at current character position
        self.lexeme_start_column = self.current_column;
        self.lexeme_start_line = self.current_line;

        // Reset state and buffer for the next lexeme
        self.state = ScannerState::None;
        self.buffer.clear();

        if char.is_whitespace() {
            None
        } else if char.is_numeric() {
            self.state = ScannerState::Number;
            self.buffer.push(char);
            None
        } else if char.is_alphabetic() || char == '_' {
            self.state = ScannerState::Text;
            self.buffer.push(char);
            None
        } else if char == '"' {
            self.state = ScannerState::Quote;
            None
        } else if is_symbol_char(char) {
            self.state = ScannerState::Symbol;
            self.buffer.push(char);
            None
        } else {
            // Handle unexpected characters (could be an error or ignored)
            None
        }
    }

    fn step_text(&mut self, char: char) -> Option<LexemeType> {
        if char.is_alphanumeric() || char == '_' || char == '-' {
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeType::Text(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_number(&mut self, char: char) -> Option<LexemeType> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else if char == '.' {
            self.state = ScannerState::NumberWithDot;
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeType::Number(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_number_with_dot(&mut self, char: char) -> Option<LexemeType> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeType::NumberWithDot(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_quote(&mut self, char: char) -> Option<LexemeType> {
        if char != '"' {
            self.buffer.push(char);
            None
        } else {
            self.state = ScannerState::ClosedQuote;
            None
        }
    }

    fn step_closed_quote(&mut self, char: char) -> Option<LexemeType> {
        let lexeme = LexemeType::Quote(self.buffer.clone());
        self.step_next_lexeme(char);
        Some(lexeme)
    }

    fn step_symbol(&mut self, char: char) -> Option<LexemeType> {
        if is_double_symbol(&self.buffer, char) {
            self.state = ScannerState::DoubleSymbol;
            self.buffer.push(char);
            None
        } else {
            let lexeme = LexemeType::Symbol(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_double_symbol(&mut self, char: char) -> Option<LexemeType> {
        let lexeme = LexemeType::DoubleSymbol(self.buffer.clone());
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
                ScannerState::Text => Some(LexemeType::Text(self.buffer.clone())),
                ScannerState::Number => Some(LexemeType::Number(self.buffer.clone())),
                ScannerState::NumberWithDot => Some(LexemeType::NumberWithDot(self.buffer.clone())),
                ScannerState::Quote => Some(LexemeType::Quote(self.buffer.clone())),
                ScannerState::ClosedQuote => Some(LexemeType::Quote(self.buffer.clone())),
                ScannerState::Symbol => Some(LexemeType::Symbol(self.buffer.clone())),
                ScannerState::DoubleSymbol => Some(LexemeType::DoubleSymbol(self.buffer.clone())),
                _ => None,
            };

            // Reset for next line (column stays at next position for newline)
            self.state = ScannerState::None;
            self.buffer.clear();
            self.current_line += 1;
            self.current_column = 0; // Next character on new line starts at 0

            result
        } else {
            match self.state {
                ScannerState::None => self.step_next_lexeme(char),
                ScannerState::Text => self.step_text(char),
                ScannerState::Number => self.step_number(char),
                ScannerState::NumberWithDot => self.step_number_with_dot(char),
                ScannerState::Quote => self.step_quote(char),
                ScannerState::ClosedQuote => self.step_closed_quote(char),
                ScannerState::Symbol => self.step_symbol(char),
                ScannerState::DoubleSymbol => self.step_double_symbol(char),
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
            ScannerState::Text => Some(LexemeType::Text(self.buffer.clone())),
            ScannerState::Number => Some(LexemeType::Number(self.buffer.clone())),
            ScannerState::NumberWithDot => Some(LexemeType::NumberWithDot(self.buffer.clone())),
            ScannerState::Quote => Some(LexemeType::Quote(self.buffer.clone())),
            ScannerState::ClosedQuote => Some(LexemeType::Quote(self.buffer.clone())),
            ScannerState::Symbol => Some(LexemeType::Symbol(self.buffer.clone())),
            ScannerState::DoubleSymbol => Some(LexemeType::DoubleSymbol(self.buffer.clone())),
            _ => None,
        };

        Some(Lexeme::new(
            lexeme_type?,
            Position::new(self.lexeme_start_line, self.lexeme_start_column),
        ))
    }
}

pub fn scan_source_code(source: &str) -> Vec<Lexeme> {
    let mut lexemes: Vec<Lexeme> = Vec::new();

    let mut scanner = Scanner::new();
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
                LexemeType::Text("hello".to_string()),
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
                Lexeme::new(LexemeType::Text("hello".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("world".to_string()), Position::new(0, 6)),
            ]
        );
    }

    #[test]
    fn test_text_with_underscores() {
        let result = scan_source_code("hello_world");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeType::Text("hello_world".to_string()),
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
                LexemeType::Text("hello-world".to_string()),
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
                LexemeType::Text("test123".to_string()),
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
                LexemeType::Number("5".to_string()),
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
                LexemeType::Number("12345".to_string()),
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
                LexemeType::NumberWithDot("123.45".to_string()),
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
                Lexeme::new(LexemeType::Number("1".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Number("3".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_zero_decimal() {
        let result = scan_source_code("0.0");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeType::NumberWithDot("0.0".to_string()),
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
                LexemeType::Quote("hello".to_string()),
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
                LexemeType::Quote("hello world".to_string()),
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
                LexemeType::Quote("test 123".to_string()),
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
                LexemeType::Quote("a+b".to_string()),
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
                Lexeme::new(LexemeType::Quote("first".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Quote("second".to_string()), Position::new(0, 8)),
            ]
        );
    }

    #[test]
    fn test_empty_quoted_string() {
        let result = scan_source_code("\"\"");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeType::Quote("".to_string()),
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
                LexemeType::Symbol("+".to_string()),
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
                LexemeType::Symbol("=".to_string()),
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
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("-".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Symbol("*".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_parentheses() {
        let result = scan_source_code("( )");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_parentheses_no_spaces() {
        let result = scan_source_code("()");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 1)),
            ]
        );
    }

    #[test]
    fn test_brackets() {
        let result = scan_source_code("[ ]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("]".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_brackets_no_spaces() {
        let result = scan_source_code("[]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("]".to_string()), Position::new(0, 1)),
            ]
        );
    }

    #[test]
    fn test_braces() {
        let result = scan_source_code("{ }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_braces_no_spaces() {
        let result = scan_source_code("{}");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(0, 1)),
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
                LexemeType::DoubleSymbol("==".to_string()),
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
                LexemeType::DoubleSymbol("+=".to_string()),
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
                Lexeme::new(LexemeType::Number("10".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeType::Number("5".to_string()), Position::new(0, 5)),
            ]
        );
    }

    #[test]
    fn test_equation() {
        let result = scan_source_code("x = 10");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("x".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("=".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Number("10".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let result = scan_source_code("func(1, 2)");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("func".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Number("1".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 9)),
            ]
        );
    }

    #[test]
    fn test_function_call_no_spaces() {
        let result = scan_source_code("func(1,2)");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("func".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Number("1".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 8)),
            ]
        );
    }

    #[test]
    fn test_complex_comparison() {
        let result = scan_source_code("if x >= 5 && y <= 10");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("if".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("x".to_string()), Position::new(0, 3)),
                Lexeme::new(
                    LexemeType::DoubleSymbol(">=".to_string()),
                    Position::new(0, 5)
                ),
                Lexeme::new(LexemeType::Number("5".to_string()), Position::new(0, 8)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("&&".to_string()),
                    Position::new(0, 10)
                ),
                Lexeme::new(LexemeType::Text("y".to_string()), Position::new(0, 13)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("<=".to_string()),
                    Position::new(0, 15)
                ),
                Lexeme::new(LexemeType::Number("10".to_string()), Position::new(0, 18)),
            ]
        );
    }

    #[test]
    fn test_mathematical_expression() {
        let result = scan_source_code("( a + b ) * c");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Symbol("*".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeType::Text("c".to_string()), Position::new(0, 12)),
            ]
        );
    }

    #[test]
    fn test_string_with_text_and_numbers() {
        let result = scan_source_code("price \"$100\"");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("price".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Quote("$100".to_string()), Position::new(0, 6)),
            ]
        );
    }

    #[test]
    fn test_assignment_with_decimal() {
        let result = scan_source_code("value = 3.14");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("value".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("=".to_string()), Position::new(0, 6)),
                Lexeme::new(
                    LexemeType::NumberWithDot("3.14".to_string()),
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
                Lexeme::new(LexemeType::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Number("1".to_string()), Position::new(0, 1)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Number("3".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeType::Symbol("]".to_string()), Position::new(0, 6)),
            ]
        );
    }

    #[test]
    fn test_array_with_spaces() {
        let result = scan_source_code("[ 1 , 2 , 3 ]");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("[".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Number("1".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Number("3".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeType::Symbol("]".to_string()), Position::new(0, 12)),
            ]
        );
    }

    #[test]
    fn test_all_operator_types() {
        let result = scan_source_code("+ - * / % ^ ! &");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol("-".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Symbol("*".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Symbol("/".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Symbol("%".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Symbol("^".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeType::Symbol("!".to_string()), Position::new(0, 12)),
                Lexeme::new(LexemeType::Symbol("&".to_string()), Position::new(0, 14)),
            ]
        );
    }

    #[test]
    fn test_not_equals() {
        let result = scan_source_code("!=");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeType::DoubleSymbol("!=".to_string()),
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
                Lexeme::new(LexemeType::Number("123".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("abc".to_string()), Position::new(0, 3)),
            ]
        );
    }

    #[test]
    fn test_underscore_only() {
        let result = scan_source_code("_");
        assert_eq!(
            result,
            vec![Lexeme::new(
                LexemeType::Text("_".to_string()),
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
                LexemeType::Text("___".to_string()),
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
                LexemeType::Text("_123text".to_string()),
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
                Lexeme::new(LexemeType::Text("text".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(".".to_string()), Position::new(0, 4)),
                Lexeme::new(LexemeType::Text("method".to_string()), Position::new(0, 5)),
            ]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = scan_source_code("a ; b");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 2)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 4)),
            ]
        );
    }

    #[test]
    fn test_semicolon_separator_no_spaces() {
        let result = scan_source_code("a;b");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 1)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 2)),
            ]
        );
    }

    #[test]
    fn test_semicolon_end_of_line() {
        let result = scan_source_code("a;\nhello;");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 1)),
                Lexeme::new(LexemeType::Text("hello".to_string()), Position::new(1, 0)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(1, 5)),
            ]
        );
    }

    #[test]
    fn test_mixed_operators() {
        let result = scan_source_code("a += 5; b -= 3; c *= 2; d /= 4; e %= 3; f ^= 2;");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 0)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("+=".to_string()),
                    Position::new(0, 2)
                ),
                Lexeme::new(LexemeType::Number("5".to_string()), Position::new(0, 5)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 8)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("-=".to_string()),
                    Position::new(0, 10)
                ),
                Lexeme::new(LexemeType::Number("3".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 14)),
                Lexeme::new(LexemeType::Text("c".to_string()), Position::new(0, 16)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("*=".to_string()),
                    Position::new(0, 18)
                ),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 21)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 22)),
                Lexeme::new(LexemeType::Text("d".to_string()), Position::new(0, 24)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("/=".to_string()),
                    Position::new(0, 26)
                ),
                Lexeme::new(LexemeType::Number("4".to_string()), Position::new(0, 29)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 30)),
                Lexeme::new(LexemeType::Text("e".to_string()), Position::new(0, 32)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("%=".to_string()),
                    Position::new(0, 34)
                ),
                Lexeme::new(LexemeType::Number("3".to_string()), Position::new(0, 37)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 38)),
                Lexeme::new(LexemeType::Text("f".to_string()), Position::new(0, 40)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("^=".to_string()),
                    Position::new(0, 42)
                ),
                Lexeme::new(LexemeType::Number("2".to_string()), Position::new(0, 45)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 46)),
            ]
        );
    }

    #[test]
    fn test_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int { return a + b; }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("fn".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("add".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeType::Symbol(":".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 15)),
                Lexeme::new(LexemeType::Symbol(":".to_string()), Position::new(0, 16)),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 18)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 21)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("->".to_string()),
                    Position::new(0, 23)
                ),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 26)),
                Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(0, 30)),
                Lexeme::new(LexemeType::Text("return".to_string()), Position::new(0, 32)),
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 39)),
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 41)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 43)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 44)),
                Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(0, 46)),
            ]
        );
    }

    #[test]
    fn test_reject_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int? { return a + b; }");
        assert_eq!(
            result,
            vec![
                Lexeme::new(LexemeType::Text("fn".to_string()), Position::new(0, 0)),
                Lexeme::new(LexemeType::Text("add".to_string()), Position::new(0, 3)),
                Lexeme::new(LexemeType::Symbol("(".to_string()), Position::new(0, 6)),
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 7)),
                Lexeme::new(LexemeType::Symbol(":".to_string()), Position::new(0, 8)),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 10)),
                Lexeme::new(LexemeType::Symbol(",".to_string()), Position::new(0, 13)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 15)),
                Lexeme::new(LexemeType::Symbol(":".to_string()), Position::new(0, 16)),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 18)),
                Lexeme::new(LexemeType::Symbol(")".to_string()), Position::new(0, 21)),
                Lexeme::new(
                    LexemeType::DoubleSymbol("->".to_string()),
                    Position::new(0, 23)
                ),
                Lexeme::new(LexemeType::Text("int".to_string()), Position::new(0, 26)),
                Lexeme::new(LexemeType::Symbol("?".to_string()), Position::new(0, 29)),
                Lexeme::new(LexemeType::Symbol("{".to_string()), Position::new(0, 31)),
                Lexeme::new(LexemeType::Text("return".to_string()), Position::new(0, 33)),
                Lexeme::new(LexemeType::Text("a".to_string()), Position::new(0, 40)),
                Lexeme::new(LexemeType::Symbol("+".to_string()), Position::new(0, 42)),
                Lexeme::new(LexemeType::Text("b".to_string()), Position::new(0, 44)),
                Lexeme::new(LexemeType::Symbol(";".to_string()), Position::new(0, 45)),
                Lexeme::new(LexemeType::Symbol("}".to_string()), Position::new(0, 47)),
            ]
        );
    }
}
