//! The Hexe chess engine as a self-contained type.

mod uci;

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
    pub fn new(options: Options) -> Engine {
        Engine {
            options: options,
        }
    }
}

/// Chess engine options.
pub struct Options {
}

impl Default for Options {
    fn default() -> Options {
        Options {}
    }
}
