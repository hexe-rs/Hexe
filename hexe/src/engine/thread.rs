use std::mem;
use std::thread::{self, JoinHandle};

use crossbeam_deque::{Deque, Stealer, Steal};

pub struct Thread {
    /// The index for this thread.
    index: usize,
    /// Join up with everyone else.
    handle: JoinHandle<()>,
}

pub struct Job;

impl Job {
    fn execute(&self, thread: usize) {
        unimplemented!("Can't yet execute on thread {}", thread);
    }
}

pub struct Pool {
    /// All threads spawned within this pool.
    threads: Vec<Thread>,
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
        let mut pool = Pool {
            threads: Vec::with_capacity(n),
            jobs: Deque::new(),
        };

        for index in 0..n {
            // Request stealer while in pool thread
            let stealer = pool.jobs.stealer();

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

        pool
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }

    /// A deque of jobs are available to the pool.
    pub fn jobs(&self) -> &Deque<Job> {
        &self.jobs
    }
}
