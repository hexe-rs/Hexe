use std::mem;
use std::thread::{self, JoinHandle};

use crossbeam_deque::{Deque, Stealer, Steal};

mod job;
pub use self::job::Job;

struct Thread {
    /// The index for this thread.
    index: usize,
    /// Join up with everyone else.
    handle: JoinHandle<()>,
}

struct PoolInner {
    /// All threads spawned within this pool.
    threads: Vec<Thread>,
}

#[cfg(test)]
assert_impl!(inner; PoolInner, Send, Sync);

pub struct Pool {
    /// An inner thread pool that must
    inner: PoolInner,
    /// Insertion point for jobs.
    jobs: Deque<Job>,
}

impl Drop for Pool {
    fn drop(&mut self) {
        for thread in self.inner.threads.drain(..) {
            if let Err(_) = thread.handle.join() {
                unreachable!("Thread panicked");
            }
        }
    }
}

impl Pool {
    /// Creates a new pool with `n` number of threads.
    pub fn new(n: usize) -> Pool {
        let mut pool = PoolInner {
            threads: Vec::with_capacity(n),
        };
        let jobs = Deque::<Job>::new();

        for index in 0..n {
            // Request stealer while in pool thread
            let stealer = jobs.stealer();

            let handle = thread::spawn(move || {
                // Move the stealer into worker scope
                let stealer = stealer;
                loop {
                    match stealer.steal() {
                        Steal::Empty => {
                            println!("Thread {} found deque empty", index);
                            return;
                        },
                        Steal::Data(job) => job.execute(index),
                        Steal::Retry => continue,
                    }
                }
            });

            pool.threads.push(Thread {
                index,
                handle,
            });
        }

        Pool { inner: pool, jobs }
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.inner.threads.len()
    }

    /// Enqueues the job to be executed.
    pub fn enqueue(&self, job: Job) {
        self.jobs.push(job);
    }
}
