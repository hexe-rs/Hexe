//! A bitmap chess board representation.
//!
//! `BitBoard` conveniently represents chess boards as 64-bit integers. Each bit
//! represents an individual square. Occupancy is represented by the value of
//! each bit.
//!
//! For example, given a `BitBoard` for all pawns and a `BitBoard` for all
//! whites, we can get all white pawns via a bitwise 'and' operation on the two
//! sets:
//!
//! ```txt
//! Pawns:            White:
//! . . . . . . . .   . . . . . . . .
//! 1 1 1 1 1 1 1 1   . . . . . . . .
//! . . . . . . . .   . . . . . . . .
//! . . . . . . . . & . . . . . . . .
//! . . . . . . . .   . . . . . . . .
//! . . . . . . . .   . . . . . . . .
//! 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1
//! . . . . . . . .   1 1 1 1 1 1 1 1
//!
//! White Pawns:
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! 1 1 1 1 1 1 1 1
//! . . . . . . . .
//! ```
//!
//! `BitBoard` can also be used to represent multiple piece move destinations
//! simultaneously:
//!
//! ```txt
//! Knight attacks at D4:
//! . . . . . . . .
//! . . . . . . . .
//! . . 1 . 1 . . .
//! . 1 . . . 1 . .
//! . . . . . . . .
//! . 1 . . . 1 . .
//! . . 1 . 1 . . .
//! . . . . . . . .
//! ```
//!
//! This is actually how [`Square::D4.knight_attacks()`][ka] works internally:
//! via a lookup table.
//!
//! [ka]: ../square/enum.Square.html#method.knight_attacks

use core::{fmt, ops, str};

#[cfg(feature = "serde")]
use serde::*;
use uncon::*;

use misc::Direction;
use prelude::*;
use util::Bytes;

pub mod masks;

mod carry_rippler;
pub use self::carry_rippler::*;

#[cfg(all(test, nightly))]
mod benches;

impl_rand!(u64 => BitBoard);

/// A mapping of sixty-four bits to squares of a chess board.
///
/// # Examples
///
/// ## Iteration
///
/// Because `BitBoard` implements [`Iterator`], its bits can be traversed over
/// with a `for` loop. This also works in reverse with `.rev()`.
///
/// ```
/// # use hexe_core::prelude::*;
/// for square in BitBoard::FULL {
///     /* ... */
/// }
/// ```
///
/// ## Bit Operation Composition
///
/// Board components ([`Square`], [`File`], and [`Rank`]) can be used first in
/// an operation chain to construct a `BitBoard`.
///
/// This syntax should not be misused to create obscure operations that are hard
/// to follow.
///
/// ```
/// # use hexe_core::prelude::*;
/// let f = File::B;
/// let r = Rank::Seven;
/// let s = Square::new(f, r);
///
/// assert_eq!(f & r, BitBoard::from(s));
/// assert_eq!(f | r, r | f);
/// assert_eq!(s & (f ^ r), BitBoard::EMPTY);
/// ```
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [`Square`]: ../square/enum.Square.html
/// [`File`]:   ../square/enum.File.html
/// [`Rank`]:   ../square/enum.Rank.html
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

const NOT_FILE_A: u64 = !masks::FILE_A.0;
const NOT_FILE_H: u64 = !masks::FILE_H.0;

const NOT_FILE_AB: u64 = !(masks::FILE_A.0 | masks::FILE_B.0);
const NOT_FILE_GH: u64 = !(masks::FILE_G.0 | masks::FILE_H.0);

#[cfg(feature = "serde")]
impl Serialize for BitBoard {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_u64(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BitBoard {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        u64::deserialize(de).map(From::from)
    }
}

macro_rules! forward_fmt_impl {
    ($($f:ident)+) => {
        $(impl fmt::$f for BitBoard {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::$f::fmt(&self.0, f)
            }
        })+
    }
}

forward_fmt_impl! { Binary Octal LowerHex UpperHex }

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u64);

        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 2 for "0x" + 16 for number
                write!(f, "{:#018X}", self.0)
            }
        }

        f.debug_tuple("BitBoard").field(&Hex(self.0)).finish()
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.map_str(|s| s.fmt(f))
    }
}

