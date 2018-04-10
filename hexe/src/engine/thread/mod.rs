use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_deque::{Deque, Stealer, Steal};
use parking_lot::{Condvar, Mutex};

use core::mv::Move;
use engine::Limits;
use position::Position;
use table::Table;
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

    /// The transposition table.
    pub table: Table,
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
    /// The job stealer.
    pub jobs: Stealer<Job>,
}

impl<'ctx> Context<'ctx> {
    /// Runs the worker loop within the thread context.
    pub fn run(mut self) {
        loop {
            match self.try_next() {
                Ok(_) => {},
                Err(Interrupt::Kill) => return,
                Err(Interrupt::Stop) => self.stop(),
            }
        }
    }

    /// Returns whether the thread should be killed or stopped as `Result::Err`.
    fn interrupt(&self) -> Result<(), Interrupt> {
        if self.worker.kill.load(Ordering::SeqCst) {
            Err(Interrupt::Kill)
        } else if self.shared.stop.load(Ordering::SeqCst) {
            Err(Interrupt::Stop)
        } else {
            Ok(())
        }
    }

    /// Attempts to steal and execute the next job with a chance of being
    /// interrupted.
    fn try_next(&mut self) -> Result<(), Interrupt> {
        self.interrupt()?;

        eprintln!("Thread {} attempting steal", self.thread);
        match self.jobs.steal() {
            Steal::Empty => {
                eprintln!("Thread {} found empty deque", self.thread);
                let mut guard = self.shared.empty_mutex.lock();

                eprintln!("Thread {} now waiting", self.thread);
                self.shared.empty_cond.wait(&mut guard);

                eprintln!("Thread {} finished waiting", self.thread);
                Ok(())
            },
            Steal::Data(job) => self.execute(job),
            Steal::Retry => Ok(()),
        }
    }

    /// Executes `job` within the worker thread context.
    fn execute(&mut self, job: Job) -> Result<(), Interrupt> {
        // Check if we're being asked to exit before making any progress
        self.interrupt()?;

        match job {
            Job::Search { limits, moves } => {
                eprintln!("Thread {} is now searching", self.thread);
            },
        }

        eprintln!("Thread {} finished job", self.thread);
        Ok(())
    }

    /// Stops the thread unconditionally.
    #[cold]
    fn stop(&self) {
        eprintln!("Thread {} should stop", self.thread);
        let mut guard = self.shared.stop_mutex.lock();

        eprintln!("Thread {} will stop now", self.thread);
        self.shared.stop_cond.wait(&mut guard);
    }
}

/// A request to stop current progress.
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    /// Stop progress for all threads.
    Stop,
    /// Immediately kill the current thread.
    Kill,
}
