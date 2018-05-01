//! Module containing naive filter implementations.
//!
//! The implementations in this module will be very inefficient
//! on memory as they're backed by almost no optimizations.
use std::collections::HashSet;
use super::Filter;

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
    fn detect(&mut self, input: &str) -> bool {
        self.inner.insert(input.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::{Filter, NaiveFilter};

    #[test]
    fn naive_filter_detection() {
        let mut filter = NaiveFilter::new();

        let ins1 = filter.detect("input1");
        let ins2 = filter.detect("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }
}