macro_rules! forward_sh_impl {
    ($($t1:ident $f1:ident $t2:ident $f2:ident)+) => { $(
        impl<T> ops::$t1<T> for BitBoard where u64: ops::$t1<T, Output=u64> {
            type Output = Self;

            #[inline]
            fn $f1(self, shift: T) -> Self { BitBoard((self.0).$f1(shift)) }
        }

        impl<T> ops::$t2<T> for BitBoard where u64: ops::$t2<T> {
            #[inline]
            fn $f2(&mut self, shift: T) { (self.0).$f2(shift) }
        }
    )+ }
}

forward_sh_impl! {
    Shl shl ShlAssign shl_assign
    Shr shr ShrAssign shr_assign
}

impl_bit_set! { BitBoard !0 => Square }

impl_composition_ops! { BitBoard => Square File Rank }

impl From<u64> for BitBoard {
    #[inline(always)]
    fn from(bits: u64) -> Self { BitBoard(bits) }
}

impl AsRef<u64> for BitBoard {
    #[inline(always)]
    fn as_ref(&self) -> &u64 { &self.0 }
}

impl AsMut<u64> for BitBoard {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut u64 { &mut self.0 }
}

impl From<BitBoard> for u64 {
    #[inline(always)]
    fn from(bb: BitBoard) -> Self { bb.0 }
}

impl AsRef<BitBoard> for u64 {
    #[inline(always)]
    fn as_ref(&self) -> &BitBoard {
        unsafe { self.into_unchecked() }
    }
}

impl AsMut<BitBoard> for u64 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BitBoard {
        unsafe { self.into_unchecked() }
    }
}

impl From<Square> for BitBoard {
    #[inline]
    fn from(square: Square) -> Self {
        BitBoard(1 << square as usize)
    }
}

impl From<File> for BitBoard {
    #[inline]
    fn from(file: File) -> Self {
        masks::FILE_A << file as usize
    }
}

impl From<Rank> for BitBoard {
    #[inline]
    fn from(rank: Rank) -> Self {
        masks::RANK_1 << ((rank as usize) << 3)
    }
}

impl From<Color> for BitBoard {
    #[inline]
    fn from(color: Color) -> Self {
        match color {
            Color::White => BitBoard::WHITE,
            Color::Black => BitBoard::BLACK,
        }
    }
}

impl BitBoard {
    /// White board squares.
    pub const WHITE: BitBoard = BitBoard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: BitBoard = BitBoard(0xAA55AA55AA55AA55);

    /// Generates a random `BitBoard` with few bits set.
    #[inline]
    #[cfg(any(test, feature = "rand"))]
    pub fn rand_sparse<R: ::rand::Rng>(rng: &mut R) -> BitBoard {
        BitBoard(rng.next_u64() & rng.next_u64() & rng.next_u64())
    }

    /// Returns a `BitBoard` containing squares between `start` and `end`.
    #[inline]
    pub fn between(start: Square, end: Square) -> BitBoard {
        start.between(end)
    }

    /// Returns a `BitBoard` line spanning the entire board from edge to edge,
    /// intersecting `start` and `end`.
    #[inline]
    pub fn line(start: Square, end: Square) -> BitBoard {
        start.line(end)
    }

    /// Returns whether `self` has an empty rank.
    #[inline]
    pub fn contains_empty_rank(self) -> bool {
        self.0.contains_zero_byte()
    }

    /// Returns whether the path for `right` is empty within `self`.
    #[inline]
    pub fn path_is_empty(self, right: Right) -> bool {
        (self & right.path()).is_empty()
    }

    /// Returns an iterator over the subsets of `self`.
    #[inline]
    pub fn carry_rippler(self) -> CarryRippler {
        self.into()
    }

    /// Generates pawn attacks for each of the bits of `self`.
    #[inline]
    pub fn pawn_attacks(self, color: Color) -> BitBoard {
        use self::Direction::*;
        match color {
            Color::White => self.shift(UpRight)   | self.shift(UpLeft),
            Color::Black => self.shift(DownRight) | self.shift(DownLeft),
        }
    }

    /// Generates knight attacks for each of the bits of `self`.
    #[inline]
    pub fn knight_attacks(self) -> BitBoard {
        let l1 = (self >> 1) & NOT_FILE_H;
        let l2 = (self >> 2) & NOT_FILE_GH;
        let r1 = (self << 1) & NOT_FILE_A;
        let r2 = (self << 2) & NOT_FILE_AB;
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
    }

