//! Module containing set implementations for unique filtering.
//!
//! Although the initial implementation is a simple binding around
//! `std::collections::HashSet`, it is abstracted away from the main
//! flow in case we want to support multiple filtering mechanisms in
//! future (perhaps for different performance enhancements).
mod digest;
mod naive;

// re-exported filters
pub use self::digest::DigestFilter;
pub use self::naive::NaiveFilter;

/// Trait for any type which can be used to filter unique values.
///
/// The filter only supports a single operation of `insert/2` which
/// will provide the ability to check/insert in a single operation.
pub trait Filter {
    /// Create a new instance using defaults.
    fn new() -> Self
    where
        Self: Sized;

    /// Detects a duplicate value.
    ///
    /// Return values are booleans to represent whether the value
    /// was added to the internal filter or not (i.e. `true` if
    /// this is the first time the value has been seen).
    fn detect(&mut self, input: &str) -> bool;
}
