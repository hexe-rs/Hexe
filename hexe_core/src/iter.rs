//! Iterators over types.

use core::ops::Range;

use misc::Contained;
use uncon::*;

macro_rules! impl_iterable {
    ($t:ty, $raw:ty, $max:expr) => {
        impl AllIterable for $t {
            #[doc(hidden)]
            type __Iter = Range<$raw>;

            const ALL: All<Self> = All { iter: 0..($max as $raw) };

            #[inline]
            #[doc(hidden)]
            fn __next(iter: &mut Self::__Iter) -> Option<Self> {
                iter.next().map(|n| unsafe { n.into_unchecked() })
            }

            #[inline]
            #[doc(hidden)]
            fn __next_back(iter: &mut Self::__Iter) -> Option<Self> {
                iter.next_back().map(|n| unsafe { n.into_unchecked() })
            }

            #[inline]
            #[doc(hidden)]
            fn __len(iter: &Self::__Iter) -> usize {
                iter.len()
            }

            #[inline]
            #[doc(hidden)]
            fn __range(iter: &Self::__Iter) -> Range<usize> {
                Range { start: iter.start as usize, end: iter.end as usize }
            }
        }

        impl<'a> Contained<&'a All<$t>> for $t {
            #[inline]
            fn contained_in(self, all: &'a All<Self>) -> bool {
                let value = self as $raw;
                (all.iter.start <= value) && (value < all.iter.end)
            }
        }
    }
}

/// A type whose instances can be iterated over via `hexe_core::iter::All`.
pub trait AllIterable: Sized {
    #[doc(hidden)]
    type __Iter: Sized;

    /// An iterator over all instances of this type.
    const ALL: All<Self>;

    #[doc(hidden)]
    fn __next(_: &mut Self::__Iter) -> Option<Self>;

    #[doc(hidden)]
    fn __next_back(_: &mut Self::__Iter) -> Option<Self>;

    #[doc(hidden)]
    fn __len(_: &Self::__Iter) -> usize;

    #[doc(hidden)]
    fn __range(_: &Self::__Iter) -> Range<usize>;
}

impl_iterable!(::castle::Side,     u8, 2);
impl_iterable!(::castle::Right,    u8, 4);
impl_iterable!(::color::Color,     u8, 2);
impl_iterable!(::piece::Piece,     u8, 12);
impl_iterable!(::piece::PieceKind, u8, 6);
impl_iterable!(::piece::Promotion, u8, 4);
impl_iterable!(::square::File,     u8, 8);
impl_iterable!(::square::Rank,     u8, 8);
impl_iterable!(::square::Square,   u8, 64);

/// An iterator over all instances of `T`.
#[derive(Clone, PartialEq, Eq)]
pub struct All<T: AllIterable> {
    pub(crate) iter: T::__Iter,
}

impl<T: AllIterable> Default for All<T> {
    #[inline]
    fn default() -> Self { T::ALL }
}

impl<T: AllIterable> Iterator for All<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> { T::__next(&mut self.iter) }

    #[inline]
    fn last(mut self) -> Option<T> { self.next_back() }

    #[inline]
    fn count(self) -> usize { self.len() }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: AllIterable> DoubleEndedIterator for All<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> { T::__next_back(&mut self.iter) }
}

impl<T: AllIterable> ExactSizeIterator for All<T> {
    #[inline]
    fn len(&self) -> usize { T::__len(&self.iter) }
}

impl<T: AllIterable> All<T> {
    /// Returns whether `self` contains `item`.
    #[inline]
    pub fn contains<'a>(&'a self, item: T) -> bool
        where T: Contained<&'a Self>
    {
        item.contained_in(self)
    }

    /// Returns the range over which `self` iterates.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        T::__range(&self.iter)
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl All<::square::Square> {
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
