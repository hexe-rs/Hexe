#[cfg(feature = "log")]
extern crate env_logger;
extern crate clap;

#[macro_use]
extern crate hexe;

use std::str::FromStr;

use clap::{Arg, App, AppSettings};
use hexe::engine::Engine;

const ABOUT: &str = "
A UCI-compatible chess engine.

Project homepage: https://github.com/hexe-rs/Hexe
Library docs:     https://docs.rs/hexe";

static mut NUM_THREADS: Option<usize> = None;
static mut HASH_SIZE:   Option<usize> = None;

/// Parses `val` and stores it in `dst`.
fn parse<T>(val: String, dst: &mut Option<T>) -> Result<(), String>
    where T: FromStr,
          T::Err: ToString,
{
    match val.parse::<T>() {
        Ok(val) => {
            *dst = Some(val);
            Ok(())
        },
        Err(err) => Err(err.to_string())
    }
}

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
        .arg(Arg::with_name("hash size")
            .long("hash")
            .short("H")
            .value_name("SIZE")
            .takes_value(true)
            .validator(|val| parse(val, unsafe { &mut HASH_SIZE }))
            .help("The hash table size in megabytes"))
        .arg(Arg::with_name("threads")
            .long("threads")
            .value_name("N")
            .takes_value(true)
            .validator(|val| parse(val, unsafe { &mut NUM_THREADS }))
            .empty_values(false)
            .help("The number of OS threads used to run the engine; \
                   if not provided or N is 0, all available logical \
                   cores are used"));

    // Conditionally include logging flag if feature is enabled
    if cfg!(feature = "log") {
        app = app
            .arg(Arg::with_name("log")
                .long("log")
                .short("l")
                .global(true)
                .takes_value(true)
                .value_name("LOG")
                .env("HEXE_LOG")
                .help("The logging directive"))
            .arg(Arg::with_name("color")
                .long("color")
                .short("C")
                .global(true)
                .takes_value(true)
                .value_name("WHEN")
                .possible_values(&["auto", "always", "never"])
                .help("When to color logging output"))
    }

    // Matches unused when "log" is disabled
    #[allow(unused_variables)]
    let matches = app.get_matches();

    let mut engine = Engine::builder();

    // Set by `get_matches`
    unsafe {
        if let Some(n) = NUM_THREADS {
            engine.num_threads(n);
        }
        if let Some(n) = HASH_SIZE {
            engine.hash_size(n);
        }
    }

    #[cfg(feature = "log")]
    {
        use env_logger::Builder;

        let mut builder = Builder::new();

        if let Some(style) = matches.value_of("color") {
            builder.parse_write_style(style);
        }

        if let Some(log_arg) = matches.value_of("log") {
            builder.parse(log_arg);
        }

        builder.default_format_module_path(false).init();
    }

    engine.build().uci().start();
}
