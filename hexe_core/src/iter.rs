//! Iterators over types.

use core::ops;

use misc::Contained;
use uncon::*;

mod private {
    use super::*;

    pub trait Iterable: Sized {
        type Iter: Sized;

        fn next(_: &mut Self::Iter) -> Option<Self>;

        fn next_back(_: &mut Self::Iter) -> Option<Self>;

        fn len(_: &Self::Iter) -> usize;

        fn indices(_: &Self::Iter) -> ops::Range<usize>;
    }
}

use self::private::Iterable;

macro_rules! impl_iterable {
    ($t:ty, $raw:ty, $max:expr) => {
        impl Iterable for $t {
            type Iter = ops::Range<$raw>;

            #[inline]
            fn next(iter: &mut Self::Iter) -> Option<Self> {
                iter.next().map(|n| unsafe { n.into_unchecked() })
            }

            #[inline]
            fn next_back(iter: &mut Self::Iter) -> Option<Self> {
                iter.next_back().map(|n| unsafe { n.into_unchecked() })
            }

            #[inline]
            fn len(iter: &Self::Iter) -> usize {
                iter.len()
            }

            #[inline]
            fn indices(iter: &Self::Iter) -> ops::Range<usize> {
                let start = iter.start as usize;
                let end   = iter.end   as usize;
                ops::Range { start, end }
            }
        }

        impl AllIterable for $t {
            const ALL: Range<Self> = Range { iter: 0..($max as $raw) };
        }

        impl<'a> Contained<&'a Range<$t>> for $t {
            #[inline]
            fn contained_in(self, all: &'a Range<Self>) -> bool {
                let value = self as $raw;
                (all.iter.start <= value) && (value < all.iter.end)
            }
        }

        impl<T> ::misc::Extract<[T; $max]> for $t {
            type Output = T;

            #[inline]
            fn extract<'a>(self, buf: &'a [T; $max]) -> &'a T {
                &buf[self as usize]
            }

            #[inline]
            fn extract_mut<'a>(self, buf: &'a mut [T; $max]) -> &'a mut T {
                &mut buf[self as usize]
            }
        }

        impl<'r, T> ::misc::Extract<[T; $max]> for &'r Range<$t> {
            type Output = [T];

            #[inline]
            fn extract<'a>(self, buf: &'a [T; $max]) -> &'a [T] {
                unsafe { buf.get_unchecked(self.indices()) }
            }

            #[inline]
            fn extract_mut<'a>(self, buf: &'a mut [T; $max]) -> &'a mut [T] {
                unsafe { buf.get_unchecked_mut(self.indices()) }
            }
        }
    }
}

/// A type whose instances can all be efficiently iterated over via
/// [`Range`](struct.Range.html).
pub trait AllIterable: Iterable {
    /// An iterator over all instances of this type.
    const ALL: Range<Self>;
}

impl_iterable!(::castle::Side,     u8, 2);
impl_iterable!(::castle::Right,    u8, 4);
impl_iterable!(::color::Color,     u8, 2);
impl_iterable!(::piece::Piece,     u8, 12);
impl_iterable!(::piece::Role,      u8, 6);
impl_iterable!(::piece::Promotion, u8, 4);
impl_iterable!(::square::File,     u8, 8);
impl_iterable!(::square::Rank,     u8, 8);
impl_iterable!(::square::Square,   u8, 64);

/// An efficient iterator over instances of `T`.
///
/// Unlike the standard library's [`Range`], the start and end values are
/// guaranteed to _always_ be in order.
///
/// [`Range`]: https://doc.rust-lang.org/std/ops/struct.Range.html
#[derive(Clone, PartialEq, Eq)]
pub struct Range<T: Iterable> {
    pub(crate) iter: T::Iter,
}

impl<T: AllIterable> Default for Range<T> {
    #[inline]
    fn default() -> Self { T::ALL }
}

impl<T: Iterable> Iterator for Range<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> { T::next(&mut self.iter) }

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

impl<T: Iterable> DoubleEndedIterator for Range<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> { T::next_back(&mut self.iter) }
}

impl<T: Iterable> ExactSizeIterator for Range<T> {
    #[inline]
    fn len(&self) -> usize { T::len(&self.iter) }
}

impl<T: Iterable> Range<T> {
    /// Returns whether `self` contains `item`.
    #[inline]
    pub fn contains<'a>(&'a self, item: T) -> bool
        where T: Contained<&'a Self>
    {
        item.contained_in(self)
    }

    /// Returns the range of indices over which `self` iterates.
    #[inline]
    pub fn indices(&self) -> ops::Range<usize> {
        T::indices(&self.iter)
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
