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
extern crate clap;

// inner mods
mod filters;
mod options;
mod statistics;

// scope requirements
use options::Options;
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};

fn main() {
    // parse in our options from the command line args
    let options = Options::from(&mut env::args_os());

    // ensure all sources exist as readers
    let readers: Vec<Box<Read>> = options
        .get_inputs()
        .into_iter()
        .map(|input| -> Box<Read> {
            match input.as_ref() {
                "-" => Box::new(stdin()),
                any => Box::new(File::open(any).unwrap()),
            }
        })
        .collect();

    // create filter from provided options
    let mut filter = options.new_filter();

    // sequential readers for now
    for reader in readers {
        // iterate every line coming from the reader
        for line in BufReader::new(reader).lines() {
            // unwrap the next input
            let input = line.unwrap();

            // echo back to console
            if filter.insert(&input) {
                if !options.is_inverted() {
                    println!("{}", input);
                }
            } else if options.is_inverted() {
                println!("{}", input);
            }
        }
    }
}
