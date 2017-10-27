use super::*;
use core::mem;
use prelude::*;

const NONE: u8 = 1 + Piece::BlackKing as u8;

/// A mapping of sixty-four squares to pieces.
///
/// This allows for faster lookups than possible with bitboards.
#[derive(Copy, Clone)]
pub struct PieceMap([u8; 64]);

impl PartialEq for PieceMap {
    #[inline]
    fn eq(&self, other: &PieceMap) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Eq for PieceMap {}

impl Default for PieceMap {
    #[inline]
    fn default() -> PieceMap {
        PieceMap::EMPTY
    }
}

impl PieceMap {
    /// An empty piece map.
    pub const EMPTY: PieceMap = PieceMap([NONE; 64]);

    /// Inserts the piece at a square.
    #[inline]
    pub fn insert(&mut self, sq: Square, pc: Piece) {
        self.0[sq as usize] = pc as u8;
    }

    /// Removes the piece at a square.
    #[inline]
    pub fn remove(&mut self, sq: Square) {
        self.0[sq as usize] = NONE;
    }

    /// Replaces the piece at a square with a new one and returns the previous
    /// piece, if any.
    #[inline]
    pub fn replace(&mut self, sq: Square, pc: Piece) -> Option<Piece> {
        match mem::replace(&mut self.0[sq as usize], pc as u8) {
            NONE => None,
            p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns the piece at a square, if any.
    #[inline]
    pub fn get(&self, sq: Square) -> Option<Piece> {
        match self.0[sq as usize] {
            NONE => None,
            p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns the piece at a square without checking.
    ///
    /// # Safety
    ///
    /// Calling this method when there's no piece the given square will produce
    /// [undefined behavior][ub]. Use with caution.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn get_unchecked(&self, sq: Square) -> Piece {
        self.0[sq as usize].into_unchecked()
    }
}
