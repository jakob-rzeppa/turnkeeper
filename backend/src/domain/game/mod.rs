//! # Game Domain Module
//!
//! Contains the game aggregate with entities, value objects, and domain commands.
//!
//! ## Core Concepts
//!
//! - **Game Aggregate**: The [`entities::game::Game`] is the aggregate root
//! - **Players**: Each game contains multiple [`entities::player::Player`] entities
//! - **Stats**: Players have customizable statistics via [`entities::stat::Stat`]
//! - **Value Objects**: Type-safe wrappers for stat keys and values
//!
//! ## Invariants that can't be checked by the aggregate
//!
//! - No duplicate game names

pub mod entities;
pub mod value_objects;
pub mod error;
pub mod projections;
pub mod commands;