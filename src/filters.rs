//! Module containing filter implementations for Runiq.
//!
//! Each structure in this module has different filtering properties
//! and should be chosen based on the specific use case of the user.
//!
//! Please see the struct documentation for further information on
//! each filter, including their runtime characteristics.
use growable_bloom_filter::{GrowableBloom, GrowableBloomBuilder};
use identity_hash::BuildIdentityHasher;
use strum_macros::EnumString;
use xxhash_rust::xxh3::xxh3_64;

use std::collections::HashSet;

/// Trait for any type which can be used to filter unique values.
///
/// The filter only supports a single operation to detect a unique input
/// which will provide the ability to check/insert in a single operation.
pub trait Filter {
    /// Detects a unique value.
    ///
    /// Return values are booleans to represent whether the value
    /// was added to the internal filter or not (i.e. `true` if
    /// this is the first time the value has been seen).
    fn detect(&mut self, input: &[u8]) -> bool;
}

/// Basic filter implementation backed by a `HashSet`.
///
/// This implementation offers nothing more than abstraction over
/// using a `HashSet` directly, and will store raw values in the
/// set. Naturally this means that memory will not be particularly
/// efficient, but it is guaranteed to be completely accurate when
/// calculating unique collisions in inputs.
#[derive(Clone, Debug, Default)]
pub struct SimpleFilter {
    inner: HashSet<Vec<u8>>,
}

/// Implement all trait methods.
impl Filter for SimpleFilter {
    fn detect(&mut self, input: &[u8]) -> bool {
        self.inner.insert(input.to_vec())
    }
}

/// Digest filter implementation backed by a `HashSet`.
///
/// This implementation offers much better memory efficiency when
/// compared to the `SimpleFilter` due to the fact that raw values
/// are hashed to `usize` values before being stored in the set.
///
/// It's also a little faster due to some improved efficiency
/// when comparing values in the set itself, but it's not of any
/// real consequence and is barely noticeable.
#[derive(Clone, Debug, Default)]
pub struct QuickFilter {
    inner: HashSet<u64, BuildIdentityHasher<u64>>,
}

/// Implement all trait methods.
impl Filter for QuickFilter {
    fn detect(&mut self, input: &[u8]) -> bool {
        self.inner.insert(xxh3_64(input))
    }
}

/// Uniq filter implementation to only remove consecutive duplicates.
///
/// This is the fastest filter (although not by much), and the best in
/// terms of memory efficiency as it only requires a single value stored
/// in the filter memory at once. It operates in the same was as the Unix
/// `uniq` utility, and thus requires your data be sorted prior to any
/// execution.
///
/// Remember that repeatedly running Runiq on the same input would be
/// a good candidate for sorting your data initially and then making
/// use of this filter to optimize memory usage going forward.
#[derive(Clone, Debug, Default)]
pub struct SortedFilter {
    inner: Vec<u8>,
}

/// Implement all trait methods.
impl Filter for SortedFilter {
    fn detect(&mut self, input: &[u8]) -> bool {
        // check for consec collision
        if input == &self.inner[..] {
            return false;
        }

        // overwrite the previous value
        self.inner = input.to_vec();
        true
    }
}

/// Bitset filter backed by a scalable Bloom Filter.
///
/// This filter operates with the least amount of memory, with a cost
/// of speed (roughly 60-70% of the speed of the `QuickFilter`, using
/// only 25% of the memory).
///
/// The backing bloom filter initializes with `1e6` bits by default, with
/// `1e-7` probability of collisions. This is roughly comparable to the
/// collision rate of the digest filter, so this should be chosen when
/// memory is critical.
#[derive(Debug)]
pub struct CompactFilter {
    inner: GrowableBloom,
}

impl Default for CompactFilter {
    fn default() -> Self {
        Self {
            inner: GrowableBloomBuilder::new()
                .estimated_insertions(1_000_000)
                .desired_error_ratio(1e-8)
                .growth_factor(2)
                .tightening_ratio(0.5)
                .build(),
        }
    }
}

/// Implement all trait methods.
impl Filter for CompactFilter {
    fn detect(&mut self, input: &[u8]) -> bool {
        self.inner.insert(xxh3_64(input))
    }
}

/// Enum to store all possible variants of filters.
///
/// This will implement the `Into` trait in order to create a new
/// boxed filter from a filter kind to keep conversion contained.
#[derive(Copy, Clone, Debug, EnumString)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Filters {
    /// Hashed comparisons with more efficient throughput.
    Quick,

    /// Naive comparisons of strings within a set.
    Simple,

    /// Adjacent comparisons on sorted data (similar to uniq).
    Sorted,

    /// Bloom filter comparisons with compact memory usage.
    Compact,
}

/// Implement `From` to convert to `Filter`.
impl From<Filters> for Box<dyn Filter> {
    /// Creates a new `Filter` type based on the enum value.
    fn from(kind: Filters) -> Self {
        match kind {
            Filters::Quick => Box::<QuickFilter>::default(),
            Filters::Simple => Box::<SimpleFilter>::default(),
            Filters::Compact => Box::<CompactFilter>::default(),
            Filters::Sorted => Box::<SortedFilter>::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_filter_detection() {
        let mut filter = SimpleFilter::default();

        let ins1 = filter.detect(b"input1");
        let ins2 = filter.detect(b"input1");

        assert!(ins1);
        assert!(!ins2);
    }

    #[test]
    fn digest_filter_detection() {
        let mut filter = QuickFilter::default();

        let ins1 = filter.detect(b"input1");
        let ins2 = filter.detect(b"input1");

        assert!(ins1);
        assert!(!ins2);
    }

    #[test]
    fn sorted_filter_detection() {
        let mut filter = SortedFilter::default();

        let ins1 = filter.detect(b"input1");
        let ins2 = filter.detect(b"input1");
        let ins3 = filter.detect(b"input2");
        let ins4 = filter.detect(b"input1");

        assert!(ins1);
        assert!(!ins2);
        assert!(ins3);
        assert!(ins4);
    }

    #[test]
    fn bloom_filter_detection() {
        let mut filter = CompactFilter::default();

        let ins1 = filter.detect(b"input1");
        let ins2 = filter.detect(b"input1");

        assert!(ins1);
        assert!(!ins2);
    }
}
