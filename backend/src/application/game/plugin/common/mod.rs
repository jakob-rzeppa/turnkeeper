#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    first_char: usize,
}

impl Position {
    pub fn new(line: usize, first_char: usize) -> Self {
        Position { line, first_char }
    }

    pub fn matches(&self, line: usize, first_char: usize) -> bool {
        self.line == line && self.first_char == first_char
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.line, self.first_char)
    }
}
