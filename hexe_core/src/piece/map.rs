use super::*;
use core::mem;
use prelude::*;

const NONE: u8 = 1 + Piece::BlackKing as u8;

/// A mapping of sixty-four squares to pieces.
///
/// This allows for faster lookups than possible with bitboards.
pub struct PieceMap([u8; 64]);

impl Clone for PieceMap {
    #[inline]
    fn clone(&self) -> PieceMap { PieceMap(self.0) }
}

impl Copy for PieceMap {}

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

    /// Creates a new piece map by instantiating each slot with the provided
    /// initializer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::*;
    /// let piece_map = PieceMap::from_init(|sq| {
    ///     # None
    ///     /* ... */
    /// });
    /// ```
    #[inline]
    pub fn from_init<F>(mut init: F) -> PieceMap
        where F: FnMut(Square) -> Option<Piece>
    {
        let mut map: PieceMap = unsafe { mem::uninitialized() };
        for (i, slot) in map.0.iter_mut().enumerate() {
            *slot = init(i.into()).map(|p| p as u8).unwrap_or(NONE);
        }
        map
    }

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

    /// Returns a reference to the piece at a square, if any.
    #[inline]
    pub fn get(&self, sq: Square) -> Option<&Piece> {
        match self.0[sq as usize] {
            NONE => None,
            ref p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns a mutable reference to the piece at a square, if any.
    #[inline]
    pub fn get_mut(&mut self, sq: Square) -> Option<&mut Piece> {
        match self.0[sq as usize] {
            NONE => None,
            ref mut p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns a reference to the piece at a square without checking.
    ///
    /// # Safety
    ///
    /// Calling this method when there's no piece at the given square will
    /// produce [undefined behavior][ub]. Use with caution.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn get_unchecked(&self, sq: Square) -> &Piece {
        (&self.0[sq as usize]).into_unchecked()
    }

    /// Returns a mutable reference to the piece at a square without checking.
    ///
    /// # Safety
    ///
    /// Calling this method when there's no piece at the given square will
    /// produce [undefined behavior][ub]. Use with caution.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, sq: Square) -> &mut Piece {
        (&mut self.0[sq as usize]).into_unchecked()
    }
}
