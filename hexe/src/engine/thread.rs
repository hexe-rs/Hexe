use std::thread::{self, JoinHandle};

pub struct Thread {
    index: usize,
    handle: JoinHandle<()>,
}

pub struct Pool {
    threads: Vec<Thread>,
}

impl Pool {
    /// Creates a new pool with `n` number of threads.
    pub fn new(n: usize) -> Pool {
        let mut threads = Vec::<Thread>::with_capacity(n);
        for index in 0..n {
            let handle = thread::spawn(move || loop {
                unimplemented!();
            });
            threads.push(Thread { index, handle });
        }
        Pool { threads }
    }

    /// Returns the number of threads in the pool.
    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }
}
