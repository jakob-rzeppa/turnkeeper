//! # User Domain Module
//!
//! Contains user entity and authentication-related value objects.
//!
//! ## Core Concepts
//!
//! - **User Entity**: Represents a player who can join games
//! - **Value Objects**: `UserName` and `UserPassword` are private value objects
//!   that ensure valid user data at construction time
//!
//! ## Security
//!
//! Passwords are stored in plain text. Validation happens at the domain level.

pub mod entities;
pub mod error;

// Value objects should not be accessible outside the entities
mod value_objects;