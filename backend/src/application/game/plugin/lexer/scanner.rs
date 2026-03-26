#[derive(Debug, Clone, PartialEq)]
pub struct LexemeWithPosition {
    pub lexeme: Lexeme,
    pub line: usize,
    pub first_char: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme {
    Text(String),
    Number(String),
    NumberWithDot(String),
    Quote(String),
    Symbol(String),
    DoubleSymbol(String)
}

fn is_symbol_char(char: char) -> bool {
    let symbols = ['=', '+', '-', '*', '/', '%', '^', '!', '&', '|', '<', '>', ';', '(', ')', '{', '}', '[', ']', ',', '.', ':', '?'];
    symbols.contains(&char)
}

fn is_double_symbol(first: &str, second: char) -> bool {
    let double_symbols = ["==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "^=", "=>", "->"];
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

    fn step_next_lexeme(&mut self, char: char) -> Option<Lexeme> {
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

    fn step_text(&mut self, char: char) -> Option<Lexeme> {
        if char.is_alphanumeric() || char == '_' || char == '-' {
            self.buffer.push(char);
            None
        } else {
            let lexeme = Lexeme::Text(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_number(&mut self, char: char) -> Option<Lexeme> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else if char == '.' {
            self.state = ScannerState::NumberWithDot;
            self.buffer.push(char);
            None
        } else {
            let lexeme = Lexeme::Number(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_number_with_dot(&mut self, char: char) -> Option<Lexeme> {
        if char.is_numeric() {
            self.buffer.push(char);
            None
        } else {
            let lexeme = Lexeme::NumberWithDot(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_quote(&mut self, char: char) -> Option<Lexeme> {
        if char != '"' {
            self.buffer.push(char);
            None
        } else {
            self.state = ScannerState::ClosedQuote;
            None
        }
    }

    fn step_closed_quote(&mut self, char: char) -> Option<Lexeme> {
        let lexeme = Lexeme::Quote(self.buffer.clone());
        self.step_next_lexeme(char);
        Some(lexeme)
    }

    fn step_symbol(&mut self, char: char) -> Option<Lexeme> {
        if is_double_symbol(&self.buffer, char) {
            self.state = ScannerState::DoubleSymbol;
            self.buffer.push(char);
            None
        } else {
            let lexeme = Lexeme::Symbol(self.buffer.clone());
            self.step_next_lexeme(char);
            Some(lexeme)
        }
    }

    fn step_double_symbol(&mut self, char: char) -> Option<Lexeme> {
        let lexeme = Lexeme::DoubleSymbol(self.buffer.clone());
        self.step_next_lexeme(char);
        Some(lexeme)
    }

    fn step(&mut self, char: char) -> Option<LexemeWithPosition> {
        // Save current lexeme position before it might be modified
        let lexeme_line = self.lexeme_start_line;
        let lexeme_col = self.lexeme_start_column;

        let is_newline = char == '\n';
        
        let lexeme = if is_newline {
            // Handle newline: finalize current lexeme if any
            let result = match self.state {
                ScannerState::Text => Some(Lexeme::Text(self.buffer.clone())),
                ScannerState::Number => Some(Lexeme::Number(self.buffer.clone())),
                ScannerState::NumberWithDot => Some(Lexeme::NumberWithDot(self.buffer.clone())),
                ScannerState::Quote => Some(Lexeme::Quote(self.buffer.clone())),
                ScannerState::ClosedQuote => Some(Lexeme::Quote(self.buffer.clone())),
                ScannerState::Symbol => Some(Lexeme::Symbol(self.buffer.clone())),
                ScannerState::DoubleSymbol => Some(Lexeme::DoubleSymbol(self.buffer.clone())),
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
            Some(LexemeWithPosition {
                lexeme: lex,
                line: lexeme_line,
                first_char: lexeme_col,
            })
        } else {
            // Only increment column for non-newline characters
            if !is_newline {
                self.current_column += 1;
            }
            None
        }
    }

    fn last_step(&mut self) -> Option<LexemeWithPosition> {
        let lexeme = match self.state {
            ScannerState::Text => Some(Lexeme::Text(self.buffer.clone())),
            ScannerState::Number => Some(Lexeme::Number(self.buffer.clone())),
            ScannerState::NumberWithDot => Some(Lexeme::NumberWithDot(self.buffer.clone())),
            ScannerState::Quote => Some(Lexeme::Quote(self.buffer.clone())),
            ScannerState::ClosedQuote => Some(Lexeme::Quote(self.buffer.clone())),
            ScannerState::Symbol => Some(Lexeme::Symbol(self.buffer.clone())),
            ScannerState::DoubleSymbol => Some(Lexeme::DoubleSymbol(self.buffer.clone())),
            _ => None,
        };

        lexeme.map(|lex| LexemeWithPosition {
            lexeme: lex,
            line: self.lexeme_start_line,
            first_char: self.lexeme_start_column,
        })
    }
}

pub fn scan_source_code(source: &str) -> Vec<LexemeWithPosition> {
    let mut lexemes: Vec<LexemeWithPosition> = Vec::new();

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
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("hello".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multiple_words() {
        let result = scan_source_code("hello world");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("hello".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("world".to_string()),
                line: 0,
                first_char: 6,
            },
        ]);
    }

    #[test]
    fn test_text_with_underscores() {
        let result = scan_source_code("hello_world");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("hello_world".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_text_with_hyphens() {
        let result = scan_source_code("hello-world");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("hello-world".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_text_with_numbers() {
        let result = scan_source_code("test123");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("test123".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    // Number Tests
    #[test]
    fn test_single_digit() {
        let result = scan_source_code("5");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Number("5".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multi_digit_number() {
        let result = scan_source_code("12345");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Number("12345".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_decimal_number() {
        let result = scan_source_code("123.45");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::NumberWithDot("123.45".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multiple_numbers() {
        let result = scan_source_code("1 2 3");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Number("1".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("3".to_string()),
                line: 0,
                first_char: 4,
            },
        ]);
    }

    #[test]
    fn test_zero_decimal() {
        let result = scan_source_code("0.0");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::NumberWithDot("0.0".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    // Quote Tests
    #[test]
    fn test_simple_quoted_string() {
        let result = scan_source_code("\"hello\"");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Quote("hello".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_quoted_string_with_spaces() {
        let result = scan_source_code("\"hello world\"");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Quote("hello world".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_quoted_string_with_numbers() {
        let result = scan_source_code("\"test 123\"");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Quote("test 123".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_quoted_string_with_symbols() {
        let result = scan_source_code("\"a+b\"");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Quote("a+b".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multiple_quoted_strings() {
        let result = scan_source_code("\"first\" \"second\"");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Quote("first".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Quote("second".to_string()),
                line: 0,
                first_char: 8,
            },
        ]);
    }

    #[test]
    fn test_empty_quoted_string() {
        let result = scan_source_code("\"\"");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Quote("".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    // Symbol Tests
    #[test]
    fn test_single_symbol() {
        let result = scan_source_code("+");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Symbol("+".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_equals_symbol() {
        let result = scan_source_code("=");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Symbol("=".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multiple_single_symbols() {
        let result = scan_source_code("+ - *");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("+".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("-".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("*".to_string()),
                line: 0,
                first_char: 4,
            },
        ]);
    }

    #[test]
    fn test_parentheses() {
        let result = scan_source_code("( )");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 2,
            },
        ]);
    }

    #[test]
    fn test_parentheses_no_spaces() {
        let result = scan_source_code("()");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 1,
            },
        ]);
    }

    #[test]
    fn test_brackets() {
        let result = scan_source_code("[ ]");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("[".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("]".to_string()),
                line: 0,
                first_char: 2,
            },
        ]);
    }

    #[test]
    fn test_brackets_no_spaces() {
        let result = scan_source_code("[]");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("[".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("]".to_string()),
                line: 0,
                first_char: 1,
            },
        ]);
    }

    #[test]
    fn test_braces() {
        let result = scan_source_code("{ }");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("{".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("}".to_string()),
                line: 0,
                first_char: 2,
            },
        ]);
    }

    #[test]
    fn test_braces_no_spaces() {
        let result = scan_source_code("{}");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("{".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("}".to_string()),
                line: 0,
                first_char: 1,
            },
        ]);
    }

    // Double Symbol Tests
    #[test]
    fn test_double_equals() {
        let result = scan_source_code("==");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("==".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_plus_equals() {
        let result = scan_source_code("+=");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("+=".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_minus_equals() {
        let result = scan_source_code("-=");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("-=".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_less_than_equals() {
        let result = scan_source_code("<=");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("<=".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_greater_than_equals() {
        let result = scan_source_code(">=");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol(">=".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_logical_and() {
        let result = scan_source_code("&&");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("&&".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
    }

    #[test]
    fn test_logical_or() {
        let result = scan_source_code("||");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("||".to_string()),
                line: 0,
                first_char: 0,
            }
        ]);
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
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Number("10".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("+".to_string()),
                line: 0,
                first_char: 3,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("5".to_string()),
                line: 0,
                first_char: 5,
            },
        ]);
    }

    #[test]
    fn test_equation() {
        let result = scan_source_code("x = 10");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("x".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("=".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("10".to_string()),
                line: 0,
                first_char: 4,
            },
        ]);
    }

    #[test]
    fn test_function_call() {
        let result = scan_source_code("func(1, 2)");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("func".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("1".to_string()),
                line: 0,
                first_char: 5,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 9,
            },
        ]);
    }

    #[test]
    fn test_function_call_no_spaces() {
        let result = scan_source_code("func(1,2)");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("func".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("1".to_string()),
                line: 0,
                first_char: 5,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 7,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 8,
            },
        ]);
    }

    #[test]
    fn test_complex_comparison() {
        let result = scan_source_code("if x >= 5 && y <= 10");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("if".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("x".to_string()),
                line: 0,
                first_char: 3,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol(">=".to_string()),
                line: 0,
                first_char: 5,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("5".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("&&".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("y".to_string()),
                line: 0,
                first_char: 13,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("<=".to_string()),
                line: 0,
                first_char: 15,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("10".to_string()),
                line: 0,
                first_char: 18,
            },
        ]);
    }

    #[test]
    fn test_mathematical_expression() {
        let result = scan_source_code("( a + b ) * c");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("+".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("*".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("c".to_string()),
                line: 0,
                first_char: 12,
            },
        ]);
    }

    #[test]
    fn test_string_with_text_and_numbers() {
        let result = scan_source_code("price \"$100\"");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("price".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Quote("$100".to_string()),
                line: 0,
                first_char: 6,
            },
        ]);
    }

    #[test]
    fn test_assignment_with_decimal() {
        let result = scan_source_code("value = 3.14");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("value".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("=".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::NumberWithDot("3.14".to_string()),
                line: 0,
                first_char: 8,
            },
        ]);
    }

    #[test]
    fn test_array_initialization() {
        let result = scan_source_code("[1,2,3]");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("[".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("1".to_string()),
                line: 0,
                first_char: 1,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 3,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("3".to_string()),
                line: 0,
                first_char: 5,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("]".to_string()),
                line: 0,
                first_char: 6,
            },
        ]);
    }

    #[test]
    fn test_array_with_spaces() {
        let result = scan_source_code("[ 1 , 2 , 3 ]");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("[".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("1".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("3".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("]".to_string()),
                line: 0,
                first_char: 12,
            },
        ]);
    }

    #[test]
    fn test_all_operator_types() {
        let result = scan_source_code("+ - * / % ^ ! &");
        assert_eq!(result.len(), 8);
        assert!(result.contains(&LexemeWithPosition {
            lexeme: Lexeme::Symbol("+".to_string()),
            line: 0,
            first_char: 0,
        }));
        assert!(result.contains(&LexemeWithPosition {
            lexeme: Lexeme::Symbol("-".to_string()),
            line: 0,
            first_char: 2,
        }));
        assert!(result.contains(&LexemeWithPosition {
            lexeme: Lexeme::Symbol("*".to_string()),
            line: 0,
            first_char: 4,
        }));
        assert!(result.contains(&LexemeWithPosition {
            lexeme: Lexeme::Symbol("/".to_string()),
            line: 0,
            first_char: 6,
        }));
    }

    #[test]
    fn test_not_equals() {
        let result = scan_source_code("!=");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::DoubleSymbol("!=".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    // Edge Cases
    #[test]
    fn test_number_followed_by_text() {
        // Once scanner enters Number state, it can't transition to Text
        // This is expected behavior in the current implementation
        let result = scan_source_code("123abc");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Number("123".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("abc".to_string()),
                line: 0,
                first_char: 3,
            }
        ]);
    }

    #[test]
    fn test_underscore_only() {
        let result = scan_source_code("_");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("_".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_multiple_underscores() {
        let result = scan_source_code("___");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("___".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_text_starts_with_number_like_char() {
        let result = scan_source_code("_123text");
        assert_eq!(result, vec![LexemeWithPosition {
            lexeme: Lexeme::Text("_123text".to_string()),
            line: 0,
            first_char: 0,
        }]);
    }

    #[test]
    fn test_dot_after_non_number_becomes_token() {
        // Dot is now recognized as a symbol
        let result = scan_source_code("text.method");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("text".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(".".to_string()),
                line: 0,
                first_char: 4,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("method".to_string()),
                line: 0,
                first_char: 5,
            },
        ]);
    }

    #[test]
    fn test_semicolon_separator() {
        let result = scan_source_code("a ; b");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 4,
            },
        ]);
    }

    #[test]
    fn test_semicolon_separator_no_spaces() {
        let result = scan_source_code("a;b");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 1,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 2,
            },
        ]);
    }

    #[test]
    fn test_semicolon_end_of_line() {
        let result = scan_source_code("a;\nhello;");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 1,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("hello".to_string()),
                line: 1,        
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 1,
                first_char: 5,
            },
        ]);
    }

    #[test]
    fn test_mixed_operators() {
        let result = scan_source_code("a += 5; b -= 3; c *= 2; d /= 4; e %= 3; f ^= 2;");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("+=".to_string()),
                line: 0,
                first_char: 2,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("5".to_string()),
                line: 0,
                first_char: 5,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("-=".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("3".to_string()),
                line: 0,
                first_char: 13,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 14,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("c".to_string()),
                line: 0,
                first_char: 16,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("*=".to_string()),
                line: 0,
                first_char: 18,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 21,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 22,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("d".to_string()),
                line: 0,
                first_char: 24,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("/=".to_string()),
                line: 0,
                first_char: 26,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("4".to_string()),
                line: 0,
                first_char: 29,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 30,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("e".to_string()),
                line: 0,
                first_char: 32,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("%=".to_string()),
                line: 0,
                first_char: 34,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("3".to_string()),
                line: 0,
                first_char: 37,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 38,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("f".to_string()),
                line: 0,
                first_char: 40,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("^=".to_string()),
                line: 0,
                first_char: 42,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Number("2".to_string()),
                line: 0,
                first_char: 45,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 46,
            },
        ]);
    }

    #[test]
    fn test_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int { return a + b; }");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("fn".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("add".to_string()),
                line: 0,
                first_char: 3,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 7,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(":".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 13,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 15,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(":".to_string()),
                line: 0,
                first_char: 16,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 18,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 21,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("->".to_string()),
                line: 0,
                first_char: 23,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 26,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("{".to_string()),
                line: 0,
                first_char: 30,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("return".to_string()),
                line: 0,
                first_char: 32,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 39,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("+".to_string()),
                line: 0,
                first_char: 41,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 43,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 44,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("}".to_string()),
                line: 0,
                first_char: 46,
            },
        ]);
    }

    #[test]
    fn test_reject_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int? { return a + b; }");
        assert_eq!(result, vec![
            LexemeWithPosition {
                lexeme: Lexeme::Text("fn".to_string()),
                line: 0,
                first_char: 0,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("add".to_string()),
                line: 0,
                first_char: 3,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("(".to_string()),
                line: 0,
                first_char: 6,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 7,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(":".to_string()),
                line: 0,
                first_char: 8,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 10,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(",".to_string()),
                line: 0,
                first_char: 13,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 15,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(":".to_string()),
                line: 0,
                first_char: 16,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 18,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(")".to_string()),
                line: 0,
                first_char: 21,
            },
            LexemeWithPosition {
                lexeme: Lexeme::DoubleSymbol("->".to_string()),
                line: 0,
                first_char: 23,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("int".to_string()),
                line: 0,
                first_char: 26,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("?".to_string()),
                line: 0,
                first_char: 29,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("{".to_string()),
                line: 0,
                first_char: 31,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("return".to_string()),
                line: 0,
                first_char: 33,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("a".to_string()),
                line: 0,
                first_char: 40,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("+".to_string()),
                line: 0,
                first_char: 42,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Text("b".to_string()),
                line: 0,
                first_char: 44,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol(";".to_string()),
                line: 0,
                first_char: 45,
            },
            LexemeWithPosition {
                lexeme: Lexeme::Symbol("}".to_string()),
                line: 0,
                first_char: 47,
            },
        ]);
    }
}