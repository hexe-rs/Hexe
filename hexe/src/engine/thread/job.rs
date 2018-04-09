pub struct Job;

impl Job {
    pub fn execute(&self, thread: usize) {
        eprintln!("Thread {} is now executing a job", thread);
    }
}
