extern crate hexe;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let mut eng = hexe::engine::Engine::default();
    let mut uci = eng.uci();

    match args.len() {
        0 => uci.start(),
        _ => uci.start_with(args),
    }
}
