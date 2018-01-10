//! A chess move.

mod vec;
pub use self::vec::*;

use prelude::*;
use core::piece::Promotion;

const SRC_SHIFT:  usize =  0;
const DST_SHIFT:  usize =  6;
const PROM_SHIFT: usize = 12;
const KIND_SHIFT: usize = 14;

const SRC_MASK:  u16 = 0b111111;
const DST_MASK:  u16 = SRC_MASK;
const PROM_MASK: u16 = 0b11;
const KIND_MASK: u16 = PROM_MASK;

macro_rules! base_bits {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << SRC_SHIFT) | (($s2 as u16) << DST_SHIFT)
    }
}

/// A chess piece move from a start `Square` to an end `Square` that carries
/// metadata for promotion and move kind.
///
/// - 6 bits for source square
/// - 6 bits for destination square
/// - 2 bits for promotion piece kind
/// - 2 bits for move kind
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move(u16);

impl Move {
    /// Creates a new `Move` from one square to another with a promotion and
    /// move kind.
    #[inline]
    pub fn new(from: Square, to: Square, prom: Promotion, kind: MoveKind) -> Move {
        Move(base_bits!(from, to)
            | ((prom as u16) << PROM_SHIFT)
            | ((kind as u16) << KIND_SHIFT))
    }

    /// Returns the source square for `self`.
    #[inline]
    pub fn src(self) -> Square {
        ((self.0 >> SRC_SHIFT) & SRC_MASK).into()
    }

    /// Returns the destination square for `self`.
    #[inline]
    pub fn dst(self) -> Square {
        ((self.0 >> DST_SHIFT) & DST_MASK).into()
    }

    /// Returns the promotion for `self`.
    #[inline]
    pub fn prom(self) -> Promotion {
        ((self.0 >> PROM_SHIFT) & PROM_MASK).into()
    }

    /// Returns the kind for `self`.
    #[inline]
    pub fn kind(self) -> MoveKind {
        ((self.0 >> KIND_SHIFT) & KIND_MASK).into()
    }
}

/// A chess piece move kind.
#[derive(PartialEq, Eq, Clone, Copy, Hash, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum MoveKind {
    /// Normal move.
    Normal,
    /// [Castling][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Castling
    Castle,
    /// [Promotion][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Promotion_(chess)
    Promotion,
    /// [En passant][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/En_passant
    EnPassant,
}
