//! The Hexe chess engine as a self-contained type.

// TODO lint when everything is implemented
#![allow(unused_variables)]

mod uci;
pub use self::uci::Uci;

/// An instance of the Hexe chess engine.
pub struct Engine {
    options: Options,
}

impl Default for Engine {
    fn default() -> Engine {
        Engine::new(Options::default())
    }
}

impl Engine {
    /// Creates an instance of the engine.
    pub fn new(mut options: Options) -> Engine {
        if options.num_threads == 0 {
            options.num_threads = ::num_cpus::get();
        }
        Engine {
            options: options,
        }
    }

    /// Creates a Universal Chess Interface for this engine.
    #[inline]
    pub fn uci(&mut self) -> Uci {
        Uci::from(self)
    }
}

/// Chess engine options.
pub struct Options {
    num_threads: usize,
}

impl Options {
    /// Set the number of threads to be used by the engine.
    ///
    /// If `num_threads` is 0, or you do not call this function, then the number
    /// of threads will be selected automatically. The default is the number of
    /// logical CPUs.
    #[inline]
    pub fn num_threads(mut self, num_threads: usize) -> Options {
        self.num_threads = num_threads;
        self
    }
}

impl Default for Options {
    fn default() -> Options {
        Options {
            num_threads: 0,
        }
    }
}

impl Options {
    /// Attempts to set the option of `name` to `value`. Returns `false` if
    /// `name` is not an option.
    fn set(&mut self, name: &str, value: &str) -> bool {
        // Performs a case-insensitive check against the option
        let match_option = |opt: &str| {
            if name.len() == opt.len() {
                let a = name.as_bytes().iter();
                let b = opt.as_bytes().iter();
                for (&a, &b) in a.zip(b) {
                    if a | 32 != b {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        };

        if match_option("threads") {
            if let Ok(val) = value.parse() {
                self.num_threads = val;
            }
            true
        } else {
            false
        }
    }
}
