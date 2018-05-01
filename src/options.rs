//! Options parsing module used to configure application state.
//!
//! Nothing particularly important to see here, just typical
//! parsing of things like command line arguments into something
//! more easily used internally (from the main application flow).
use clap::{App, AppSettings, Arg, ArgSettings};
use filters::{Filter, NaiveFilter};
use std::ffi::OsString;

/// Options struct to store configuration state.
///
/// The options struct will basically contain anything relevant
/// to the execution of the application; things such as inputs
/// and flags to dictate behaviour will be stored here. It acts
/// (in essence) as application configuration.
pub struct Options {
    filter: String,
    inputs: Vec<String>,
    invert: bool,
}

impl Options {
    /// Creates an `Options` struct from an iterable set of arguments.
    ///
    /// Arguments can be any kind of iterator, as long as they can be
    /// successfully cloned and parsed into an instance of `OsString`.
    pub fn from<I, T>(args: I) -> Options
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // create a new parser for our args
        let parser = Options::create_parser();

        // parse out the arguments into matching opts
        let options = parser.get_matches_from(args);

        // create opts
        Options {
            // store the filter to use for unique detection
            filter: options.value_of("filter").unwrap().to_owned(),

            // grab and store inversion flags
            invert: options.is_present("invert"),

            // own all inputs
            inputs: options
                .values_of("inputs")
                .unwrap()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    /// Returns configured inputs.
    pub fn get_inputs(&self) -> &[String] {
        &self.inputs
    }

    /// Returns whether this run is inverted.
    pub fn is_inverted(&self) -> bool {
        self.invert
    }

    /// Creates a new boxed `Filter` baed on the configured filter.
    pub fn new_filter(&self) -> Box<Filter> {
        Box::new(NaiveFilter::new())
    }

    /// Creates a parser used to generate `Options`.
    ///
    /// All command line usage information can be found in the definitions
    /// below, and follows the API of the `clap` library.
    ///
    /// In terms of visibility, this method is defined on the struct due to
    /// the parser being specifically designed around the `Options` struct.
    fn create_parser<'a, 'b>() -> App<'a, 'b> {
        App::new("")
            // package metadata from cargo
            .name(env!("CARGO_PKG_NAME"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .version(env!("CARGO_PKG_VERSION"))

            // arguments and flag details
            .args(&[
                // filter: -f, --filter [naive]
                Arg::with_name("filter")
                    .help("Filter to use to determine uniqueness")
                    .short("f")
                    .long("filter")
                    .default_value("naive")
                    .possible_values(&["naive"])
                    .set(ArgSettings::HideDefaultValue),

                // inputs: +required +multiple
                Arg::with_name("inputs")
                    .help("Input sources to filter")
                    .multiple(true)
                    .required(true),

                // invert: -i --invert
                Arg::with_name("invert")
                    .help("Prints duplicates instead of uniques")
                    .short("i")
                    .long("invert"),
            ])

            // settings required for parsing
            .settings(&[
                AppSettings::ArgRequiredElseHelp,
                AppSettings::TrailingVarArg,
            ])
    }
}
