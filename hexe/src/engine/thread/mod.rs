use std::mem;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_deque::{Deque, Stealer, Steal};

use util::AnySend;

mod job;
pub use self::job::Job;

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
struct Worker {
    kill: AtomicBool,
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
        self.kill_all();
        for thread in self.threads.drain(..) {
            if thread.handle.join().is_err() {
                unreachable!("Thread panicked");
            }
        }
    }
}

impl Pool {
    /// Creates a new pool with `n` number of threads.
    pub fn new(n: usize) -> Pool {
        let mut pool = Pool {
            threads: Vec::<Thread>::with_capacity(n),
            shared: Arc::new(Shared {
                empty_cond: Condvar::new(),
                empty_mutex: Mutex::default(),
            }),
            jobs: Deque::<Job>::new(),
        };
        pool.add_threads(n);
        pool
    }

    /// Adds `n` number of threads to the pool.
    pub fn add_threads(&mut self, n: usize) {
        let start = self.num_threads();
        let range = start..(start + n);

        for index in range {
            let stealer = self.jobs.stealer();
            let shared  = Arc::clone(&self.shared);

            // The pool owns the pointer to the unique value
            let mut worker = Box::new(Worker {
                kill: AtomicBool::new(false),
            });

            // Wrap up in order to send to the corresponding thread
            let worker_ptr = AnySend::new(&mut *worker as *mut Worker);

            let handle = thread::spawn(move || {
                // Move all shared data into worker thread scope
                let stealer = stealer;
                let unique  = unsafe { &mut *worker_ptr.get() };
                let shared  = shared;

                while !unique.kill.load(Ordering::SeqCst) {
                    eprintln!("Thread {} about to attempt steal", index);
                    match stealer.steal() {
                        Steal::Empty => {
                            eprintln!("Thread {} found empty deque", index);
                            let guard = shared.empty_mutex.lock().unwrap();

                            eprintln!("Thread {} is now waiting", index);
                            drop(shared.empty_cond.wait(guard).unwrap());

                            eprintln!("Thread {} finished waiting", index);
                        },
                        Steal::Data(job) => job.execute(index),
                        Steal::Retry => continue,
                    }
                }

                eprintln!("Thread {} about to exit", index);
            });

            self.threads.push(Thread { worker, handle });
        }
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }

    /// Kills all threads.
    pub fn kill_all(&self) {
        for thread in &self.threads {
            thread.worker.kill.store(true, Ordering::SeqCst);
        }
        // Wake up anyone sleeping until the next enqueue
        self.shared.empty_cond.notify_all();
    }

    /// Enqueues the job to be executed.
    pub fn enqueue(&self, job: Job) {
        self.jobs.push(job);
        self.shared.empty_cond.notify_one();
    }
}
