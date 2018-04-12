use std::cmp;
use std::ops;

use position::Position;
use super::*;

pub struct Pool {
    /// All threads spawned within this pool.
    threads: Vec<Thread>,
    /// Owning handle on the shared data.
    shared: Box<Shared>,
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
    /// Creates a new pool with `n` number of threads and `size_mb` number of
    /// megabytes available in the shared transposition table.
    pub fn new(n: usize, size_mb: usize) -> Pool {
        let mut pool = Pool {
            threads: Default::default(),
            shared: Box::new(
                Shared {
                    table: Table::new(size_mb),
                    .. Default::default()
                }
            ),
            jobs: Default::default(),
        };
        pool.set_threads(n);
        pool
    }

    /// Sets the number of threads to `n`.
    pub fn set_threads(&mut self, n: usize) {
        debug!("Setting engine threads to {}", n);

        let cur = self.num_threads();

        match n.cmp(&cur) {
            cmp::Ordering::Equal   => return,
            cmp::Ordering::Greater => self.add_range(cur..n),
            cmp::Ordering::Less    => self.rem_after(n),
        }
    }

    /// Removes all threads after `n` in `self`.
    fn rem_after(&mut self, n: usize) {
        for thread in &self.threads[n..] {
            thread.worker.kill();
        }

        // Wake up anyone who might have been erm... killed?
        self.shared.empty_cond.notify_all();
        self.shared.stop_cond.notify_all();

        for thread in self.threads.drain(n..) {
            thread.handle.join();
        }
    }

    /// Adds `n` number of threads to the pool.
    pub fn add_threads(&mut self, n: usize) {
        let i = self.num_threads();
        self.add_range(i..(i + n));
    }

    fn add_range(&mut self, range: ops::Range<usize>) {
        self.threads.reserve(range.len());

        for index in range {
            let stealer = self.jobs.stealer();

            // The pool owns the pointer to the unique value
            let mut worker = Box::<Worker>::default();

            // The pool owns the boxed values and no worker outlives it
            let worker_ptr = AnySend::new(&*worker as *const _);
            let shared_ptr = AnySend::new(&*self.shared as *const _);

            let handle = thread::spawn(move || {
                let context = Context {
                    thread: index,
                    worker: unsafe { &*worker_ptr.get() },
                    shared: unsafe { &*shared_ptr.get() },
                    position: Position::default(),
                    jobs: stealer,
                };
                context.run();
                trace!("Thread {} about to exit", index);
            });

            self.threads.push(Thread { worker, handle });
        }
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }

    /// Stops what each thread is currently doing.
    pub fn stop_all(&self) {
        self.shared.stop()
    }

    /// Resumes all stopped threads.
    pub fn resume_all(&self) {
        trace!("Resuming all stopped threads");
        self.shared.stop.store(false, Ordering::SeqCst);
        self.shared.stop_cond.notify_all();
    }

    /// Attempts to kill `thread`, returning whether or not it is in the pool.
    pub fn kill(&self, thread: usize) -> bool {
        if let Some(thread) = self.threads.get(thread) {
            thread.worker.kill();
            true
        } else {
            false
        }
    }

    /// Kills all threads.
    pub fn kill_all(&self) {
        trace!("Killing all threads");
        for thread in &self.threads {
            thread.worker.kill.store(true, Ordering::SeqCst);
        }
        // Wake up anyone sleeping
        self.shared.empty_cond.notify_all();
        self.resume_all();
    }

    /// Returns a reference to the data shared by all threads.
    pub fn shared(&self) -> &Shared { &self.shared }

    /// Returns a mutable reference to the data shared by all threads.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no thread is currently accessing `Shared`,
    /// such as with thread pool resizing.
    pub unsafe fn shared_mut(&mut self) -> &mut Shared {
        &mut self.shared
    }

    /// Enqueues the job to be executed.
    pub fn enqueue(&self, job: Job) {
        self.jobs.push(job);
        self.shared.empty_cond.notify_one();
    }
}
