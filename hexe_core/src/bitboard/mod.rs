//! A bitmap chess board representation.
//!
//! Bitboards conveniently represent chess boards as 64-bit integers. Each bit
//! represents an individual square. Occupancy is represented by the value of
//! each bit.
//!
//! For example, given a bitboard for all pawns and a bitboard for all whites,
//! we can get all white pawns via a bitwise 'and' operation on the two sets:
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
//! Bitboards can also be used to represent multiple piece move destinations
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

pub mod masks;

mod carry_rippler;
pub use self::carry_rippler::*;

mod dir;
pub use self::dir::*;

use core::{fmt, ops, str};
use prelude::*;
use util::Bytes;

#[cfg(feature = "serde")]
use serde::*;
use uncon::*;

impl_rand!(u64 => Bitboard);

/// A mapping of sixty-four bits to squares of a chess board.
///
/// # Examples
///
/// ## Iteration
///
/// Because `Bitboard` implements [`Iterator`], its bits can be traversed over
/// with a `for` loop. This also works in reverse with `.rev()`.
///
/// ```
/// # use hexe_core::prelude::*;
/// for square in Bitboard::FULL {
///     /* ... */
/// }
/// ```
///
/// ## Bit Operation Composition
///
/// Board components ([`Square`], [`File`], and [`Rank`]) can be used first in
/// an operation chain to construct a bitboard.
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
/// assert_eq!(f & r, Bitboard::from(s));
/// assert_eq!(f | r, r | f);
/// assert_eq!(s & (f ^ r), Bitboard::EMPTY);
/// ```
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [`Square`]: ../square/enum.Square.html
/// [`File`]:   ../square/enum.File.html
/// [`Rank`]:   ../square/enum.Rank.html
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

const NOT_FILE_A: u64 = !masks::FILE_A.0;
const NOT_FILE_H: u64 = !masks::FILE_H.0;

const NOT_FILE_AB: u64 = !(masks::FILE_A.0 | masks::FILE_B.0);
const NOT_FILE_GH: u64 = !(masks::FILE_G.0 | masks::FILE_H.0);

#[cfg(feature = "serde")]
impl Serialize for Bitboard {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_u64(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Bitboard {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        u64::deserialize(de).map(From::from)
    }
}

macro_rules! forward_fmt_impl {
    ($($f:ident)+) => {
        $(impl fmt::$f for Bitboard {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::$f::fmt(&self.0, f)
            }
        })+
    }
}

forward_fmt_impl! { Binary Octal LowerHex UpperHex }

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u64);

        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 2 for "0x" + 16 for number
                write!(f, "{:#018X}", self.0)
            }
        }

        f.debug_tuple("Bitboard").field(&Hex(self.0)).finish()
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.map_str(|s| s.fmt(f))
    }
}

macro_rules! forward_sh_impl {
    ($($t1:ident $f1:ident $t2:ident $f2:ident)+) => { $(
        impl<T> ops::$t1<T> for Bitboard where u64: ops::$t1<T, Output=u64> {
            type Output = Self;

            #[inline]
            fn $f1(self, shift: T) -> Self { Bitboard((self.0).$f1(shift)) }
        }

        impl<T> ops::$t2<T> for Bitboard where u64: ops::$t2<T> {
            #[inline]
            fn $f2(&mut self, shift: T) { (self.0).$f2(shift) }
        }
    )+ }
}

forward_sh_impl! {
    Shl shl ShlAssign shl_assign
    Shr shr ShrAssign shr_assign
}

impl_bit_set! { Bitboard !0 => Square }

impl_composition_ops! { Bitboard => Square File Rank }

impl From<u64> for Bitboard {
    #[inline(always)]
    fn from(bits: u64) -> Self { Bitboard(bits) }
}

impl AsRef<u64> for Bitboard {
    #[inline(always)]
    fn as_ref(&self) -> &u64 { &self.0 }
}

impl AsMut<u64> for Bitboard {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut u64 { &mut self.0 }
}

impl From<Bitboard> for u64 {
    #[inline(always)]
    fn from(bb: Bitboard) -> Self { bb.0 }
}

impl AsRef<Bitboard> for u64 {
    #[inline(always)]
    fn as_ref(&self) -> &Bitboard {
        unsafe { self.into_unchecked() }
    }
}

impl AsMut<Bitboard> for u64 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Bitboard {
        unsafe { self.into_unchecked() }
    }
}

impl From<Square> for Bitboard {
    #[inline]
    fn from(square: Square) -> Self {
        Bitboard(1 << square as usize)
    }
}

impl From<File> for Bitboard {
    #[inline]
    fn from(file: File) -> Self {
        masks::FILE_A << file as usize
    }
}

impl From<Rank> for Bitboard {
    #[inline]
    fn from(rank: Rank) -> Self {
        masks::RANK_1 << ((rank as usize) << 3)
    }
}

impl From<Color> for Bitboard {
    #[inline]
    fn from(color: Color) -> Self {
        match color {
            Color::White => Bitboard::WHITE,
            Color::Black => Bitboard::BLACK,
        }
    }
}

impl Bitboard {
    /// White board squares.
    pub const WHITE: Bitboard = Bitboard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: Bitboard = Bitboard(0xAA55AA55AA55AA55);

    /// Generates a random bitboard with few bits set.
    #[inline]
    #[cfg(any(test, feature = "rand"))]
    pub fn rand_sparse<R: ::rand::Rng>(rng: &mut R) -> Bitboard {
        Bitboard(rng.next_u64() & rng.next_u64() & rng.next_u64())
    }

    /// Returns a `Bitboard` containing squares between `start` and `end`.
    #[inline]
    pub fn between(start: Square, end: Square) -> Bitboard {
        start.between(end)
    }

