//! # Statement Module
//!
//! Contains AST node types for each statement kind supported by plugin scripts.

pub mod r#if;

use r#if::IfStatement;

/// A statement in a plugin script.
pub enum Statement {
    If(IfStatement),
}
