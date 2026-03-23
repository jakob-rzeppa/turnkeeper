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
        }
    }

    fn step_next_lexeme(&mut self, char: char) -> Option<Lexeme> {
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

    fn step(&mut self, char: char) -> Option<Lexeme> {
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
    }

    fn last_step(&mut self) -> Option<Lexeme> {
        match self.state {
            ScannerState::Text => Some(Lexeme::Text(self.buffer.clone())),
            ScannerState::Number => Some(Lexeme::Number(self.buffer.clone())),
            ScannerState::NumberWithDot => Some(Lexeme::NumberWithDot(self.buffer.clone())),
            ScannerState::Quote => Some(Lexeme::Quote(self.buffer.clone())),
            ScannerState::ClosedQuote => Some(Lexeme::Quote(self.buffer.clone())),
            ScannerState::Symbol => Some(Lexeme::Symbol(self.buffer.clone())),
            ScannerState::DoubleSymbol => Some(Lexeme::DoubleSymbol(self.buffer.clone())),
            _ => None,
        }
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
        assert_eq!(result, vec![Lexeme::Text("hello".to_string())]);
    }

    #[test]
    fn test_multiple_words() {
        let result = scan_source_code("hello world");
        assert_eq!(result, vec![
            Lexeme::Text("hello".to_string()),
            Lexeme::Text("world".to_string()),
        ]);
    }

    #[test]
    fn test_text_with_underscores() {
        let result = scan_source_code("hello_world");
        assert_eq!(result, vec![Lexeme::Text("hello_world".to_string())]);
    }

    #[test]
    fn test_text_with_hyphens() {
        let result = scan_source_code("hello-world");
        assert_eq!(result, vec![Lexeme::Text("hello-world".to_string())]);
    }

    #[test]
    fn test_text_with_numbers() {
        let result = scan_source_code("test123");
        assert_eq!(result, vec![Lexeme::Text("test123".to_string())]);
    }

    // Number Tests
    #[test]
    fn test_single_digit() {
        let result = scan_source_code("5");
        assert_eq!(result, vec![Lexeme::Number("5".to_string())]);
    }

    #[test]
    fn test_multi_digit_number() {
        let result = scan_source_code("12345");
        assert_eq!(result, vec![Lexeme::Number("12345".to_string())]);
    }

    #[test]
    fn test_decimal_number() {
        let result = scan_source_code("123.45");
        assert_eq!(result, vec![Lexeme::NumberWithDot("123.45".to_string())]);
    }

    #[test]
    fn test_multiple_numbers() {
        let result = scan_source_code("1 2 3");
        assert_eq!(result, vec![
            Lexeme::Number("1".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Number("3".to_string()),
        ]);
    }

    #[test]
    fn test_zero_decimal() {
        let result = scan_source_code("0.0");
        assert_eq!(result, vec![Lexeme::NumberWithDot("0.0".to_string())]);
    }

    // Quote Tests
    #[test]
    fn test_simple_quoted_string() {
        let result = scan_source_code("\"hello\"");
        assert_eq!(result, vec![Lexeme::Quote("hello".to_string())]);
    }

    #[test]
    fn test_quoted_string_with_spaces() {
        let result = scan_source_code("\"hello world\"");
        assert_eq!(result, vec![Lexeme::Quote("hello world".to_string())]);
    }

    #[test]
    fn test_quoted_string_with_numbers() {
        let result = scan_source_code("\"test 123\"");
        assert_eq!(result, vec![Lexeme::Quote("test 123".to_string())]);
    }

    #[test]
    fn test_quoted_string_with_symbols() {
        let result = scan_source_code("\"a+b\"");
        assert_eq!(result, vec![Lexeme::Quote("a+b".to_string())]);
    }

    #[test]
    fn test_multiple_quoted_strings() {
        let result = scan_source_code("\"first\" \"second\"");
        assert_eq!(result, vec![
            Lexeme::Quote("first".to_string()),
            Lexeme::Quote("second".to_string()),
        ]);
    }

    #[test]
    fn test_empty_quoted_string() {
        let result = scan_source_code("\"\"");
        assert_eq!(result, vec![Lexeme::Quote("".to_string())]);
    }

    // Symbol Tests
    #[test]
    fn test_single_symbol() {
        let result = scan_source_code("+");
        assert_eq!(result, vec![Lexeme::Symbol("+".to_string())]);
    }

    #[test]
    fn test_equals_symbol() {
        let result = scan_source_code("=");
        assert_eq!(result, vec![Lexeme::Symbol("=".to_string())]);
    }

    #[test]
    fn test_multiple_single_symbols() {
        let result = scan_source_code("+ - *");
        assert_eq!(result, vec![
            Lexeme::Symbol("+".to_string()),
            Lexeme::Symbol("-".to_string()),
            Lexeme::Symbol("*".to_string()),
        ]);
    }

    #[test]
    fn test_parentheses() {
        let result = scan_source_code("( )");
        assert_eq!(result, vec![
            Lexeme::Symbol("(".to_string()),
            Lexeme::Symbol(")".to_string()),
        ]);
    }

    #[test]
    fn test_parentheses_no_spaces() {
        let result = scan_source_code("()");
        assert_eq!(result, vec![
            Lexeme::Symbol("(".to_string()),
            Lexeme::Symbol(")".to_string()),
        ]);
    }

    #[test]
    fn test_brackets() {
        let result = scan_source_code("[ ]");
        assert_eq!(result, vec![
            Lexeme::Symbol("[".to_string()),
            Lexeme::Symbol("]".to_string()),
        ]);
    }

    #[test]
    fn test_brackets_no_spaces() {
        let result = scan_source_code("[]");
        assert_eq!(result, vec![
            Lexeme::Symbol("[".to_string()),
            Lexeme::Symbol("]".to_string()),
        ]);
    }

    #[test]
    fn test_braces() {
        let result = scan_source_code("{ }");
        assert_eq!(result, vec![
            Lexeme::Symbol("{".to_string()),
            Lexeme::Symbol("}".to_string()),
        ]);
    }

    #[test]
    fn test_braces_no_spaces() {
        let result = scan_source_code("{}");
        assert_eq!(result, vec![
            Lexeme::Symbol("{".to_string()),
            Lexeme::Symbol("}".to_string()),
        ]);
    }

    // Double Symbol Tests
    #[test]
    fn test_double_equals() {
        let result = scan_source_code("==");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("==".to_string())]);
    }

    #[test]
    fn test_plus_equals() {
        let result = scan_source_code("+=");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("+=".to_string())]);
    }

    #[test]
    fn test_minus_equals() {
        let result = scan_source_code("-=");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("-=".to_string())]);
    }

    #[test]
    fn test_less_than_equals() {
        let result = scan_source_code("<=");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("<=".to_string())]);
    }

    #[test]
    fn test_greater_than_equals() {
        let result = scan_source_code(">=");
        assert_eq!(result, vec![Lexeme::DoubleSymbol(">=".to_string())]);
    }

    #[test]
    fn test_logical_and() {
        let result = scan_source_code("&&");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("&&".to_string())]);
    }

    #[test]
    fn test_logical_or() {
        let result = scan_source_code("||");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("||".to_string())]);
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
            Lexeme::Number("10".to_string()),
            Lexeme::Symbol("+".to_string()),
            Lexeme::Number("5".to_string()),
        ]);
    }

    #[test]
    fn test_equation() {
        let result = scan_source_code("x = 10");
        assert_eq!(result, vec![
            Lexeme::Text("x".to_string()),
            Lexeme::Symbol("=".to_string()),
            Lexeme::Number("10".to_string()),
        ]);
    }

    #[test]
    fn test_function_call() {
        let result = scan_source_code("func ( 1 , 2 )");
        assert_eq!(result, vec![
            Lexeme::Text("func".to_string()),
            Lexeme::Symbol("(".to_string()),
            Lexeme::Number("1".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(")".to_string()),
        ]);
    }

    #[test]
    fn test_function_call_no_spaces() {
        let result = scan_source_code("func(1,2)");
        assert_eq!(result, vec![
            Lexeme::Text("func".to_string()),
            Lexeme::Symbol("(".to_string()),
            Lexeme::Number("1".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(")".to_string()),
        ]);
    }

    #[test]
    fn test_complex_comparison() {
        let result = scan_source_code("if x >= 5 && y <= 10");
        assert_eq!(result, vec![
            Lexeme::Text("if".to_string()),
            Lexeme::Text("x".to_string()),
            Lexeme::DoubleSymbol(">=".to_string()),
            Lexeme::Number("5".to_string()),
            Lexeme::DoubleSymbol("&&".to_string()),
            Lexeme::Text("y".to_string()),
            Lexeme::DoubleSymbol("<=".to_string()),
            Lexeme::Number("10".to_string()),
        ]);
    }

    #[test]
    fn test_mathematical_expression() {
        let result = scan_source_code("( a + b ) * c");
        assert_eq!(result, vec![
            Lexeme::Symbol("(".to_string()),
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol("+".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::Symbol(")".to_string()),
            Lexeme::Symbol("*".to_string()),
            Lexeme::Text("c".to_string()),
        ]);
    }

    #[test]
    fn test_string_with_text_and_numbers() {
        let result = scan_source_code("price \"$100\"");
        assert_eq!(result, vec![
            Lexeme::Text("price".to_string()),
            Lexeme::Quote("$100".to_string()),
        ]);
    }

    #[test]
    fn test_assignment_with_decimal() {
        let result = scan_source_code("value = 3.14");
        assert_eq!(result, vec![
            Lexeme::Text("value".to_string()),
            Lexeme::Symbol("=".to_string()),
            Lexeme::NumberWithDot("3.14".to_string()),
        ]);
    }

    #[test]
    fn test_array_initialization() {
        let result = scan_source_code("[1,2,3]");
        assert_eq!(result, vec![
            Lexeme::Symbol("[".to_string()),
            Lexeme::Number("1".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("3".to_string()),
            Lexeme::Symbol("]".to_string()),
        ]);
    }

    #[test]
    fn test_array_with_spaces() {
        let result = scan_source_code("[ 1 , 2 , 3 ]");
        assert_eq!(result, vec![
            Lexeme::Symbol("[".to_string()),
            Lexeme::Number("1".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Number("3".to_string()),
            Lexeme::Symbol("]".to_string()),
        ]);
    }

    #[test]
    fn test_all_operator_types() {
        let result = scan_source_code("+ - * / % ^ ! &");
        assert_eq!(result.len(), 8);
        assert!(result.contains(&Lexeme::Symbol("+".to_string())));
        assert!(result.contains(&Lexeme::Symbol("-".to_string())));
        assert!(result.contains(&Lexeme::Symbol("*".to_string())));
        assert!(result.contains(&Lexeme::Symbol("/".to_string())));
    }

    #[test]
    fn test_not_equals() {
        let result = scan_source_code("!=");
        assert_eq!(result, vec![Lexeme::DoubleSymbol("!=".to_string())]);
    }

    // Edge Cases
    #[test]
    fn test_number_followed_by_text() {
        // Once scanner enters Number state, it can't transition to Text
        // This is expected behavior in the current implementation
        let result = scan_source_code("123abc");
        assert_eq!(result, vec![
            Lexeme::Number("123".to_string()),
            Lexeme::Text("abc".to_string()),
        ]);
    }

    #[test]
    fn test_underscore_only() {
        let result = scan_source_code("_");
        assert_eq!(result, vec![Lexeme::Text("_".to_string())]);
    }

    #[test]
    fn test_multiple_underscores() {
        let result = scan_source_code("___");
        assert_eq!(result, vec![Lexeme::Text("___".to_string())]);
    }

    #[test]
    fn test_text_starts_with_number_like_char() {
        let result = scan_source_code("_123text");
        assert_eq!(result, vec![Lexeme::Text("_123text".to_string())]);
    }

    #[test]
    fn test_dot_after_non_number_becomes_token() {
        // Dot is now recognized as a symbol
        let result = scan_source_code("text.method");
        assert_eq!(result, vec![
            Lexeme::Text("text".to_string()),
            Lexeme::Symbol(".".to_string()),
            Lexeme::Text("method".to_string()),
        ]);
    }

    #[test]
    fn test_semicolon_separator() {
        let result = scan_source_code("a ; b");
        assert_eq!(result, vec![
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("b".to_string()),
        ]);
    }

    #[test]
    fn test_semicolon_separator_no_spaces() {
        let result = scan_source_code("a;b");
        assert_eq!(result, vec![
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("b".to_string()),
        ]);
    }

    #[test]
    fn test_semicolon_end_of_line() {
        let result = scan_source_code("a;\nhello;");
        assert_eq!(result, vec![
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("hello".to_string()),
            Lexeme::Symbol(";".to_string()),
        ]);
    }

    #[test]
    fn test_mixed_operators() {
        let result = scan_source_code("a += 5; b -= 3; c *= 2; d /= 4; e %= 3; f ^= 2;");
        assert_eq!(result, vec![
            Lexeme::Text("a".to_string()),
            Lexeme::DoubleSymbol("+=".to_string()),
            Lexeme::Number("5".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::DoubleSymbol("-=".to_string()),
            Lexeme::Number("3".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("c".to_string()),
            Lexeme::DoubleSymbol("*=".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("d".to_string()),
            Lexeme::DoubleSymbol("/=".to_string()),
            Lexeme::Number("4".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("e".to_string()),
            Lexeme::DoubleSymbol("%=".to_string()),
            Lexeme::Number("3".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Text("f".to_string()),
            Lexeme::DoubleSymbol("^=".to_string()),
            Lexeme::Number("2".to_string()),
            Lexeme::Symbol(";".to_string()),
        ]);
    }

    #[test]
    fn test_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int { return a + b; }");
        assert_eq!(result, vec![
            Lexeme::Text("fn".to_string()),
            Lexeme::Text("add".to_string()),
            Lexeme::Symbol("(".to_string()),
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol(":".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::Symbol(":".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol(")".to_string()),
            Lexeme::DoubleSymbol("->".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol("{".to_string()),
            Lexeme::Text("return".to_string()),
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol("+".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Symbol("}".to_string()),
        ]);
    }

    #[test]
    fn test_reject_function_definition() {
        let result = scan_source_code("fn add(a: int, b: int) -> int? { return a + b; }");
        assert_eq!(result, vec![
            Lexeme::Text("fn".to_string()),
            Lexeme::Text("add".to_string()),
            Lexeme::Symbol("(".to_string()),
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol(":".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol(",".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::Symbol(":".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol(")".to_string()),
            Lexeme::DoubleSymbol("->".to_string()),
            Lexeme::Text("int".to_string()),
            Lexeme::Symbol("?".to_string()),
            Lexeme::Symbol("{".to_string()),
            Lexeme::Text("return".to_string()),
            Lexeme::Text("a".to_string()),
            Lexeme::Symbol("+".to_string()),
            Lexeme::Text("b".to_string()),
            Lexeme::Symbol(";".to_string()),
            Lexeme::Symbol("}".to_string()),
        ]);
    }
}