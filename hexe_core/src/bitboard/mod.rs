//! A bitmap chess board representation.

pub mod masks;
mod carry_rippler;
mod impls;
mod tables;

use core::str;
use prelude::*;
pub use self::carry_rippler::*;

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

impl Bitboard {
    /// White board squares.
    pub const WHITE: Bitboard = Bitboard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: Bitboard = Bitboard(0xAA55AA55AA55AA55);

    /// Returns a `Bitboard` containing squares between `start` and `end`.
    #[inline]
    pub fn between(start: Square, end: Square) -> Bitboard {
        self::tables::BETWEEN[start as usize][end as usize].into()
    }

    /// Returns a `Bitboard` line spanning the entire board from edge to edge,
    /// intersecting `start` and `end`.
    #[inline]
    pub fn line(start: Square, end: Square) -> Bitboard {
        self::tables::LINE[start as usize][end as usize].into()
    }

    /// Returns whether the path for `right` is empty within `self`.
    #[inline]
    pub fn path_is_empty(self, right: CastleRight) -> bool {
        (self & right.path()).is_empty()
    }

    /// Returns an iterator over the subsets of `self`.
    #[inline]
    pub fn carry_rippler(self) -> CarryRippler {
        CarryRippler::new(self)
    }

    /// Generates pawn pushes for each of the bits of `self`.
    #[inline]
    pub fn pawn_pushes(self, color: Color) -> Bitboard {
        let direction = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };
        self.shift(direction)
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
        self.shift(match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        })
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
    pub fn map_str<F, T>(&self, f: F) -> T
        where F: for<'a> FnOnce(&'a mut str) -> T
    {
        let mut buf = *b". . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .\n\
                         . . . . . . . .";
        for idx in self.map(|s| (0b111000 ^ s as usize) << 1) {
            unsafe { *buf.get_unchecked_mut(idx) = b'1' };
        }
        unsafe { f(str::from_utf8_unchecked_mut(&mut buf)) }
    }
}

/// A cardinal direction that can be used to shift or fill the bits of a
/// [`Bitboard`](struct.Bitboard.html).
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    /// North (up).
    North,
    /// South (down).
    South,
    /// East (right).
    East,
    /// West (left).
    West,
    /// Northeast (up + right).
    Northeast,
    /// Southeast (down + right).
    Southeast,
    /// Northwest (up + left).
    Northwest,
    /// Southwest (down + left).
    Southwest
}
