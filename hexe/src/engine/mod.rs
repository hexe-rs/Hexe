//! The Hexe chess engine as a self-contained type.

// TODO lint when everything is implemented
#![allow(unused_variables)]

use std::usize;

use position::Position;

use table::Table;
use thread::Pool;

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
    table: Table,
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
        EngineBuilder {
            num_threads: 0,
            hash_mb: 0,
        }
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

    /// Returns the number of threads that the engine currently has spawned.
    #[inline]
    pub fn num_threads(&self) -> usize {
        self.pool.num_threads()
    }
}

/// A type that can be used to build an [`Engine`](struct.Engine.html) instance.
#[derive(Copy, Clone, Debug)]
pub struct EngineBuilder {
    num_threads: usize,
    hash_mb: usize,
}

impl EngineBuilder {
    /// Builds a new [`Engine`](struct.Engine.html) with the options of `self`.
    pub fn build(self) -> Engine {
        let num_threads = match self.num_threads {
            0 => ::num_cpus::get(),
            n => n,
        };
        let hash_mb = match self.hash_mb {
            0 => 1,
            n => n,
        };
        Engine {
            pool:     Pool::new(num_threads),
            table:    Table::new(hash_mb, true),
            position: Position::default(),
            options:  Options { num_threads },
        }
    }

    /// Set the number of threads to be used by the engine.
    ///
    /// If `n` is 0, or you do not call this function, then the number of
    /// threads will be selected automatically. The default is the number of
    /// logical CPUs.
    #[inline]
    pub fn num_threads(mut self, n: usize) -> EngineBuilder {
        self.num_threads = n;
        self
    }

    /// The number of megabytes available for the hash table.
    ///
    /// If `n` is 0, or you do not call this function, then the table size will
    /// be selected automatically. The default is 1.
    #[inline]
    pub fn hash_mb(mut self, n: usize) -> EngineBuilder {
        self.hash_mb = n;
        self
    }
}

/// Chess engine options.
struct Options {
    num_threads: usize,
}

impl Options {
    /// Attempts to set the option of `name` to `value`. Returns `false` if
    /// `name` is not an option.
    fn set(&mut self, name: &str, value: &str) -> bool {
        // Performs a case-insensitive check against the option
        let match_option = |opt: &str| {
            ::util::matches_lower_alpha(opt.as_ref(), name.as_ref())
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
