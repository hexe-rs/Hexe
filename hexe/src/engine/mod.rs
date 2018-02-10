//! The Hexe chess engine as a self-contained type.

use std::io::{self, BufRead};

/// An instance of the Hexe chess engine.
pub struct Engine {
}

impl Engine {
    /// Creates an instance of the engine.
    pub fn new() -> Engine {
        Engine {}
    }

    /// Runs the UCI (Universal Chess Interface) loop.
    ///
    /// This method retains a lock on `stdin` until it exits. To feed lines
    /// differently, use [`start_uci_with`](#method.start_uci_with).
    pub fn start_uci(&mut self) {
        let stdin = io::stdin();
        let lines = stdin.lock()
                         .lines()
                         .filter_map(Result::ok);
        self.start_uci_with(lines);
    }

    /// Runs the UCI (Universal Chess Interface) loop.
    ///
    /// UCI commands are fed via the `lines` iterator.
    pub fn start_uci_with<I>(&mut self, lines: I)
        where I: IntoIterator,
              I::Item: AsRef<str>,
    {
        for line in lines {
            let mut split = line.as_ref().split_whitespace();
            let cmd: &str = split.next().unwrap_or("");

            match cmd {
                "quit"       => return,
                "uci"        => (),
                "isready"    => println!("readyok"),
                "ucinewgame" => (),
                "go"         => (),
                _            => println!("Unknown command: {}", cmd),
            }
        }
    }
}

