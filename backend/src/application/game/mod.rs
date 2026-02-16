//! # Game Application Module
//!
//! Contains request handlers, event handlers, and DTOs for game operations.
//!
//! ## Components
//!
//! * [`request_handlers`] - HTTP request handlers for game CRUD operations
//! * [`event_handlers`] - WebSocket event handlers for active game sessions
//! * [`dto`] - Data Transfer Objects for external communication
//! * [`requests`] - Request/Response types for handlers
//! * [`contracts`] - Repository contracts/traits

pub mod dto;
pub mod request_handlers;
pub mod requests;
pub mod contracts;
pub mod event_handlers;