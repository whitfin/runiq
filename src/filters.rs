//! Module containing filter implementations for Runiq.
//!
//! Each structure in this module has different filtering properties
//! and should be chosen based on the specific use case of the user.
//!
//! Please see the struct documentation for further information on
//! each filter, including their runtime characteristics.
use fnv::FnvHashSet;
use std::collections::HashSet;
use xxhash2;

/// Enumerable filters for clap-rs.
arg_enum! {
    /// Enum to store all possible variants of filters.
    ///
    /// This will implement the `Into` trait in order to create a new
    /// boxed filter from a filter kind to keep conversion contained.
    #[doc(hidden)]
    #[derive(Copy, Clone, Debug)]
    pub enum FilterKind {
        Consecutive,
        Digest,
        Naive,
    }
}

/// Trait for any type which can be used to filter unique values.
///
/// The filter only supports a single operation of [`Filter::detect`]
/// which will provide the ability to check/insert in a single operation.
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

/// Implement `Into` to convert to `Filter`.
impl Into<Box<Filter>> for FilterKind {
    /// Creates a new `Filter` type based on the enum value.
    fn into(self) -> Box<Filter> {
        match self {
            FilterKind::Consecutive => Box::new(ConsecutiveFilter::new()),
            FilterKind::Digest => Box::new(DigestFilter::new()),
            FilterKind::Naive => Box::new(NaiveFilter::new()),
        }
    }
}

/// Basic filter implementation backed by a `HashSet`.
///
/// This implementation offers nothing more than abstraction over
/// using a `HashSet` directly, and will store raw values in the
/// set. Naturally this means that memory will not be particularly
/// efficient, but it is guaranteed to be completely accurate when
/// calculating unique collisions in inputs.
#[derive(Clone, Debug, Default)]
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
        self.inner.insert(input.into())
    }
}

/// Digest filter implementation backed by a `HashSet`.
///
/// This implementation offers much better memory efficiency when
/// compared to the `NaiveFilter` due to the fact that raw values
/// are hashed to `usize` values before being stored in the set.
///
/// It's also a little faster due to some improved efficiency
/// when comparing values in the set itself, but it's not of any
/// real consequence and is barely noticeable.
#[derive(Clone, Debug, Default)]
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
/// use of this filter to optimize memory usage going forware.
#[derive(Clone, Debug)]
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
    use super::*;

    #[test]
    fn naive_filter_detection() {
        let mut filter = NaiveFilter::new();

        let ins1 = filter.detect("input1");
        let ins2 = filter.detect("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }

    #[test]
    fn digest_filter_detection() {
        let mut filter = DigestFilter::new();

        let ins1 = filter.detect("input1");
        let ins2 = filter.detect("input1");

        assert_eq!(ins1, true);
        assert_eq!(ins2, false);
    }

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
