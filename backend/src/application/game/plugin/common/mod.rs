
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    first_char: usize,
}

impl Position {
    pub fn new(line: usize, first_char: usize) -> Self {
        Position { line, first_char }
    }
}