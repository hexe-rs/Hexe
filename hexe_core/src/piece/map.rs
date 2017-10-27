//! A square to piece mapping for fast square lookups.

use super::*;
use core::marker::PhantomData;
use core::mem;
use core::ops;
use square::Squares;
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

impl ops::Index<Square> for PieceMap {
    type Output = Piece;

    #[inline]
    fn index(&self, sq: Square) -> &Piece {
        self.get(sq).expect("no piece found for square")
    }
}

impl ops::IndexMut<Square> for PieceMap {
    #[inline]
    fn index_mut(&mut self, sq: Square) -> &mut Piece {
        self.get_mut(sq).expect("no piece found for square")
    }
}

impl PieceMap {
    /// An empty piece map.
    pub const EMPTY: PieceMap = PieceMap([NONE; 64]);

    /// Creates an empty piece map.
    #[inline]
    pub fn new() -> PieceMap {
        PieceMap::default()
    }

    /// Creates a new piece map by instantiating each slot with the provided
    /// initializer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
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

    /// Returns the first square and piece pair in the map.
    #[inline]
    pub fn first(&self) -> Option<(Square, &Piece)> {
        self.iter().next()
    }

    /// Returns the first square and mutable piece pair in the map.
    #[inline]
    pub fn first_mut(&mut self) -> Option<(Square, &mut Piece)> {
        self.iter_mut().next()
    }

    /// Returns the last square and piece pair in the map.
    #[inline]
    pub fn last(&self) -> Option<(Square, &Piece)> {
        self.iter().next_back()
    }

    /// Returns the last square and mutable piece pair in the map.
    #[inline]
    pub fn last_mut(&mut self) -> Option<(Square, &mut Piece)> {
        self.iter_mut().next_back()
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

    /// Clears the map, removing all pieces.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = [NONE; 64];
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

    /// Returns whether the map contains a piece at the given square.
    #[inline]
    pub fn contains(&self, sq: Square) -> bool {
        self.0[sq as usize] != NONE
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

    /// Returns an iterator visiting all square-piece pairs in order.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter { map: self, iter: Squares::default() }
    }

    /// Returns an iterator visiting all square-piece pairs mutably in order.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { map: self, iter: Squares::default(), _marker: PhantomData }
    }
}

impl<'a> IntoIterator for &'a PieceMap {
    type Item = (Square, &'a Piece);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> { self.iter() }
}

impl<'a> IntoIterator for &'a mut PieceMap {
    type Item = (Square, &'a mut Piece);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> IterMut<'a> { self.iter_mut() }
}

/// A [`PeiceMap`](struct.PieceMap.html) iterator.
pub struct Iter<'a> {
    map: &'a PieceMap,
    iter: Squares,
}

#[cfg(test)]
assert_impl!(iter; Iter<'static>, Send, Sync);

impl<'a> Iterator for Iter<'a> {
    type Item = (Square, &'a Piece);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next() {
            if let Some(pc) = self.map.get(sq) {
                return Some((sq, pc));
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next_back() {
            if let Some(pc) = self.map.get(sq) {
                return Some((sq, pc));
            }
        }
        None
    }
}

/// A mutable [`PeiceMap`](struct.PieceMap.html) iterator.
pub struct IterMut<'a> {
    map: *mut PieceMap,
    iter: Squares,
    // Rust doesn't like mutable borrows here
    _marker: PhantomData<&'a mut PieceMap>,
}

#[cfg(test)]
assert_impl!(iter_mut; IterMut<'static>, Send, Sync);

unsafe impl<'a> Send for IterMut<'a> {}
unsafe impl<'a> Sync for IterMut<'a> {}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Square, &'a mut Piece);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next() {
            let map = unsafe { &mut *self.map };
            if let Some(pc) = map.get_mut(sq) {
                return Some((sq, pc));
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next_back() {
            let map = unsafe { &mut *self.map };
            if let Some(pc) = map.get_mut(sq) {
                return Some((sq, pc));
            }
        }
        None
    }
}
