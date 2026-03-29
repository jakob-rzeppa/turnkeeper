//! # Abstract Syntax Tree Module
//!
//! Defines the node types that make up the abstract syntax tree for
//! plugin scripts.

pub mod statement;

/// An expression that can appear in a plugin script.
pub enum Expression {
    /// A boolean literal (`true` or `false`).
    Bool(bool),
    /// A numeric literal.
    Number(f64),
    /// A string literal.
    Str(String),
}
