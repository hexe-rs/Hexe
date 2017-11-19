use super::*;

/// An iterator over all squares.
#[derive(Clone, PartialEq, Eq)]
pub struct Squares {
    // Range for iterating in reverse
    // Invariant: always within 0..64
    iter: Range<u8>
}

impl Iterator for Squares {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        use uncon::IntoUnchecked;
        if let Some(n) = self.iter.next() {
            unsafe { Some(n.into_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn last(mut self) -> Option<Square> {
        self.next_back()
    }
}

impl DoubleEndedIterator for Squares {
    #[inline]
    fn next_back(&mut self) -> Option<Square> {
        use uncon::IntoUnchecked;
        if let Some(n) = self.iter.next_back() {
            unsafe { Some(n.into_unchecked()) }
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Squares {
    #[inline]
    fn len(&self) -> usize {
        self.iter.end as usize - self.iter.start as usize
    }
}

impl Default for Squares {
    #[inline]
    fn default() -> Self {
        Squares { iter: 0..64 }
    }
}

impl Squares {
    /// Returns whether `self` contains `square`.
    #[inline]
    pub fn contains(&self, square: Square) -> bool {
        let value = square as u8;
        (self.iter.start <= value) && (value < self.iter.end)
    }

    /// Returns the range over which `self` iterates.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        Range { start: self.iter.start as usize, end: self.iter.end as usize }
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extracts a slice from the buffer over which `self` iterates.
    #[inline]
    pub fn extract<'a, T: 'a>(&self, buf: &'a [T; 64]) -> &'a [T] {
        unsafe { buf.get_unchecked(self.range()) }
    }

    /// Extracts a mutable slice from the buffer over which `self` iterates.
    #[inline]
    pub fn extract_mut<'a, T: 'a>(&self, buf: &'a mut [T; 64]) -> &'a mut [T] {
        unsafe { buf.get_unchecked_mut(self.range()) }
    }
}
