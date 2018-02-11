extern crate hexe;

use std::env;

fn main() {
    let mut args = env::args_os();
    let mut eng  = hexe::engine::Engine::default();
    let mut uci  = eng.uci();

    match args.len() {
        1 => uci.start(),
        _ => {
            args.next();
            uci.start_with(args.filter_map(|a| a.into_string().ok()));
        },
    }
}
