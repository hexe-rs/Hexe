use core::mv::Move;
use engine::Limits;
use super::*;

pub enum Job {
    Search {
        limits: Limits,
        moves: Box<[Move]>,
    },
}

pub struct Context<'a> {
    pub index: usize,
    pub worker: &'a mut Worker,
    pub shared: &'a Shared,
}

impl Job {
    /// Executes `self` on thread `id` with mutable access to `Worker` and
    /// a reference to `Shared`.
    pub fn execute(self, context: &mut Context) {
        match self {
            Job::Search { limits, moves } => {
                eprintln!("Thread {} is now executing a search", context.index);
            },
        }
    }
}
