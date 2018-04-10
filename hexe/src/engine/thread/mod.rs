use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_deque::{Deque, Steal};
use parking_lot::{Condvar, Mutex};

use core::mv::Move;
use engine::Limits;
use position::Position;
use util::AnySend;

mod pool;
pub use self::pool::Pool;

struct Thread {
    /// Data unique to this thread.
    ///
    /// Although the pool owns this pointer, only its thread may access mutably.
    ///
    /// Boxed to ensure a stable address.
    worker: Box<Worker>,
    /// Join up with everyone else.
    handle: JoinHandle<()>,
}

/// Data unique to a given thread. The pool may not access it mutably, but the
/// corresponding running thread may if data.
pub struct Worker {
    kill: AtomicBool,
}

/// Data shared between the pool and threads.
pub struct Shared {
    /// The condition variable for the deque being empty.
    empty_cond: Condvar,
    /// The mutex associated with `empty_cond`.
    empty_mutex: Mutex<()>,
}

#[cfg(test)]
assert_impl!(shared; Shared, Send, Sync);

pub enum Job {
    Search {
        limits: Limits,
        moves: Box<[Move]>,
    },
}

/// Context data available to a worker thread.
pub struct Context<'ctx> {
    /// The thread identifier.
    pub thread: usize,
    /// The thread's unique worker data.
    pub worker: &'ctx mut Worker,
    /// Data shared with all threads.
    pub shared: &'ctx Shared,
    /// The current position.
    pub position: Position,
}

impl Job {
    /// Executes `self` within `context`.
    pub fn execute(self, context: &mut Context) {
        match self {
            Job::Search { limits, moves } => {
                eprintln!("Thread {} is now searching", context.thread);
            },
        }
        eprintln!("Thread {} finished job", context.thread);
    }
}
