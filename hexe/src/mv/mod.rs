//! A chess move.

mod vec;
pub use self::vec::*;

use prelude::*;
use core::piece::Promotion;

const START_SHIFT: usize =  0;
const END_SHIFT:   usize =  6;
const PROM_SHIFT:  usize = 12;
const KIND_SHIFT:  usize = 14;

macro_rules! base_bits {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << START_SHIFT) | (($s2 as u16) << END_SHIFT)
    }
}

/// A chess piece move from a start `Square` to an end `Square` that carries
/// metadata for promotion and move kind.
///
/// - 6 bits for start square
/// - 6 bits for end square
/// - 2 bits for promotion piece kind
/// - 2 bits for move kind
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move(u16);

impl Move {
    /// Creates a new `Move` from `start` to `end` squares with a promotion and
    /// move kind.
    #[inline]
    pub fn new(start: Square, end: Square, prom: Promotion, kind: MoveKind) -> Move {
        Move(base_bits!(start, end)
            | ((prom as u16) << PROM_SHIFT)
            | ((kind as u16) << KIND_SHIFT))
    }

    /// Returns the start square for `self`.
    #[inline]
    pub fn start(&self) -> Square {
        ((self.0 >> START_SHIFT) & 0x3F).into()
    }

    /// Returns the start square for `self`.
    #[inline]
    pub fn end(&self) -> Square {
        ((self.0 >> END_SHIFT) & 0x3F).into()
    }

    /// Returns the promotion for `self`.
    #[inline]
    pub fn promotion(&self) -> Promotion {
        ((self.0 >> PROM_SHIFT) & 0x3).into()
    }

    /// Returns the kind for `self`.
    #[inline]
    pub fn kind(&self) -> MoveKind {
        ((self.0 >> KIND_SHIFT) & 0x3).into()
    }
}

/// A chess piece move kind.
#[derive(PartialEq, Eq, Clone, Copy, Hash, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum MoveKind {
    Normal,
    Castle,
    Promotion,
    EnPassant,
}
