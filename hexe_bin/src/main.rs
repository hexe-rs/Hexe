#[cfg(feature = "log")]
extern crate env_logger;
extern crate clap;

#[macro_use]
extern crate hexe;

use clap::{Arg, App};
use hexe::engine::Engine;

fn main() {
    let mut app = App::new("Hexe")
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .author(authors!())
        .about("A chess engine")
        .set_term_width(80)
        .arg(Arg::with_name("threads")
            .long("threads")
            .takes_value(true)
            .help("The number of OS threads used to run the engine. \
                   If the value is 0, then the number of \
                   available logical cores is used."));

    // Conditionally include logging flag if feature is enabled
    if cfg!(feature = "log") {
        app = app.arg(Arg::with_name("log")
            .long("log")
            .takes_value(true)
            .help("The logging directive"))
    }

    let matches = app.get_matches();

    let mut engine = Engine::builder();

    if let Some(num_threads) = matches.value_of("threads") {
        match num_threads.parse() {
            Ok(n)  => {
                engine.num_threads(n);
            },
            Err(_) => eprintln!("Invalid digit found in \'--threads\'"),
        }
    }

    #[cfg(feature = "log")]
    {
        use env_logger::Builder;

        let mut builder = Builder::from_env("HEXE_LOG");

        if let Some(log_arg) = matches.value_of_os("log") {
            if let Some(s) = log_arg.to_str() {
                builder.parse(s);
            } else {
                eprintln!("Invalid UTF-8 string found in \'--log\'")
            }
        }

        builder.default_format_module_path(false).init();
    }

    engine.build().uci().start();
}
