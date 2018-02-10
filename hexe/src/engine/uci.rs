use super::*;

use std::io::{self, BufRead};
use std::str;

macro_rules! name { () => { "Hexe" } }

macro_rules! authors { () => { "Nikolai Vazquez" } }

macro_rules! id {
    ($mac:ident) => {
        concat!("id ", stringify!($mac), " ", $mac!())
    }
}

/// UCI related functionality.
impl Engine {
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
            let line      = line.as_ref();
            let mut split = line.split_whitespace();
            let cmd       = split.next().unwrap_or("");

            match cmd {
                "quit"       => return,
                "uci"        => self.uci(),
                "stop"       => self.uci_stop(),
                "ponderhit"  => self.uci_ponder_hit(),
                "position"   => self.uci_position(split),
                "setoption"  => self.uci_set_option(split),
                "ucinewgame" => self.uci_new_game(),
                "go"         => self.uci_go(split),
                "isready"    => println!("readyok"),
                _            => println!("Unknown command: {}", line),
            }
        }
    }

    fn uci(&self) {
        println!(id!(name));
        println!(id!(authors));
        println!("uciok");
    }

    fn uci_stop(&mut self) {

    }

    fn uci_ponder_hit(&mut self) {

    }

    fn uci_position(&mut self, _: str::SplitWhitespace) {

    }

    fn uci_set_option(&mut self, _: str::SplitWhitespace) {

    }

    fn uci_new_game(&mut self) {

    }

    fn uci_go(&mut self, _: str::SplitWhitespace) {

    }
}
