//! Module containing digest-based filter implementations.
//!
//! The implementations in this module should provide better memory
//! efficiency over the naive implementations provided.
use fnv::FnvHashSet;
use super::Filter;
use xxhash2;

/// Digest filter backed by a HashSet.
///
/// This offers better memory usage over the `NaiveFilter` as it
/// will hash values to u64 before storing in the set. It's also
/// a little faster, but not particularly noticeable.
#[derive(Default)]
pub struct DigestFilter {
    inner: FnvHashSet<usize>,
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
        // grab the bytes from the input
        let bytes = input.as_bytes();

        // hash based on whether we're u32 or u64, for efficiency
        let digest: usize = if cfg!(target_pointer_width = "64") {
            xxhash2::hash64(bytes, 0) as usize
        } else {
            xxhash2::hash32(bytes, 0) as usize
        };

        // insert the new digest
        self.inner.insert(digest)
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
