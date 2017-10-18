//! A bitmap chess board representation.

pub mod masks;
mod impls;
mod tables;

use prelude::*;

/// A mapping of sixty-four bits to squares of a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

const NOT_FILE_A: u64 = !masks::FILE_A.0;
const NOT_FILE_H: u64 = !masks::FILE_H.0;

const NOT_FILE_AB: u64 = !(masks::FILE_A.0 | masks::FILE_B.0);
const NOT_FILE_GH: u64 = !(masks::FILE_G.0 | masks::FILE_H.0);

impl Bitboard {
    /// A bitboard with all bits set to 1.
    pub const FULL: Bitboard = Bitboard(!0);

    /// A bitboard with all bits set to 0.
    pub const EMPTY: Bitboard = Bitboard(0);

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
