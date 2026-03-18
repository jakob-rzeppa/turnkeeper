//! # Persistence Module
//!
//! Handles database connections and data persistence.
//!
//! ## Components
//!
//! * [`db`] - Database connection pool creation and management
//! * [`repositories`] - Repository implementations for data access
//!
//! ## Database
//!
//! Uses SQLite with SQLx for async database operations.

pub mod db;
pub mod repositories;