//! The Hexe chess engine as a self-contained type.

// TODO lint when everything is implemented
#![allow(unused_variables)]

use std::usize;

use scoped_threadpool::Pool;

use position::Position;

mod uci;
pub use self::uci::Uci;

/// An instance of the Hexe chess engine.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use hexe::engine::Engine;
///
/// let mut engine = Engine::builder()
///                         .num_threads(4)
///                         .build();
/// # return;
/// engine.uci().start();
/// ```
pub struct Engine {
    pool: Pool,
    engine: EngineInner,
}

struct EngineInner {
    position: Position,
    options: Options,
}

impl Default for Engine {
    #[inline]
    fn default() -> Engine { Engine::new() }
}

impl Engine {
    /// Creates a new builder for setting engine options.
    #[inline]
    pub fn builder() -> EngineBuilder {
        EngineBuilder { num_threads: 0 }
    }

    /// Creates an instance of the engine with the default options applied.
    #[inline]
    pub fn new() -> Engine {
        Engine::builder().build()
    }

    /// Creates a Universal Chess Interface for this engine.
    #[inline]
    pub fn uci(&mut self) -> Uci {
        Uci::from(self)
    }
}

/// A type that can be used to build an [`Engine`](struct.Engine.html) instance.
#[derive(Copy, Clone, Debug)]
pub struct EngineBuilder {
    num_threads: u32,
}

impl EngineBuilder {
    /// Builds a new [`Engine`](struct.Engine.html) with the options of `self`.
    pub fn build(self) -> Engine {
        let mut num_threads = self.num_threads;
        if num_threads == 0 {
            num_threads = ::num_cpus::get() as u32;
        }
        Engine {
            pool: Pool::new(num_threads),
            engine: EngineInner {
                position: Position::default(),
                options: Options { num_threads },
            },
        }
    }

    /// Set the number of threads to be used by the engine.
    ///
    /// If `n` is 0, or you do not call this function, then the number of
    /// threads will be selected automatically. The default is the number of
    /// logical CPUs.
    #[inline]
    pub fn num_threads(mut self, n: u32) -> EngineBuilder {
        self.num_threads = n;
        self
    }
}

/// Chess engine options.
struct Options {
    num_threads: u32,
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
            panic!("Cannot currently set number of threads");
        } else {
            false
        }
    }

    fn report(&self) {
        println!(
            "\noption name Threads type spin default {} min 1 max {}",
            ::num_cpus::get(),
            usize::MAX,
        );
    }
}
