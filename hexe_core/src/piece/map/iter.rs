use super::*;
use square::Squares;
use core::mem;

macro_rules! iter {
    ($next:ident) => {
        #[inline]
        fn $next(&mut self) -> Option<Self::Item> {
            while let Some(sq) = self.iter.$next() {
                if let Some(pc) = self.map.get(sq) {
                    return Some((sq, pc));
                }
            }
            None
        }
    }
}

macro_rules! iter_mut {
    ($next:ident) => {
        #[inline]
        fn $next(&mut self) -> Option<Self::Item> {
            while let Some(sq) = self.iter.$next() {
                if let Some(pc) = self.map.get_mut(sq) {
                    // Extend the lifetime
                    let pc = unsafe { mem::transmute(pc) };
                    return Some((sq, pc));
                }
            }
            None
        }
    }
}

impl PieceMap {
    #[inline]
    fn find_len(&self, iter: &Squares) -> usize {
        iter.extract(&self.0).iter().fold(iter.len(), |len, &pc| {
            len - (pc == NONE) as usize
        })
    }
}

impl<'a> IntoIterator for &'a PieceMap {
    type Item = (Square, &'a Piece);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        Iter { map: self, iter: Squares::default() }
    }
}

impl<'a> IntoIterator for &'a mut PieceMap {
    type Item = (Square, &'a mut Piece);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> IterMut<'a> {
        IterMut { map: self, iter: Squares::default() }
    }
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

    iter! { next }

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
    iter! { next_back }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.map.find_len(&self.iter)
    }
}

impl<'a> fmt::Debug for Iter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// A mutable [`PieceMap`](struct.PieceMap.html) iterator.
pub struct IterMut<'a> {
    map: &'a mut PieceMap,
    iter: Squares,
}

#[cfg(test)]
assert_impl!(iter_mut; IterMut<'static>, Send, Sync);

impl<'a> From<IterMut<'a>> for Iter<'a> {
    #[inline]
    fn from(iter: IterMut) -> Iter {
        Iter { map: iter.map, iter: iter.iter }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Square, &'a mut Piece);

    iter_mut! { next }

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
    iter_mut! { next_back }
}

impl<'a> ExactSizeIterator for IterMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.map.find_len(&self.iter)
    }
}

impl<'a> fmt::Debug for IterMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Iter { map: self.map, iter: self.iter.clone() }.fmt(f)
    }
}
