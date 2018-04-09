use std::mem;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Condvar, Mutex};

use crossbeam_deque::{Deque, Stealer, Steal};

mod job;
pub use self::job::Job;

struct Thread {
    /// The index for this thread.
    index: usize,
    /// Join up with everyone else.
    handle: JoinHandle<()>,
}

/// Data shared between the pool and threads.
struct Shared {
    /// The condition variable for the deque being empty.
    empty_cond: Condvar,
    /// The mutex associated with `empty_cond`.
    empty_mutex: Mutex<()>,
}

#[cfg(test)]
assert_impl!(shared; Shared, Send, Sync);

pub struct Pool {
    /// All threads spawned within this pool.
    threads: Vec<Thread>,
    /// Our handle on the shared data.
    shared: Arc<Shared>,
    /// Insertion point for jobs.
    jobs: Deque<Job>,
}

impl Drop for Pool {
    fn drop(&mut self) {
        for thread in self.threads.drain(..) {
            if let Err(_) = thread.handle.join() {
                unreachable!("Thread panicked");
            }
        }
    }
}

impl Pool {
    /// Creates a new pool with `n` number of threads.
    pub fn new(n: usize) -> Pool {
        let mut threads = Vec::<Thread>::with_capacity(n);

        let jobs = Deque::<Job>::new();

        let shared = Arc::new(Shared {
            empty_cond: Condvar::new(),
            empty_mutex: Mutex::default(),
        });

        for index in 0..n {
            let stealer = jobs.stealer();
            let shared  = Arc::clone(&shared);

            let handle = thread::spawn(move || {
                // Move all shared data into worker thread scope
                let stealer = stealer;
                let shared  = shared;

                loop {
                    match stealer.steal() {
                        Steal::Empty => {
                            eprintln!("Thread {} about to get guard", index);
                            let guard = shared.empty_mutex.lock().unwrap();
                            eprintln!("Thread {} is now waiting", index);
                            shared.empty_cond.wait(guard).unwrap();
                            eprintln!("Thread {} finished waiting", index);
                        },
                        Steal::Data(job) => job.execute(index),
                        Steal::Retry => continue,
                    }
                }
            });

            threads.push(Thread {
                index,
                handle,
            });
        }

        Pool { threads, shared, jobs }
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }

    /// Enqueues the job to be executed.
    pub fn enqueue(&self, job: Job) {
        self.jobs.push(job);
        self.shared.empty_cond.notify_one();
    }
}
