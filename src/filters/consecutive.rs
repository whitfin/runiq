//! Module containing implementations for consecutive filters.
//!
//! This filter is meant to mimic the Unix tool `uniq`, in that it will
//! only detect sequential duplicates. This means that unless your data
//! will guarantee duplicates are next to each other, you'll usually
//! have to sort the data in advance (which is not handled by `runiq`).
//!
//! Naturally, this filter should only be used on a single thread as the
//! order of the data is critical and cannot be mutated due to passing
//! data across threads.
//!
//! When benchmarked, this filter operates at roughly twice the speed of
//! Unix `uniq` and half the memory (although the memory is so low that
//! it's not of any real concern here).
use super::Filter;

/// Consecutive filter containing only the previous value.
///
/// This is the fastest filter, and best for memory usage, but requires
/// that your data be sorted prior to execution. Although this sounds
/// like a bad thing, remember that running multiple passes on the same
/// data would benefit from sorting in advance to allow cheap repetition.
pub struct ConsecutiveFilter {
    inner: String,
}

/// Implement all trait methods.
impl Filter for ConsecutiveFilter {
    /// Creates a new `ConsecutiveFilter`.
    fn new() -> ConsecutiveFilter {
        ConsecutiveFilter {
            inner: "rcf_default".to_owned(),
        }
    }

    /// Detects a duplicate value.
    #[inline]
    fn detect(&mut self, input: &str) -> bool {
        // check for consec collision
        if input == self.inner {
            return false;
        }

        // overwrite the previous value
        self.inner = input.to_owned();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsecutiveFilter, Filter};

    #[test]
    fn consecutive_filter_detection() {
        let mut filter = ConsecutiveFilter::new();

        let ins1 = filter.detect("input1");
        let ins2 = filter.detect("input1");
        let ins3 = filter.detect("input2");
        let ins4 = filter.detect("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
        assert_eq!(ins3, true);
        assert_eq!(ins4, true);
    }
}
