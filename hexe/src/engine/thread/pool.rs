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
    /// Creates a new pool with `n` number of threads.
    pub fn new(n: usize) -> Pool {
        let mut pool = Pool {
            threads: Vec::<Thread>::with_capacity(n),
            shared:  Box::<Shared>::default(),
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
                // Move all shared data into worker thread scope
                let stealer = stealer;

                let ref mut context = Context {
                    thread: index,
                    worker: unsafe { &*worker_ptr.get() },
                    shared: unsafe { &*shared_ptr.get() },
                    position: Position::default(),
                };

                while !context.worker.kill.load(Ordering::SeqCst) {
                    eprintln!("Thread {} about to attempt steal", index);
                    match stealer.steal() {
                        Steal::Empty => {
                            eprintln!("Thread {} found empty deque", index);
                            let mut guard = context.shared.empty_mutex.lock();

                            eprintln!("Thread {} is now waiting", index);
                            context.shared.empty_cond.wait(&mut guard);

                            eprintln!("Thread {} finished waiting", index);
                        },
                        Steal::Data(job) => job.execute(context),
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
        eprintln!("Killing all threads");
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
