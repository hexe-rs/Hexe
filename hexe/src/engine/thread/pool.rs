use position::Position;
use super::*;

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

                let ref mut context = Context {
                    thread: index,
                    worker: unsafe { &mut *worker_ptr.get() },
                    shared: &shared,
                    position: Position::default(),
                };

                while !context.worker.kill.load(Ordering::SeqCst) {
                    eprintln!("Thread {} about to attempt steal", index);
                    match stealer.steal() {
                        Steal::Empty => {
                            eprintln!("Thread {} found empty deque", index);
                            let mut guard = shared.empty_mutex.lock();

                            eprintln!("Thread {} is now waiting", index);
                            shared.empty_cond.wait(&mut guard);

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
