#[cfg(feature = "log")]
extern crate env_logger;
extern crate clap;

#[macro_use]
extern crate hexe;

use clap::{Arg, App, AppSettings};
use hexe::engine::Engine;

const ABOUT: &str = "
A UCI-compatible chess engine.

Project home page: https://github.com/hexe-rs/Hexe";

static mut NUM_THREADS: usize = 0;

fn main() {
    let mut app = App::new("Hexe")
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .author(authors!())
        .about(ABOUT)
        .set_term_width(80)
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::StrictUtf8,
            AppSettings::VersionlessSubcommands,
        ])
        .arg(Arg::with_name("threads")
            .long("threads")
            .value_name("N")
            .takes_value(true)
            .validator(|val| {
                // Parsing here makes use of clap's
                // built-in error handling
                match val.parse() {
                    Ok(n) => unsafe {
                        NUM_THREADS = n;
                        Ok(())
                    },
                    Err(e) => Err(e.to_string()),
                }
            })
            .empty_values(false)
            .help("The number of OS threads used to run the engine. \
                   If N is 0, the number of \
                   available logical cores is used."));

    // Conditionally include logging flag if feature is enabled
    if cfg!(feature = "log") {
        app = app.arg(Arg::with_name("log")
            .long("log")
            .global(true)
            .value_name("LOG")
            .takes_value(true)
            .env("HEXE_LOG")
            .help("The logging directive."))
    }

    let matches = app.get_matches();

    let mut engine = Engine::builder();

    unsafe {
        // Set by `get_matches`
        engine.num_threads(NUM_THREADS);
    }

    #[cfg(feature = "log")]
    {
        use env_logger::Builder;

        let mut builder = Builder::new();

        if let Some(log_arg) = matches.value_of_lossy("log") {
            builder.parse(&log_arg);
        }

        builder.default_format_module_path(false).init();
    }

    engine.build().uci().start();
}
