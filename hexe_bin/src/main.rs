#[cfg(feature = "log")]
extern crate env_logger;
extern crate hexe;

use std::env;
use std::ffi::OsString;

/// Converts the input into a String while trying to keep the original buffer.
fn to_string(os_str: OsString) -> String {
    match os_str.into_string() {
        Ok(s) => s,
        Err(s) => s.to_string_lossy().into_owned()
    }
}

fn main() {
    #[cfg(feature = "log")]
    env_logger::init();

    let mut args = env::args_os();
    let mut eng  = hexe::engine::Engine::default();
    let mut uci  = eng.uci();

    match args.len() {
        1 => uci.start(),
        _ => {
            args.next();
            uci.start_with(args.map(to_string));
        },
    }
}
