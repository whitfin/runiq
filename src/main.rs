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
use bytelines::ByteLinesReader;

mod options;
mod statistics;

use crate::options::Options;
use crate::statistics::Stats;
use runiq::Filter;

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

const EOL: &[u8; 1] = &[b'\n'];

fn main() -> io::Result<()> {
    // parse in our options from the command line args
    let options = Options::from(env::args_os());

    // borrow IO for checker
    let stdin = io::stdin();
    let stdout = io::stdout();

    // ensure all sources exist as readers
    let readers: Vec<Box<dyn Read>> = options
        .inputs
        .iter()
        .map(|input| -> Box<dyn Read> {
            match input.as_ref() {
                "-" => Box::new(stdin.lock()),
                any => Box::new(File::open(any).unwrap()),
            }
        })
        .collect();

    // create boxed filter from provided option filter
    let mut filter: Box<dyn Filter> = options.filter.into();

    // create statistics container for filters
    let mut statistics = Stats::new();

    // lock stdout to speed up the writes
    let mut stdout = stdout.lock();

    // sequential readers for now
    for reader in readers {
        // construct our line reader to iterate lines of bytes
        let mut lines = BufReader::new(reader).byte_lines();

        // iterate all lines as &[u8] slices
        while let Some(line) = lines.next() {
            // unwrap the input line
            let input = line?;

            // track input sizing
            if options.statistics {
                statistics.add_size(input.len() + 1)
            }

            // detect duplicate value
            if filter.detect(input) {
                // handle stats or print
                if options.statistics {
                    // add a unique count
                    statistics.add_unique();
                } else if !options.inverted {
                    // echo if not inverted
                    stdout.write_all(input)?;
                    stdout.write_all(EOL)?;
                }
            } else {
                // handle stats or print
                if options.statistics {
                    // add a duplicate count
                    statistics.add_duplicate();
                } else if options.inverted {
                    // echo if we're inverted
                    stdout.write_all(input)?;
                    stdout.write_all(EOL)?;
                }
            }
        }
    }

    // handle stats logging
    if options.statistics {
        statistics.print();
    }

    // flush buffers
    stdout.flush()?;

    // done
    Ok(())
}
