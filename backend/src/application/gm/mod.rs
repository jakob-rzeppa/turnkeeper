//! # Game Master Application Module
//!
//! Contains request handlers for Game Master operations.
//!
//! GMs have elevated privileges to manage games and users.
//!
//! ## Components
//!
//! * [`request_handlers`] - HTTP request handlers for GM operations
//! * [`requests`] - Request DTOs
//! * [`responses`] - Response DTOs
//! * [`contracts`] - JWT contracts/traits for GM authentication

pub mod requests;
pub mod responses;
pub mod contracts;
pub mod request_handlers;