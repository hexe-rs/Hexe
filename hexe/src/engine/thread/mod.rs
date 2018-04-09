use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossbeam_deque::{Deque, Steal};
use parking_lot::{Condvar, Mutex};

use core::mv::Move;
use engine::Limits;
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

pub struct Context<'a> {
    pub index: usize,
    pub worker: &'a mut Worker,
    pub shared: &'a Shared,
}

impl Job {
    /// Executes `self` on thread `id` with mutable access to `Worker` and
    /// a reference to `Shared`.
    pub fn execute(self, context: &mut Context) {
        match self {
            Job::Search { limits, moves } => {
                eprintln!("Thread {} is now executing a search", context.index);
            },
        }
    }
}
