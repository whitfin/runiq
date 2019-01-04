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
//! Runiq is only built as a command line tool, although it may be
//! distributed as a core crate if the backing implementation becomes
//! interesting for other use cases.
#![doc(html_root_url = "https://docs.rs/runiq/1.1.3")]
pub mod filters;

mod options;
mod statistics;

use bytelines::ByteLinesReader;

use crate::filters::Filter;
use crate::options::Options;
use crate::statistics::Stats;

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    // parse in our options from the command line args
    let options = Options::from(&mut env::args_os());

    // borrow IO for checker
    let stdin = io::stdin();
    let stdout = io::stdout();

    // ensure all sources exist as readers
    let readers: Vec<Box<Read>> = (&options.inputs)
        .into_iter()
        .map(|input| -> Box<Read> {
            match input.as_ref() {
                "-" => Box::new(stdin.lock()),
                any => Box::new(File::open(any).unwrap()),
            }
        })
        .collect();

    // create boxed filter from provided option filter
    let mut filter: Box<Filter> = options.filter.into();

    // create statistics container for filters
    let mut statistics = Stats::new();

    // lock stdout to speed up the writes
    let mut stdout = stdout.lock();

    // eol byte slice
    let eol = &[b'\n'];

    // sequential readers for now
    for reader in readers {
        // construct our line reader to iterate lines of bytes
        let mut lines = BufReader::new(reader).byte_lines();

        // iterate all lines as &[u8] slices
        while let Some(line) = lines.next() {
            // unwrap the input line
            let input = line?;

            // detect duplicate value
            if filter.detect(&input) {
                // handle stats or print
                if options.statistics {
                    // add a unique count
                    statistics.add_unique();
                } else if !options.inverted {
                    // echo if not inverted
                    stdout.write_all(input)?;
                    stdout.write_all(eol)?;
                }
            } else {
                // handle stats or print
                if options.statistics {
                    // add a duplicate count
                    statistics.add_duplicate();
                } else if options.inverted {
                    // echo if we're inverted
                    stdout.write_all(input)?;
                    stdout.write_all(eol)?;
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
