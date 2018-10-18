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
#![doc(html_root_url = "https://docs.rs/runiq/1.0.0")]

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
use std::io::{self, stdin, BufRead, BufReader, Read, Write};

fn main() -> io::Result<()> {
    // parse in our options from the command line args
    let options = Options::from(&mut env::args_os());

    let sin = stdin();

    // ensure all sources exist as readers
    let readers: Vec<Box<Read>> = (&options.inputs)
        .into_iter()
        .map(|input| -> Box<Read> {
            match input.as_ref() {
                "-" => Box::new(sin.lock()),
                any => Box::new(File::open(any).expect(&format!("Couldn't open {}", input))),
            }
        })
        .collect();

    // create boxed filter from provided option filter
    let mut filter: Box<Filter> = options.filter.into();

    // create statistics container for filters
    let mut statistics = Stats::new();

    let mut buf = Vec::with_capacity(100);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // sequential readers for now
    for reader in readers {
        // iterate every line coming from the reader
        let mut reader = BufReader::new(reader);
        loop {
            buf.clear();
            let bits = match reader.read_until(b'\n', &mut buf)? {
                0 => break,
                len => {
                    if buf[len - 1] == b'\n' {
                        if len == 1 {
                            &buf[..0]
                        } else {
                            &buf[..len - 2]
                        }
                    } else {
                        buf.push(b'\n');
                        &buf[..len - 1]
                    }
                }
            };

            // detect duplicate value
            if filter.detect(&bits) {
                // handle stats or print
                if options.statistics {
                    // add a unique count
                    statistics.add_unique();
                }
                if !options.inverted {
                    // echo if not inverted
                    stdout.write_all(&buf)?;
                }

            // handle stats or print
            } else {
                if options.statistics {
                    // add a duplicate count
                    statistics.add_duplicate();
                }
                if options.inverted {
                    // echo if we're inverted
                    stdout.write_all(&buf)?;
                }
            }
        }
    }

    // handle stats logging
    if options.statistics {
        statistics.print();
    }

    Ok(())
}
