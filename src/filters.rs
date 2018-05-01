//! Module containing set implementations for unique filtering.
//!
//! Although the initial implementation is a simple binding around
//! `std::collections::HashSet`, it is abstracted away from the main
//! flow in case we want to support multiple filtering mechanisms in
//! future (perhaps for different performance enhancements).
use fxhash::FxHasher64;
use std::collections::HashSet;
use std::hash::Hasher;

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
    fn detect(&mut self, input: &String) -> bool;
}

/// Basic filter backed by a HashSet.
///
/// This offers nothing more than an abstraction layer over using
/// a `HashSet` directly, and so will not offer optimal memory.
#[derive(Default)]
pub struct NaiveFilter {
    inner: HashSet<String>,
}

/// Implement all trait methods.
impl Filter for NaiveFilter {
    /// Creates a new `NaiveFilter`.
    fn new() -> NaiveFilter {
        NaiveFilter::default()
    }

    /// Detects a duplicate value.
    #[inline]
    fn detect(&mut self, input: &String) -> bool {
        self.inner.insert(input.clone())
    }
}

/// Digest filter backed by a HashSet.
///
/// This offers better memory usage over the `NaiveFilter` as it
/// will hash values to u64 before storing in the set. It's also
/// a little faster, but not particularly noticeable.
#[derive(Default)]
pub struct DigestFilter {
    inner: HashSet<u64>,
}

/// Implement all trait methods.
impl Filter for DigestFilter {
    /// Creates a new `DigestFilter`.
    fn new() -> DigestFilter {
        DigestFilter::default()
    }

    /// Detects a duplicate value.
    #[inline]
    fn detect(&mut self, input: &String) -> bool {
        let mut hasher = FxHasher64::default();
        hasher.write(input.as_bytes());
        self.inner.insert(hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::{DigestFilter, NaiveFilter};

    #[test]
    fn naive_filter_detection() {
        let mut filter = NaiveFilter::new();

        let ins1 = filter.insert("input1");
        let ins2 = filter.insert("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }

    #[test]
    fn digest_filter_detection() {
        let mut filter = DigestFilter::new();

        let ins1 = filter.insert("input1");
        let ins2 = filter.insert("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }
}
