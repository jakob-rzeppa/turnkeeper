//! # Domain Layer
//!
//! The domain layer contains pure business logic with no external dependencies.
//! It defines the core entities, value objects, and domain rules of the system.
//!
//! ## Modules
//!
//! * [`user`] - User aggregate with authentication logic
//! * [`game`] - Game aggregate with player and stat management
//! * [`gm`] - Game Master domain logic and errors
//!
//! ## Design Principles
//!
//! - **No Infrastructure Dependencies**: Domain code doesn't depend on databases, HTTP, etc.
//! - **Value Objects**: Use value objects to encapsulate validation (e.g., `UserName`, `StatKey`)
//! - **Invariants**: Entity constructors ensure valid states; invalid states are impossible
//! - **UUIDs**: All entities use UUIDs as identifiers for easy ID generation

pub mod user;
pub mod game;
pub mod gm;