use super::*;

use std::io::{self, BufRead};
use std::mem;
use std::str;


use scoped_threadpool::Scope;

use core::color::Color;
use core::mv::Move;
use util::MutRef;

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

/// Runs the engine via the [Universal Chess Interface][uci] (UCI) protocol.
///
/// [uci]: http://wbec-ridderkerk.nl/html/UCIProtocol.html
pub struct Uci<'a>(MutRef<'a, Engine>);

impl<'a> From<&'a mut Engine> for Uci<'a> {
    #[inline]
    fn from(engine: &'a mut Engine) -> Uci<'a> {
        Uci(MutRef::Borrowed(engine))
    }
}

impl<'a> From<Box<Engine>> for Uci<'a> {
    #[inline]
    fn from(engine: Box<Engine>) -> Uci<'a> {
        Uci(MutRef::Owned(engine))
    }
}

impl<'a> From<Engine> for Uci<'a> {
    #[inline]
    fn from(engine: Engine) -> Uci<'a> {
        Box::new(engine).into()
    }
}

impl<'a> Default for Uci<'a> {
    fn default() -> Uci<'a> {
        Uci(MutRef::Owned(Box::default()))
    }
}

impl<'a> Uci<'a> {
    /// Returns a reference to the underlying engine over which `self` iterates.
    #[inline]
    pub fn engine(&self) -> &Engine { &self.0 }

    /// Returns a mutable reference to the underlying engine over which `self`
    /// iterates.
    #[inline]
    pub fn engine_mut(&mut self) -> &mut Engine { &mut self.0 }

    /// Runs the UCI loop, feeding commands from `stdin`.
    ///
    /// This method retains a lock on `stdin` until it exits. To feed commands
    /// differently, use [`start_with`](#method.start_with).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust,norun
    /// use hexe::engine::Engine;
    ///
    /// let mut engine = Engine::default();
    /// engine.uci().start();
    /// ```
    pub fn start(&mut self) {
        let Engine { ref mut pool, ref mut engine } = *self.0;
        pool.scoped(|scope| {
            let stdin = io::stdin();
            let lines = stdin.lock()
                             .lines()
                             .filter_map(Result::ok);
            for line in lines {
                if !engine.run_uci_line(scope, &line) {
                    break;
                }
            }
        });
    }

    /// Runs the UCI loop, feeding commands from an iterator.
    ///
    /// # Examples
    ///
    /// The UCI can be fed command line arguments.
    ///
    /// ```rust,norun
    /// use hexe::engine::Engine;
    /// use std::env;
    ///
    /// let mut args = env::args();
    /// args.next(); // discard program name
    ///
    /// let mut engine = Engine::default();
    /// engine.uci().start_with(args);
    /// ```
    pub fn start_with<I>(&mut self, commands: I)
        where I: IntoIterator,
              I::Item: AsRef<str>,
    {
        let Engine { ref mut pool, ref mut engine } = *self.0;
        pool.scoped(|scope| {
            for line in commands {
                engine.run_uci(scope, line.as_ref());
            }
        });
    }

    /// Runs a single UCI command or multiple if newlines are found.
    #[inline]
    pub fn run(&mut self, command: &str) {
        let Engine { ref mut pool, ref mut engine } = *self.0;
        pool.scoped(|scope| {
            engine.run_uci(scope, command);
        });
    }
}

macro_rules! unknown_command {
    ($cmd:expr) => { println!("Unknown command: {}", $cmd) }
}

impl EngineInner {
    fn run_uci(&mut self, scope: &Scope, command: &str) {
        if command.is_empty() {
            unknown_command!(command);
        } else {
            for line in command.lines() {
                if !self.run_uci_line(scope, line) {
                    break;
                }
            }
        }
    }

    fn run_uci_line(&mut self, scope: &Scope, line: &str) -> bool {
        let mut split = line.split_whitespace();
        match split.next().unwrap_or("") {
            "quit"       => return false,
            "uci"        => self.cmd_uci(),
            "stop"       => self.cmd_stop(),
            "ponderhit"  => self.cmd_ponder_hit(),
            "position"   => self.cmd_position(split),
            "setoption"  => self.cmd_set_option(split),
            "ucinewgame" => self.cmd_new_game(),
            "go"         => self.cmd_go(split),
            "isready"    => println!("readyok"),
            _            => unknown_command!(line),
        }
        true
    }

    fn cmd_uci(&self) {
        println!(id!(name));
        println!(id!(authors));
        self.options.report();
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
