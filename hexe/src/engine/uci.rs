use super::*;

use std::io::{self, BufRead};
use std::mem;
use std::str;

use core::color::Color;
use mv::Move;

const WHITE: usize = Color::White as usize;
const BLACK: usize = Color::Black as usize;

macro_rules! name { () => { "Hexe" } }

macro_rules! authors { () => { "Nikolai Vazquez" } }

macro_rules! id {
    ($mac:ident) => {
        concat!("id ", stringify!($mac), " ", $mac!())
    }
}

struct Limits {
    ponder: bool,
    infinite: bool,
    moves_to_go: u32,
    time: [u32; 2],
    inc: [u32; 2],
    depth: u32,
    nodes: u32,
    mate: u32,
    move_time: u32,
}

impl Default for Limits {
    fn default() -> Limits {
        // Safe because `bool` uses 0 to represent `false`
        unsafe { mem::zeroed() }
    }
}

type UciIter<'a> = str::SplitWhitespace<'a>;

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

    fn uci_position(&mut self, _: UciIter) {

    }

    fn uci_set_option(&mut self, mut iter: UciIter) {
        iter.next(); // consume "name"

        let mut name  = String::new();
        let mut value = String::new();

        while let Some(next) = iter.next() {
            if next == "value" {
                break;
            }
            if !name.is_empty() {
                name.push(' ');
            }
            name.push_str(next);
        }

        while let Some(next) = iter.next() {
            if !value.is_empty() {
                value.push(' ');
            }
            value.push_str(next);
        }

        if !self.options.set(&name, &value) {
            println!("No such option: {}", name);
        }
    }

    fn uci_new_game(&mut self) {

    }

    fn uci_go(&mut self, mut iter: UciIter) {
        let mut limits = Limits::default();
        let mut moves  = Vec::<Move>::new();

        macro_rules! update {
            ($val:expr) => {
                if let Some(Ok(val)) = iter.next().map(str::parse) {
                    $val = val
                }
            }
        }

        while let Some(next) = iter.next() {
            match next {
                "searchmoves" => while let Some(m) = iter.next() {
                    if let Some(mv) = self.uci_read_move(m) {
                        moves.push(mv);
                    }
                },
                "ponder"    => limits.ponder = true,
                "infinite"  => limits.infinite = true,
                "wtime"     => update!(limits.time[WHITE]),
                "btime"     => update!(limits.time[BLACK]),
                "winc"      => update!(limits.inc[WHITE]),
                "binc"      => update!(limits.inc[BLACK]),
                "movestogo" => update!(limits.moves_to_go),
                "depth"     => update!(limits.depth),
                "nodes"     => update!(limits.nodes),
                "mate"      => update!(limits.mate),
                "movetime"  => update!(limits.move_time),
                _ => continue,
            }
        }

        self.uci_start_thinking(&limits, &moves);
    }

    fn uci_read_move(&self, s: &str) -> Option<Move> {
        None
    }

    fn uci_start_thinking(&mut self, limits: &Limits, moves: &[Move]) {

    }
}
