//! Statistics module for use when tracking unique rates.
//!
//! Very little is exposed from this module aside from the `Stats`
//! struct which contains tracking based on unique counters.

use bytesize::ByteSize;
use cli_table::format::{Border, Justify, Separator};
use cli_table::{print_stdout, Cell, Row, RowStruct, Table};
use format_num::NumberFormat;

/// Statistics struct to store metrics.
///
/// Currently only provides the following:
///
/// - Total number of input entries
/// - Total number of unique entries
/// - Total number of duplicate entries
/// - Rate (as a %) of duplicate entries
///
/// More might be added in future, but for now these are the only
/// metrics surfaced on the `Stats` API.
#[derive(Debug, Default)]
pub struct Stats {
    unique: u64,
    total: u64,
    size: u64,
}

impl Stats {
    /// Creates a new `Stats` container using default values.
    pub fn new() -> Stats {
        Stats::default()
    }

    /// Adds a unique entry to the stats count.
    #[inline]
    pub fn add_unique(&mut self) {
        self.total += 1;
        self.unique += 1;
    }

    /// Adds a duplicate entry to the stats count.
    #[inline]
    pub fn add_duplicate(&mut self) {
        self.total += 1;
    }

    /// Adds a size entry to the stats count.
    #[inline]
    pub fn add_size(&mut self, size: usize) {
        self.size += size as u64
    }

    /// Retrieves the total count of duplicate entries.
    pub fn duplicates(&self) -> u64 {
        self.total - self.unique
    }

    /// Retrieves the rate of receiving duplicates.
    pub fn rate(&self) -> f32 {
        ((self.unique as f64 / self.total as f64) * 100.0) as f32
    }

    /// Retrieves the total size of input entries.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Retrieves the total count of input entries.
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Retrieves the total count of unique entries.
    pub fn uniques(&self) -> u64 {
        self.unique
    }

    /// Prints all statistics to stdout.
    pub fn print(&self) {
        let num = NumberFormat::new();
        let table = vec![
            create_row(
                &num,
                "File Size:",
                self.size() as f64,
                ",.0",
                &format!("\x08(~{})", ByteSize::b(self.size()).display().si()),
            ),
            create_row(&num, "Total Count:", self.total() as f64, ",.0", ""),
            create_row(&num, "Unique Count:", self.uniques() as f64, ",.0", ""),
            create_row(&num, "Dup Offset:", self.duplicates() as f64, ",.0", ""),
            create_row(
                &num,
                "Dup Rate:",
                ((100.0 - self.rate()) / 100.0) as f64,
                ",.2%",
                "",
            ),
        ]
        .table()
        .border(Border::builder().build())
        .separator(Separator::builder().build());

        print_stdout(table).expect("unable to print stats table")
    }
}

/// Constructs a table row using a label and value.
fn create_row(num: &NumberFormat, label: &str, value: f64, fmt: &str, ext: &str) -> RowStruct {
    vec![
        format!("\x08{}", label).cell(),
        num.format(fmt, value).cell().justify(Justify::Right),
        ext.cell(),
    ]
    .row()
}

#[cfg(test)]
mod tests {
    use super::Stats;

    #[test]
    fn default_creation() {
        let stats = Stats::new();

        assert_eq!(stats.total(), 0);
        assert_eq!(stats.uniques(), 0);
        assert_eq!(stats.duplicates(), 0);
    }

    #[test]
    fn addition_of_uniques() {
        let mut stats = Stats::new();

        stats.add_unique();
        stats.add_unique();
        stats.add_unique();

        assert_eq!(stats.total(), 3);
        assert_eq!(stats.uniques(), 3);
        assert_eq!(stats.duplicates(), 0);
    }

    #[test]
    fn addition_of_duplicates() {
        let mut stats = Stats::new();

        stats.add_duplicate();
        stats.add_duplicate();
        stats.add_duplicate();

        assert_eq!(stats.total(), 3);
        assert_eq!(stats.uniques(), 0);
        assert_eq!(stats.duplicates(), 3);
    }

    #[test]
    fn generate_of_rates() {
        let mut stats = Stats::new();

        stats.add_duplicate();
        stats.add_duplicate();
        stats.add_duplicate();
        stats.add_unique();
        stats.add_unique();
        stats.add_unique();

        assert_eq!(stats.total(), 6);
        assert_eq!(stats.uniques(), 3);
        assert_eq!(stats.duplicates(), 3);
        assert_eq!(stats.rate() as u16, 50);
    }
}
