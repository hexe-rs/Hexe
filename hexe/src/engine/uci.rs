use super::*;

use std::io::{self, BufRead};
use std::mem;
use std::str;

use core::color::Color;
use core::mv::Move;
use engine::Limits;
use engine::thread::Job;

const WHITE: usize = Color::White as usize;
const BLACK: usize = Color::Black as usize;

macro_rules! name { () => { "Hexe" } }

macro_rules! id {
    ($mac:ident) => {
        concat!("id ", stringify!($mac), " ", $mac!())
    }
}

macro_rules! unknown_command {
    ($cmd:expr) => { println!("Unknown command: {}", $cmd) }
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
pub struct Uci<'a> {
    engine: &'a mut Engine,

    // Reusable string buffers
    string_buf_0: String,
    string_buf_1: String,
}

impl<'a> From<&'a mut Engine> for Uci<'a> {
    #[inline]
    fn from(engine: &'a mut Engine) -> Uci<'a> {
        Uci {
            engine,
            string_buf_0: String::new(),
            string_buf_1: String::new(),
        }
    }
}

impl<'a> Uci<'a> {
    /// Returns a reference to the underlying engine over which `self` iterates.
    #[inline]
    pub fn engine(&self) -> &Engine { &self.engine }

    /// Returns a mutable reference to the underlying engine over which `self`
    /// iterates.
    #[inline]
    pub fn engine_mut(&mut self) -> &mut Engine { &mut self.engine }

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
    /// # return;
    /// let mut engine = Engine::default();
    /// engine.uci().start();
    /// ```
    pub fn start(&mut self) {
        info!("Starting UCI from stdin");
        let stdin = io::stdin();
        let lines = stdin.lock().lines().filter_map(Result::ok);
        for line in lines {
            if !self.run_line(&line) {
                break;
            }
        }
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
    /// # return;
    /// let mut engine = Engine::default();
    /// engine.uci().start_with(args);
    /// ```
    pub fn start_with<I>(&mut self, commands: I)
        where I: IntoIterator,
              I::Item: AsRef<str>,
    {
        info!("Starting UCI from iterator");
        for line in commands {
            self.run(line.as_ref());
        }
    }

    /// Runs a single UCI command or multiple if newlines are found.
    #[inline]
    pub fn run(&mut self, command: &str) {
        if command.is_empty() {
            unknown_command!(command);
        } else {
            for line in command.lines() {
                if !self.run_line(line) {
                    break;
                }
            }
        }
    }

    fn run_line(&mut self, line: &str) -> bool {
        debug!("Running UCI command: \"{}\"", line);

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
            "resume"     => self.engine.resume_all(),
            _            => unknown_command!(line),
        }
        true
    }

    fn report_options(&self) {
        println!(
            "\noption name Threads type spin default {0} min 1 max {1}\
             \noption name Hash type spin default 1 min 1 max {1}",
            ::num_cpus::get(),
            usize::MAX,
        );
    }

    fn cmd_uci(&self) {
        println!(id!(name));
        println!(id!(authors));
        self.report_options();
        println!("uciok");
    }

    fn cmd_stop(&mut self) {
        self.engine.stop_all();
    }

    fn cmd_ponder_hit(&mut self) {
        unimplemented!();
    }

    fn cmd_position(&mut self, _: UciIter) {
        unimplemented!();
    }

    fn cmd_set_option(&mut self, mut iter: UciIter) {
        iter.next(); // consume "name"

        let name  = &mut self.string_buf_0;
        let value = &mut self.string_buf_1;

        name.clear();
        value.clear();

        while let Some(next) = iter.next() {
            if next == "value" {
                break;
            }
            if !name.is_empty() {
                name.push(' ');
            }
            name.push_str(next);
        }

        if name.is_empty() {
            error!("No option provided");
            return;
        }

        for next in iter {
            if !value.is_empty() {
                value.push(' ');
            }
            value.push_str(next);
        }

        // Performs a case-insensitive check against the option
        let match_option = |opt: &str| {
            ::util::matches_lower_alpha(opt.as_ref(), name.as_ref())
        };

        debug!("Setting UCI option \"{}\" to \"{}\"", name, value);

        macro_rules! parse {
            ($($x:ident @ $s:expr => $b:expr,)+ _ => $c:expr,) => {
                $(if match_option($s) {
                    match value.parse() {
                        Ok($x) => $b,
                        Err(e) => { parse_error!(value, e); },
                    }
                } else)+ { $c }
            }
        }

        parse! {
            threads @ "threads" => {
                if !self.engine.set_threads(threads) {
                    error!("Cannot set thread count to {}", threads);
                }
            },
            hash @ "hash" => {
                if !self.engine.set_hash_size(hash) {
                    error!("Cannot set table size to {}", hash);
                }
            },
            _ => println!("No such option: {}", name),
        }
    }

    fn cmd_new_game(&mut self) {
        unimplemented!();
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

        self.cmd_start_thinking(limits, moves.into());
    }

    fn cmd_read_move(&self, s: &str) -> Option<Move> {
        unimplemented!();
    }

    fn cmd_start_thinking(&mut self, limits: Limits, moves: Box<[Move]>) {
        let job = Job::Search { limits, moves };
        self.engine.pool.enqueue(job);
    }
}
