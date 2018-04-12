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

fn main() {
    let mut app = App::new("Hexe")
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .author(authors!())
        .about(ABOUT)
        .set_term_width(80)
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::VersionlessSubcommands,
        ])
        .arg(Arg::with_name("threads")
            .long("threads")
            .takes_value(true)
            .validator(|val| {
                for &byte in val.as_bytes() {
                    if byte < b'0' || byte > b'9' {
                        return Err("found non-digit".into())
                    }
                }
                Ok(())
            })
            .empty_values(false)
            .help("The number of OS threads used to run the engine. \
                   If the value is 0, then the number of \
                   available logical cores is used."));

    // Conditionally include logging flag if feature is enabled
    if cfg!(feature = "log") {
        app = app.arg(Arg::with_name("log")
            .long("log")
            .global(true)
            .takes_value(true)
            .env("HEXE_LOG")
            .help("The logging directive."))
    }

    let matches = app.get_matches();

    let mut engine = Engine::builder();

    if let Some(num_threads) = matches.value_of("threads") {
        engine.num_threads(num_threads.parse().unwrap());
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
