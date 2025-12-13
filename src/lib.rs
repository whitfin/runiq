//! Runiq is a utility to filter unique lines from input.
//!
//! It operates in a much faster pattern than either the Unix `sort`
//! or `uniq` utilities, and without the constraints the two provide
//! (either sorting input or only filtering sequential duplicates).
//!
//! Runiq has a focus on memory space rather than throughput, simply
//! because it comes from a need of filtering large streams of data.
//! Having said this, it should be a goal to perform at least as fast
//! as other tools of the same ilk.
//!
//! Runiq is built mainly as a command line tool, although it can be
//! used as a library as the `Filter` trait is exposed publicly. If
//! you are using Runiq as a library, do **not** rely on any modules
//! hidden from the public documentation.
#![doc(html_root_url = "https://docs.rs/runiq/2.1.0")]
mod filters;
pub use filters::{CompactFilter, Filter, Filters, QuickFilter, SimpleFilter, SortedFilter};