    /// Generates bishop attacks for each of the bits of `self`.
    pub fn bishop_attacks(self, empty: BitBoard) -> BitBoard {
        use self::Direction::*;
        self.fill_shift(UpRight,   empty) | self.fill_shift(UpLeft,   empty) |
        self.fill_shift(DownRight, empty) | self.fill_shift(DownLeft, empty)
    }

    /// Generates rook attacks for each of the bits of `self`.
    pub fn rook_attacks(self, empty: BitBoard) -> BitBoard {
        use self::Direction::*;
        self.fill_shift(Up,   empty) | self.fill_shift(Right, empty) |
        self.fill_shift(Down, empty) | self.fill_shift(Left,  empty)
    }

    /// Generates king attacks for each of the bits of `self`.
    #[inline]
    pub fn king_attacks(self) -> BitBoard {
        use self::Direction::*;
        let attacks = self.shift(Right) | self.shift(Left);
        let combine = self | attacks;
        attacks | combine.shift(Up) | combine.shift(Down)
    }

    /// Generates queen attacks for each of the bits of `self`.
    pub fn queen_attacks(self, empty: BitBoard) -> BitBoard {
        self.bishop_attacks(empty) | self.rook_attacks(empty)
    }

    /// Returns `self` advanced by one rank for `color`.
    #[inline]
    pub fn advance(self, color: Color) -> BitBoard {
        self.shift(Direction::forward(color))
    }

    /// Returns `self` retreated by one rank for `color`.
    #[inline]
    pub fn retreat(self, color: Color) -> BitBoard {
        self.shift(Direction::backward(color))
    }

    /// Returns `self` shifted in a direction (relative to white's perspective).
    #[inline]
    pub fn shift(self, direction: Direction) -> BitBoard {
        use self::Direction::*;
        match direction {
            Up        => self << 8,
            Down      => self >> 8,
            Right     => self << 1 & NOT_FILE_A,
            Left      => self >> 1 & NOT_FILE_H,
            UpRight   => self << 9 & NOT_FILE_A,
            DownRight => self >> 7 & NOT_FILE_A,
            UpLeft    => self << 7 & NOT_FILE_H,
            DownLeft  => self >> 9 & NOT_FILE_H,
        }
    }

    /// Returns `self` filled in a direction (relative to white's perspective),
    /// blocked off by non-empty squares.
    #[inline]
    pub fn fill(mut self, direction: Direction, mut empty: BitBoard) -> BitBoard {
        macro_rules! impl_fills {
            ($($v:ident, $mask:expr, $shift:expr, $op:tt;)+) => {
                match direction {
                    $(Direction::$v => {
                        const SHIFT_1: u8 = $shift;
                        const SHIFT_2: u8 = SHIFT_1 * 2;
                        const SHIFT_3: u8 = SHIFT_2 * 2;

                        empty &= $mask;
                        self  |= empty & (self $op SHIFT_1);
                        empty &= empty $op SHIFT_1;
                        self  |= empty & (self $op SHIFT_2);
                        empty &= empty $op SHIFT_2;
                        self  |= empty & (self $op SHIFT_3);
                    }),+
                }
                self
            }
        }
        impl_fills! {
            Up,   BitBoard::FULL, 8, <<;
            Down, BitBoard::FULL, 8, >>;

            Right, NOT_FILE_A, 1, <<;
            Left,  NOT_FILE_H, 1, >>;

            UpRight,   NOT_FILE_A, 9, <<;
            DownRight, NOT_FILE_A, 7, >>;
            UpLeft,    NOT_FILE_H, 7, <<;
            DownLeft,  NOT_FILE_H, 9, >>;
        }
    }

    fn fill_shift(self, direction: Direction, empty: BitBoard) -> BitBoard {
        self.fill(direction, empty).shift(direction)
    }

    /// Returns the result of applying a function to a mutable string
    /// representation of `self`.
    #[inline]
    pub fn map_str<T, F: FnOnce(&mut str) -> T>(&self, f: F) -> T {
        let mut buf = *::consts::BOARD_DOTS;
        for idx in self.map(|s| (0b111000 ^ s as usize) << 1) {
            unsafe { *buf.get_unchecked_mut(idx) = b'1' };
        }
        unsafe { f(str::from_utf8_unchecked_mut(&mut buf)) }
    }
}
