extern crate hexe;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let mut eng = hexe::engine::Engine::default();
    let mut uci = eng.uci();

    if args.len() == 0 {
        uci.start();
    } else {
        uci.start_with(args);
    }
}
