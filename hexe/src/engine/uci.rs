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

/// Runs the engine via the [Universal Chess Interface][uci] (UCI) protocol.
///
/// [uci]: http://wbec-ridderkerk.nl/html/UCIProtocol.html
pub struct Uci<'a>(UciInner<'a>);

/// A type like Cow with mutability and without the `Clone` restriction.
///
/// Whether the Engine is owned or mutably borrowed, there is no cost of getting
/// a reference to the underlying engine.
enum UciInner<'a> {
    Borrowed(&'a mut Engine),
    Owned(Box<Engine>),
}

impl<'a> From<&'a mut Engine> for Uci<'a> {
    #[inline]
    fn from(engine: &'a mut Engine) -> Uci<'a> {
        Uci(UciInner::Borrowed(engine))
    }
}

impl<'a> From<Box<Engine>> for Uci<'a> {
    #[inline]
    fn from(engine: Box<Engine>) -> Uci<'a> {
        Uci(UciInner::Owned(engine))
    }
}

impl<'a> From<Engine> for Uci<'a> {
    #[inline]
    fn from(engine: Engine) -> Uci<'a> {
        Box::new(engine).into()
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

impl<'a> Uci<'a> {
    /// Returns a reference to the underlying engine over which `self` iterates.
    #[inline]
    pub fn engine(&self) -> &Engine {
        match self.0 {
            UciInner::Borrowed(ref engine) => engine,
            UciInner::Owned(ref engine) => &**engine,
        }
    }

    /// Returns a mutable reference to the underlying engine over which `self`
    /// iterates.
    #[inline]
    pub fn engine_mut(&mut self) -> &mut Engine {
        match self.0 {
            UciInner::Borrowed(ref mut engine) => engine,
            UciInner::Owned(ref mut engine) => &mut **engine,
        }
    }

    /// Runs the UCI loop, feeding commands from `stdin`.
    ///
    /// This method retains a lock on `stdin` until it exits. To feed commands
    /// differently, use [`start_with`](#method.start_with).
    pub fn start(&mut self) {
        let stdin = io::stdin();
        let lines = stdin.lock()
                         .lines()
                         .filter_map(Result::ok);
        self.start_with(lines);
    }

    /// Runs the UCI loop, feeding commands from an iterator.
    pub fn start_with<I>(&mut self, commands: I)
        where I: IntoIterator,
              I::Item: AsRef<str>,
    {
        let engine = self.engine_mut();
        for line in commands {
            engine.run_uci(line.as_ref());
        }
    }

    /// Runs a single UCI command or multiple if newlines are found.
    #[inline]
    pub fn run(&mut self, command: &str) {
        self.engine_mut().run_uci(command);
    }
}

impl Engine {
    fn run_uci(&mut self, command: &str) {
        for line in command.lines() {
            self.run_uci_line(line);
        }
    }

    fn run_uci_line(&mut self, line: &str) {
        let mut split = line.split_whitespace();
        let cmd       = split.next().unwrap_or("");

        match cmd {
            "quit"       => return,
            "uci"        => self.cmd_uci(),
            "stop"       => self.cmd_stop(),
            "ponderhit"  => self.cmd_ponder_hit(),
            "position"   => self.cmd_position(split),
            "setoption"  => self.cmd_set_option(split),
            "ucinewgame" => self.cmd_new_game(),
            "go"         => self.cmd_go(split),
            "isready"    => println!("readyok"),
            _            => println!("Unknown command: {}", line),
        }
    }

    fn cmd_uci(&self) {
        println!(id!(name));
        println!(id!(authors));
        println!("uciok");
    }

    fn cmd_stop(&mut self) {

    }

    fn cmd_ponder_hit(&mut self) {

    }

    fn cmd_position(&mut self, _: UciIter) {

    }

    fn cmd_set_option(&mut self, mut iter: UciIter) {
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

        for next in iter {
            if !value.is_empty() {
                value.push(' ');
            }
            value.push_str(next);
        }

        if !self.options.set(&name, &value) {
            println!("No such option: {}", name);
        }
    }

    fn cmd_new_game(&mut self) {

    }

    fn cmd_go(&mut self, mut iter: UciIter) {
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
                    if let Some(mv) = self.cmd_read_move(m) {
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

        self.cmd_start_thinking(&limits, &moves);
    }

    fn cmd_read_move(&self, s: &str) -> Option<Move> {
        None
    }

    fn cmd_start_thinking(&mut self, limits: &Limits, moves: &[Move]) {

    }
}
