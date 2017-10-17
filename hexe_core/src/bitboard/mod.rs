//! A bitmap chess board representation.

pub mod masks;
mod impls;

/// A mapping of sixty-four bits to squares of a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

const NOT_FILE_A: u64 = !masks::FILE_A.0;
const NOT_FILE_H: u64 = !masks::FILE_H.0;

impl Bitboard {
    /// A bitboard with all bits set to 1.
    pub const FULL: Bitboard = Bitboard(!0);

    /// A bitboard with all bits set to 0.
    pub const EMPTY: Bitboard = Bitboard(0);

    /// White board squares.
    pub const WHITE: Bitboard = Bitboard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: Bitboard = Bitboard(0xAA55AA55AA55AA55);

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
    pub fn fill(self, direction: Direction, empty: Bitboard) -> Bitboard {
        use self::Direction::*;
        match direction {
            North     => self.fill_north(empty),
            South     => self.fill_south(empty),
            East      => self.fill_east(empty),
            West      => self.fill_west(empty),
            Northeast => self.fill_northeast(empty),
            Southeast => self.fill_southeast(empty),
            Northwest => self.fill_northwest(empty),
            Southwest => self.fill_southwest(empty),
        }
    }
}

macro_rules! impl_fills {
    ($($f:ident, $mask:expr, $shift:expr, $op:tt;)+) => {
        impl Bitboard { $(
            #[inline]
            fn $f(mut self, mut empty: Bitboard) -> Bitboard {
                const SHIFT_1: u8 = $shift;
                const SHIFT_2: u8 = SHIFT_1 * 2;
                const SHIFT_3: u8 = SHIFT_2 * 2;

                empty &= $mask;
                self  |= empty & (self $op SHIFT_1);
                empty &= empty $op SHIFT_1;
                self  |= empty & (self $op SHIFT_2);
                empty &= empty $op SHIFT_2;
                self  |= empty & (self $op SHIFT_3);
                self
            }
        )+ }
    }
}

impl_fills! {
    fill_north, Bitboard::FULL, 8, <<;
    fill_south, Bitboard::FULL, 8, >>;

    fill_east, NOT_FILE_A, 1, <<;
    fill_west, NOT_FILE_H, 1, >>;

    fill_northeast, NOT_FILE_A, 9, <<;
    fill_southeast, NOT_FILE_A, 7, >>;
    fill_northwest, NOT_FILE_H, 7, <<;
    fill_southwest, NOT_FILE_H, 9, >>;
}

/// A cardinal direction that can be used to shift or fill the bits of a
/// `Bitboard`.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Southeast,
    Northwest,
    Southwest
}
