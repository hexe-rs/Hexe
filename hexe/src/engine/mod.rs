//! The Hexe chess engine as a self-contained type.

// TODO lint when everything is implemented
#![allow(unused_variables)]

use std::usize;

use table::Table;

mod limits;
pub(crate) use self::limits::Limits;

mod thread;
use self::thread::Pool;

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
}

impl Default for Engine {
    #[inline]
    fn default() -> Engine { Engine::new() }
}

impl Engine {
    /// Creates a new builder for setting engine options.
    #[inline]
    pub fn builder() -> EngineBuilder {
        EngineBuilder(Options {
            num_threads: 0,
            hash_size: 0,
        })
    }

    /// Creates an instance of the engine with the default options applied.
    #[inline]
    pub fn new() -> Engine {
        Engine::builder().build()
    }

    /// The internal options used by this instance.
    #[inline]
    pub fn options(&self) -> Options {
        Options {
            num_threads: self.num_threads(),
            hash_size: self.table.size_mb(),
        }
    }

    /// Creates a Universal Chess Interface for this engine.
    #[inline]
    pub fn uci(&mut self) -> Uci {
        Uci::from(self)
    }

    /// Stops all worker threads of the engine.
    pub fn stop_all(&self) {
        self.pool.stop_all();
    }

    /// Resumes all stopped worker threads.
    pub fn resume_all(&self) {
        self.pool.resume_all();
    }

    /// Attempts to kill `thread`, returning whether or not it is in the pool.
    pub fn kill(&self, thread: usize) -> bool {
        self.pool.kill(thread)
    }

    /// Kills all worker threads of the engine.
    ///
    /// New threads need to be spawned to continue execution of queued jobs.
    pub fn kill_all(&self) {
        self.pool.kill_all();
    }

    /// Returns the number of threads that the engine currently has spawned.
    #[inline]
    pub fn num_threads(&self) -> usize {
        self.pool.num_threads()
    }
}

/// A type that can be used to build an [`Engine`](struct.Engine.html) instance.
#[derive(Copy, Clone, Debug)]
pub struct EngineBuilder(Options);

impl From<Options> for EngineBuilder {
    #[inline]
    fn from(opt: Options) -> EngineBuilder {
        EngineBuilder(opt)
    }
}

impl EngineBuilder {
    /// Builds a new [`Engine`](struct.Engine.html) with the options of `self`.
    pub fn build(self) -> Engine {
        let num_threads = match self.0.num_threads {
            0 => ::num_cpus::get(),
            n => n,
        };
        let hash_size = match self.0.hash_size {
            0 => 1,
            n => n,
        };
        Engine {
            pool:  Pool::new(num_threads),
            table: Table::new(hash_size, true),
        }
    }

    /// Set the number of threads to be used by the engine.
    ///
    /// If `n` is 0, or you do not call this function, then the number of
    /// threads will be selected automatically. The default is the number of
    /// logical CPUs.
    #[inline]
    pub fn num_threads(mut self, n: usize) -> EngineBuilder {
        self.0.num_threads = n;
        self
    }

    /// The number of megabytes available for the transposition table.
    ///
    /// If `n` is 0, or you do not call this function, then the table size will
    /// be selected automatically. The default is 1.
    #[inline]
    pub fn hash_size(mut self, size_mb: usize) -> EngineBuilder {
        self.0.hash_size = size_mb;
        self
    }
}

/// Chess engine options.
#[derive(Copy, Clone, Debug)]
pub struct Options {
    /// The number of threads in the engine's thread pool.
    pub num_threads: usize,
    /// The number of megabytes for the engine's transposition table.
    pub hash_size: usize,
}
