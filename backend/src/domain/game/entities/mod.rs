//! # Game Entities Module
//!
//! Contains the core entity types for the game domain.
//!
//! - [`game::Game`] - The aggregate root representing a game session
//! - [`player::Player`] - Individual players participating in a game
//! - [`stat::Stat`] - Customizable statistics attached to players

pub mod game;
mod player;
mod stat;
mod tradable;