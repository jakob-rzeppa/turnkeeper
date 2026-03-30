#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn matches(&self, line: usize, column: usize) -> bool {
        self.line == line && self.column == column
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
