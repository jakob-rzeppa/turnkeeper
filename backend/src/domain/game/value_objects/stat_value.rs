//! # Stat Value Value Objects
//!
//! Provides type-safe wrappers for different stat value types.
//!
//! Supports three types of stat values:
//! - [`BooleanStatValue`] - Boolean flags (e.g., "is_dead")
//! - [`NumberStatValue`] - Numeric values (e.g., health points)
//! - [`StringStatValue`] - Text values (e.g., character class)

/// A boolean stat value.
///
/// # Examples
///
/// ```rust,ignore
/// let is_active = BooleanStatValue::new(true);
/// assert_eq!(is_active.value(), true);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanStatValue {
    value: bool,
}

impl BooleanStatValue {
    /// Creates a new boolean stat value.
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    /// Returns the boolean value.
    pub fn value(&self) -> bool {
        self.value
    }
}

/// A numeric stat value (signed 64-bit integer).
///
/// # Examples
///
/// ```rust,ignore
/// let health = NumberStatValue::new(100);
/// assert_eq!(health.value(), 100);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NumberStatValue {
    value: i64,
}

impl NumberStatValue {
    /// Creates a new numeric stat value.
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    /// Returns the numeric value.
    pub fn value(&self) -> i64 {
        self.value
    }
}

/// A string stat value.
///
/// # Examples
///
/// ```rust,ignore
/// let class = StringStatValue::new("Warrior".to_string());
/// assert_eq!(class.value(), "Warrior");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StringStatValue {
    value: String,
}

impl StringStatValue {
    /// Creates a new string stat value.
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Returns the string value as a string slice.
    pub fn value(&self) -> &str {
        &self.value
    }
}