//! Statistics module for use when tracking unique rates.
//!
//! Very little is exposed from this module aside from the `Stats`
//! struct which contains tracking based on unique counters.

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
#[derive(Default)]
pub struct Stats {
    unique: u64,
    total: u64,
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

    /// Retrieves the total count of unique entries.
    pub fn get_unique(&self) -> u64 {
        self.unique
    }

    /// Retrieves the total count of duplicate entries.
    pub fn get_duplicate(&self) -> u64 {
        self.total - self.unique
    }

    /// Retrieves the total count of input entries.
    pub fn get_total(&self) -> u64 {
        self.total
    }

    /// Retrieves the rate of receiving duplicates.
    pub fn get_rate(&self) -> f32 {
        ((self.unique as f64 / self.total as f64) * 100.0) as f32
    }

    /// Prints all statistics to stdout.
    pub fn print(&self) {
        println!("");
        uprintln("Unique Count", self.get_unique(), 1);
        uprintln("Total Count", self.get_total(), 2);
        uprintln("Dup Offset", self.get_duplicate(), 3);
        println!("Dup Rate:{:>22.2}%", 100.0 - self.get_rate());
        println!("");
    }
}

/// Prints a u64 stats value against a label.
///
/// The label and value are provided alongside an offset used purely
/// for alignment when displayed in a terminal, since we don't want
/// to depend on a table drawing library just for this :).
///
/// This implementation is borrowed from the `separator` crate from
/// [this repo](https://github.com/saghm/rust-separator).
#[inline]
fn uprintln(label: &str, value: u64, offset: usize) {
    let str_value = value.to_string();

    let mut output = String::new();
    let mut place = str_value.len();
    let mut later_loop = false;

    for ch in str_value.chars() {
        if later_loop && place % 3 == 0 {
            output.push(',');
        }

        output.push(ch);
        later_loop = true;
        place -= 1;
    }

    println!("{}:{:>w$}", label, output, w = 18 + offset);
}

#[cfg(test)]
mod tests {
    use super::Stats;

    #[test]
    fn default_creation() {
        let stats = Stats::new();

        assert_eq!(stats.get_total(), 0);
        assert_eq!(stats.get_unique(), 0);
        assert_eq!(stats.get_duplicate(), 0);
    }

    #[test]
    fn addition_of_uniques() {
        let mut stats = Stats::new();

        stats.add_unique();
        stats.add_unique();
        stats.add_unique();

        assert_eq!(stats.get_total(), 3);
        assert_eq!(stats.get_unique(), 3);
        assert_eq!(stats.get_duplicate(), 0);
    }

    #[test]
    fn addition_of_duplicates() {
        let mut stats = Stats::new();

        stats.add_duplicate();
        stats.add_duplicate();
        stats.add_duplicate();

        assert_eq!(stats.get_total(), 3);
        assert_eq!(stats.get_unique(), 0);
        assert_eq!(stats.get_duplicate(), 3);
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

        assert_eq!(stats.get_total(), 6);
        assert_eq!(stats.get_unique(), 3);
        assert_eq!(stats.get_duplicate(), 3);
        assert_eq!(stats.get_rate(), 50.0);
    }
}
