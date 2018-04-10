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
            threads: Vec::<Thread>::with_capacity(n),
            shared:  Box::new(Shared {
                table: Table::new(size_mb, true),
                .. Default::default()
            }),
            jobs:    Deque::<Job>::default(),
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

            // The pool owns the pointer to the unique value
            let mut worker = Box::<Worker>::default();

            // The pool owns the boxed values and no worker outlives the pool
            let worker_ptr = AnySend::new(&*worker as *const Worker);
            let shared_ptr = AnySend::new(&*self.shared as *const Shared);

            let handle = thread::spawn(move || {
                let mut context = Context {
                    thread: index,
                    worker: unsafe { &*worker_ptr.get() },
                    shared: unsafe { &*shared_ptr.get() },
                    position: Position::default(),
                    jobs: stealer,
                };
                context.run();
                eprintln!("Thread {} about to exit", index);
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
        eprintln!("Stopping all threads");
        self.shared.stop.store(true, Ordering::SeqCst);
        self.shared.empty_cond.notify_all();
    }

    /// Resumes all stopped threads.
    pub fn resume_all(&self) {
        eprintln!("Resuming all stopped threads");
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
        eprintln!("Killing all threads");
        for thread in &self.threads {
            thread.worker.kill.store(true, Ordering::SeqCst);
        }
        // Wake up anyone sleeping
        self.shared.empty_cond.notify_all();
        self.resume_all();
    }

    /// Returns a reference to the data shared by all threads.
    pub fn shared(&self) -> &Shared { &self.shared }

    /// Enqueues the job to be executed.
    pub fn enqueue(&self, job: Job) {
        self.jobs.push(job);
        self.shared.empty_cond.notify_one();
    }
}
