//! # Application Layer
//!
//! ## Modules
//!
//! * [`common`] - Common utilities and shared logic
//! * [`game`] - Game-related request and command handlers
//! * [`user`] - User authentication and management handlers
//! * [`gm`] - Game Master authentication and operations
//! * [`plugin`] - Game plugin parsing and execution

pub mod common;
pub mod game;
pub mod gm;
pub mod plugin;
pub mod user;
