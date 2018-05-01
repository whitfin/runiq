//! Options parsing module used to configure application state.
//!
//! Nothing particularly important to see here, just typical
//! parsing of things like command line arguments into something
//! more easily used internally (from the main application flow).
use std::ffi::OsString;

/// Options struct to store configuration state.
///
/// The options struct will basically contain anything relevant
/// to the execution of the application; things such as inputs
/// and flags to dictate behaviour will be stored here. It acts
/// (in essence) as application configuration.
pub struct Options {
    pub inputs: Vec<String>,
    pub invert: bool,
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
        // create a parser for our args
        let parser = clap_app!(app =>

            // package metadata from cargo
            (name: env!("CARGO_PKG_NAME"))
            (about: env!("CARGO_PKG_DESCRIPTION"))
            (version: env!("CARGO_PKG_VERSION"))

            // argument details for the flags and arguments provided
            (@arg inputs: +required +multiple "Sets the input sources to use")
            (@arg invert: -i --invert "Prints duplicates instead of uniques")

            // settings required for parsing
            (@setting ArgRequiredElseHelp)
            (@setting TrailingVarArg)
        );

        // parse out the arguments into matching opts
        let options = parser.get_matches_from(args);

        // create opts
        Options {
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
}
