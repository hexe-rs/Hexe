//! A square to piece mapping for fast square lookups.

use super::*;
use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::ops;
use square::Squares;
use prelude::*;

const NONE: u8 = 1 + Piece::BlackKing as u8;

static _EMPTY: [u8; 64] = [NONE; 64];

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

impl fmt::Debug for PieceMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl ::core::iter::FromIterator<(Square, Piece)> for PieceMap {
    #[inline]
    fn from_iter<T: IntoIterator<Item=(Square, Piece)>>(iter: T) -> PieceMap {
        let mut map = PieceMap::new();
        map.extend(iter);
        map
    }
}

impl Extend<(Square, Piece)> for PieceMap {
    #[inline]
    fn extend<T: IntoIterator<Item=(Square, Piece)>>(&mut self, iter: T) {
        for (s, p) in iter.into_iter() {
            self.insert(s, p);
        }
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

    /// Reverses the square mapping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// # use hexe_core::prelude::*;
    /// let mut map = PieceMap::new();
    /// let piece = Piece::WhitePawn;
    ///
    /// map.insert(Square::A1, piece);
    /// map.reverse();
    ///
    /// assert_eq!(map[Square::H8], piece);
    /// ```
    #[inline]
    pub fn reverse(&mut self) {
        self.0.reverse()
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

    /// Performs a raw replacement.
    #[inline]
    unsafe fn __insert(&mut self, sq: Square, pc: u8) -> Option<Piece> {
        match mem::replace(&mut self.0[sq as usize], pc) {
            NONE => None,
            p => Some(p.into_unchecked())
        }
    }

    /// Inserts the piece at a square, returning the previous one if any.
    #[inline]
    pub fn insert(&mut self, sq: Square, pc: Piece) -> Option<Piece> {
        unsafe { self.__insert(sq, pc as u8) }
    }

    /// Removes the piece at a square.
    #[inline]
    pub fn remove(&mut self, sq: Square) -> Option<Piece> {
        unsafe { self.__insert(sq, NONE) }
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
        where F: FnMut(Square, &mut Piece) -> bool
    {
        for (i, slot) in self.0.iter_mut().enumerate() {
            if *slot != NONE && !f(i.into(), unsafe { slot.into_unchecked() }) {
                *slot = NONE;
            }
        }
    }

    /// Clears the map, removing all pieces.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = [NONE; 64];
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0[..] == _EMPTY[..]
    }

    /// Returns the number of pieces in `self`.
    #[inline]
    pub fn len(&self) -> usize {
        let mut len = self.0.len();
        for &slot in self.0.iter() {
            if slot == NONE {
                len -= 1;
            }
        }
        len
    }

    /// Returns whether the map contains the value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// # use hexe_core::prelude::*;
    /// let sq = Square::B7;
    /// let pc = Piece::WhiteRook;
    ///
    /// let mut map = PieceMap::new();
    /// map.insert(sq, pc);
    ///
    /// assert!(map.contains(sq));
    /// assert!(map.contains(pc));
    /// ```
    #[inline]
    pub fn contains<T: Contained>(&self, value: T) -> bool {
        value.contained_in(self)
    }

    /// Returns the first square for the piece.
    #[inline]
    pub fn find(&self, pc: Piece) -> Option<Square> {
        if let Some(index) = ::memchr::memchr(pc as u8, &self.0) {
            unsafe { Some(index.into_unchecked()) }
        } else {
            None
        }
    }

    /// Returns the last square for the piece.
    #[inline]
    pub fn rfind(&self, pc: Piece) -> Option<Square> {
        if let Some(index) = ::memchr::memrchr(pc as u8, &self.0) {
            unsafe { Some(index.into_unchecked()) }
        } else {
            None
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

    /// Returns the color of the piece at the given square, if any.
    #[inline]
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.get(sq).map(Piece::color)
    }

    /// Returns the kind of the piece at the given square, if any.
    #[inline]
    pub fn kind_at(&self, sq: Square) -> Option<PieceKind> {
        self.get(sq).map(Piece::kind)
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

/// A [`PieceMap`](struct.PieceMap.html) iterator.
#[derive(Clone)]
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
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

impl<'a> ExactSizeIterator for Iter<'a> {
    #[inline]
    fn len(&self) -> usize {
        let mut len = 0;
        for square in self.iter.clone() {
            if self.map.0[square as usize] != NONE {
                len += 1;
            }
        }
        len
    }
}

impl<'a> fmt::Debug for Iter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A mutable [`PieceMap`](struct.PieceMap.html) iterator.
#[derive(Clone)]
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

impl<'a> From<IterMut<'a>> for Iter<'a> {
    #[inline]
    fn from(iter: IterMut) -> Iter {
        Iter { map: iter._map(), iter: iter.iter }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Square, &'a mut Piece);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next() {
            if let Some(pc) = self._map_mut().get_mut(sq) {
                return Some((sq, pc));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.iter.next_back() {
            if let Some(pc) = self._map_mut().get_mut(sq) {
                return Some((sq, pc));
            }
        }
        None
    }
}

impl<'a> ExactSizeIterator for IterMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        let mut len = 0;
        for square in self.iter.clone() {
            if self._map().0[square as usize] != NONE {
                len += 1;
            }
        }
        len
    }
}

impl<'a> fmt::Debug for IterMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a> IterMut<'a> {
    #[inline]
    fn _map(&self) -> &'a PieceMap {
        unsafe { &*self.map }
    }

    #[inline]
    fn _map_mut(&mut self) -> &'a mut PieceMap {
        unsafe { &mut *self.map }
    }
}

/// A type whose instance may be contained in a [`PieceMap`](struct.PieceMap.html).
pub trait Contained {
    /// Returns whether `self` is contained in `map`.
    fn contained_in(self, map: &PieceMap) -> bool;
}

impl Contained for Square {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        map.0[self as usize] != NONE
    }
}

impl Contained for Piece {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        map.find(self).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut map = PieceMap::new();

        macro_rules! assert_len {
            ($l:expr) => {
                assert_eq!(map.len(), $l);
                assert_eq!(map.iter().len(), $l);
                assert_eq!(map.iter_mut().len(), $l);
            }
        }

        assert_len!(0);

        map.insert(Square::A1, Piece::WhitePawn);
        assert_len!(1);

        map = PieceMap::from_init(|_| Some(Piece::BlackBishop));
        assert_len!(64);
    }
}
