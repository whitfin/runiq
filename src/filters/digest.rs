//! Module containing digest-based filter implementations.
//!
//! The implementations in this module should provide better memory
//! efficiency over the naive implementations provided.
use std::collections::HashSet;
use std::hash::Hasher;
use twox_hash::XxHash;
use super::Filter;

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
    fn detect(&mut self, input: &str) -> bool {
        let mut hasher = XxHash::default();
        hasher.write(input.as_bytes());
        self.inner.insert(hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::{DigestFilter, Filter};

    #[test]
    fn digest_filter_detection() {
        let mut filter = DigestFilter::new();

        let ins1 = filter.detect("input1");
        let ins2 = filter.detect("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }
}
