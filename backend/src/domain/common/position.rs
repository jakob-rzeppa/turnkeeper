use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

    pub fn parse_str(pos_str: &str) -> Option<Self> {
        let parts: Vec<&str> = pos_str
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        if parts.len() != 2 {
            return None;
        }
        let line = parts[0].trim().parse::<usize>().ok()?;
        let column = parts[1].trim().parse::<usize>().ok()?;
        Some(Position { line, column })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Position::parse_str(&s).ok_or_else(|| serde::de::Error::custom("Invalid position format"))
    }
}

pub trait Positioned {
    fn position(&self) -> Position;
}
