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
#[derive(Default)]
pub struct Worker {
    kill: AtomicBool,
}

impl Worker {
    fn kill(&self) {
        self.kill.store(true, Ordering::SeqCst);
    }
}

/// Data shared between the pool and threads.
#[derive(Default)]
pub struct Shared {
    /// The condition variable for the deque being empty.
    empty_cond: Condvar,
    /// The mutex associated with `empty_cond`.
    empty_mutex: Mutex<()>,

    /// Whether or not to stop all searches.
    stop: AtomicBool,
    stop_cond: Condvar,
    stop_mutex: Mutex<()>,
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
    pub worker: &'ctx Worker,
    /// Data shared with all threads.
    pub shared: &'ctx Shared,
    /// The current position.
    pub position: Position,
}

impl<'ctx> Context<'ctx> {
    /// Stops the thread unconditionally.
    #[cold]
    pub fn stop(&self) {
        loop {
            eprintln!("Thread {} should stop", self.thread);
            let mut guard = self.shared.stop_mutex.lock();

            eprintln!("Thread {} will stop now", self.thread);
            self.shared.stop_cond.wait(&mut guard);

            if !self.shared.stop.load(Ordering::SeqCst) {
                break;
            }
        }
    }

    /// Stops the thread if it needs to be.
    pub fn try_stop(&self) {
        if self.shared.stop.load(Ordering::SeqCst) {
            self.stop();
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Interruption {
    Stop,
    Kill,
}

impl Job {
    /// Executes `self` within `context`.
    pub fn execute(self, context: &mut Context) -> Option<Interruption> {
        macro_rules! interrupt {
            () => {
                if context.shared.stop.load(Ordering::SeqCst) {
                    return Some(Interruption::Stop);
                }
                if context.worker.kill.load(Ordering::SeqCst) {
                    return Some(Interruption::Kill);
                }
            }
        }

        interrupt!();

        match self {
            Job::Search { limits, moves } => {
                eprintln!("Thread {} is now searching", context.thread);
            },
        }

        eprintln!("Thread {} finished job", context.thread);
        None
    }
}
