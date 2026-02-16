//! # Application Layer
//!
//! The application layer orchestrates business logic through Request and Event Handlers.
//!
//! ## Architecture
//!
//! ### Request Handlers (Stateless)
//! Handle HTTP requests with a request/response pattern:
//! - Accept a Request DTO
//! - Execute business logic via domain entities
//! - Return a Response DTO
//! 
//! The Request Handlers should not be able to edit / delete a game while running.
//!
//! ### Event Handlers (Stateful)
//! Handle WebSocket events for active game sessions:
//! - Accept events
//! - Log event to AOF (Append-Only-File) like db
//! - Mutate game state
//! - Emit events to connected clients
//!
//! ## Modules
//!
//! * [`game`] - Game-related request and event handlers
//! * [`user`] - User authentication and management handlers
//! * [`gm`] - Game Master authentication and operations

pub mod game;
pub mod user;
pub mod gm;