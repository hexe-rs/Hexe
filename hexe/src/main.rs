extern crate hexe;

use std::env;

/// A string that defaults to `str::default()` if `None`.
struct FallbackStr<T>(Option<T>);

impl<T: AsRef<str>> AsRef<str> for FallbackStr<T> {
    fn as_ref(&self) -> &str {
        if let Some(ref s) = self.0 {
            s.as_ref()
        } else {
            Default::default()
        }
    }
}

fn main() {
    let mut args = env::args_os();
    let mut eng  = hexe::engine::Engine::default();
    let mut uci  = eng.uci();

    match args.len() {
        1 => uci.start(),
        _ => {
            args.next();
            uci.start_with(args.map(|a| FallbackStr(a.into_string().ok())));
        },
    }
}