    /// Returns a `Bitboard` line spanning the entire board from edge to edge,
    /// intersecting `start` and `end`.
    #[inline]
    pub fn line(start: Square, end: Square) -> Bitboard {
        start.line(end)
    }

    /// Returns whether `self` has an empty rank.
    #[inline]
    pub fn contains_empty_rank(self) -> bool {
        self.0.contains_zero_byte()
    }

    /// Returns whether the rank in `self` is empty.
    #[inline]
    pub fn rank_is_empty(self, rank: Rank) -> bool {
        (self.0 >> rank as usize) & 0xFF == 0
    }

    /// Returns whether the file in `self` is empty.
    #[inline]
    pub fn file_is_empty(self, file: File) -> bool {
        const EMPTY: u64 = 0x0101010101010101;
        (self.0 >> file as usize) & EMPTY == 0
    }

    /// Returns whether the path for `right` is empty within `self`.
    #[inline]
    pub fn path_is_empty(self, right: CastleRight) -> bool {
        (self & right.path()).is_empty()
    }

    /// Returns an iterator over the subsets of `self`.
    #[inline]
    pub fn carry_rippler(self) -> CarryRippler {
        self.into()
    }

    /// Generates pawn attacks for each of the bits of `self`.
    #[inline]
    pub fn pawn_attacks(self, color: Color) -> Bitboard {
        use self::Direction::*;
        match color {
            Color::White => self.shift(Northeast) | self.shift(Northwest),
            Color::Black => self.shift(Southeast) | self.shift(Southwest),
        }
    }

    /// Generates knight attacks for each of the bits of `self`.
    #[inline]
    pub fn knight_attacks(self) -> Bitboard {
        let l1 = (self >> 1) & NOT_FILE_H;
        let l2 = (self >> 2) & NOT_FILE_GH;
        let r1 = (self << 1) & NOT_FILE_A;
        let r2 = (self << 2) & NOT_FILE_AB;
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
    }

    /// Generates bishop attacks for each of the bits of `self`.
    pub fn bishop_attacks(self, empty: Bitboard) -> Bitboard {
        use self::Direction::*;
        self.fill_shift(Northeast, empty) | self.fill_shift(Northwest, empty) |
        self.fill_shift(Southeast, empty) | self.fill_shift(Southwest, empty)
    }

    /// Generates rook attacks for each of the bits of `self`.
    pub fn rook_attacks(self, empty: Bitboard) -> Bitboard {
        use self::Direction::*;
        self.fill_shift(North, empty) | self.fill_shift(East, empty) |
        self.fill_shift(South, empty) | self.fill_shift(West, empty)
    }

    /// Generates king attacks for each of the bits of `self`.
    #[inline]
    pub fn king_attacks(self) -> Bitboard {
        use self::Direction::*;
        let attacks = self.shift(East) | self.shift(West);
        let combine = self | attacks;
        attacks | combine.shift(North) | combine.shift(South)
    }

    /// Generates queen attacks for each of the bits of `self`.
    pub fn queen_attacks(self, empty: Bitboard) -> Bitboard {
        self.bishop_attacks(empty) | self.rook_attacks(empty)
    }

    /// Returns `self` advanced by one rank for `color`.
    #[inline]
    pub fn advance(self, color: Color) -> Bitboard {
        self.shift(Direction::forward(color))
    }

    /// Returns `self` retreated by one rank for `color`.
    #[inline]
    pub fn retreat(self, color: Color) -> Bitboard {
        self.shift(Direction::backward(color))
    }

    /// Returns `self` shifted in a direction.
    #[inline]
    pub fn shift(self, direction: Direction) -> Bitboard {
        use self::Direction::*;
        match direction {
            North     => self << 8,
            South     => self >> 8,
            East      => self << 1 & NOT_FILE_A,
            West      => self >> 1 & NOT_FILE_H,
            Northeast => self << 9 & NOT_FILE_A,
            Southeast => self >> 7 & NOT_FILE_A,
            Northwest => self << 7 & NOT_FILE_H,
            Southwest => self >> 9 & NOT_FILE_H,
        }
    }

    /// Returns `self` filled in a direction, blocked off by non-empty squares.
    #[inline]
    pub fn fill(mut self, direction: Direction, mut empty: Bitboard) -> Bitboard {
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
            North, Bitboard::FULL, 8, <<;
            South, Bitboard::FULL, 8, >>;

            East, NOT_FILE_A, 1, <<;
            West, NOT_FILE_H, 1, >>;

            Northeast, NOT_FILE_A, 9, <<;
            Southeast, NOT_FILE_A, 7, >>;
            Northwest, NOT_FILE_H, 7, <<;
            Southwest, NOT_FILE_H, 9, >>;
        }
    }

    fn fill_shift(self, direction: Direction, empty: Bitboard) -> Bitboard {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_is_empty() {
        static EMPTIES: [u64; 8] = [
            0xFEFEFEFEFEFEFEFE, 0xFDFDFDFDFDFDFDFC,
            0xFBFBFBFBFBFBFBF8, 0xF7F7F7F7F7F7F7F0,
            0xEFEFEFEFEFEFEFE0, 0xDFDFDFDFDFDFDFC0,
            0xBFBFBFBFBFBFBF80, 0x7F7F7F7F7F7F7F00,
        ];

        for file in File::ALL {
            let mut value = Bitboard::FULL;
            let mut check = |slice| {
                for &x in slice {
                    value &= x;
                    assert!(value.file_is_empty(file));
                }
            };
            check(&EMPTIES[(file as usize)..]);
            check(&EMPTIES[..(file as usize)]);
        }
    }
}
