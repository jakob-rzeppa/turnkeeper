//! # User Application Module
//!
//! Contains request handlers and DTOs for user authentication and management.
//!
//! ## Request Handlers
//!
//! * Login - Authenticate existing users
//! * Register - Create new user accounts
//! * Authenticate - Validate JWT tokens
//!
//! ## Components
//!
//! * [`request_handlers`] - HTTP request handlers
//! * [`requests`] - Request DTOs
//! * [`responses`] - Response DTOs
//! * [`contracts`] - Repository and JWT contracts/traits

pub mod contracts;
pub mod requests;
pub mod responses;
pub mod request_handlers;