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
#![doc(html_root_url = "https://docs.rs/runiq/1.1.0")]

// crate imports
#[macro_use]
extern crate clap;
extern crate fnv;
extern crate scalable_bloom_filter;
extern crate xxhash2;

// documented mods
pub mod filters;

// priv mods
mod options;
mod statistics;

// scope requirements
use filters::Filter;
use options::Options;
use statistics::Stats;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

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
        }).collect();

    // create boxed filter from provided option filter
    let mut filter: Box<Filter> = options.filter.into();

    // create statistics container for filters
    let mut statistics = Stats::new();

    // lock stdout to speed up the writes
    let mut stdout = stdout.lock();

    // alloc vector with arbitrary start capacity
    let mut buf = Vec::with_capacity(128);

    // sequential readers for now
    for reader in readers {
        // wrap the reader in a BufReader
        let mut reader = BufReader::new(reader);

        loop {
            // reset bytes
            buf.clear();

            // iterate every line coming from the reader (but as bytes)
            let input = match reader.read_until(b'\n', &mut buf) {
                // short circuit on error
                Err(e) => return Err(e),
                // no input, done
                Ok(0) => break,
                // bytes!
                Ok(n) => {
                    // always use full buffer
                    let mut trim = &buf[..];

                    // always "pop" the delim
                    if trim[n - 1] == b'\n' {
                        trim = &trim[..n - 1];
                    }

                    // also "pop" a leading \r
                    if trim[n - 1] == b'\r' {
                        trim = &trim[..n - 1];
                    }

                    trim
                }
            };

            // detect duplicate value
            if filter.detect(&input) {
                // handle stats or print
                if options.statistics {
                    // add a unique count
                    statistics.add_unique();
                } else if !options.inverted {
                    // echo if not inverted
                    stdout.write_all(input)?;
                }
            } else {
                // handle stats or print
                if options.statistics {
                    // add a duplicate count
                    statistics.add_duplicate();
                } else if options.inverted {
                    // echo if we're inverted
                    stdout.write_all(input)?;
                }
            }
        }
    }

    // handle stats logging
    if options.statistics {
        statistics.print();
    }

    // done
    Ok(())
}
