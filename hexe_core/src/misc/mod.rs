//! Miscellaneous traits and types.

mod dir;
pub use self::dir::Direction;

/// A type whose instance may be contained in some value.
pub trait Contained<T> {
    /// Returns whether `self` is contained in `other`.
    fn contained_in(self, other: T) -> bool;
}

/// A type whose instances may be used to extract references from buffers.
///
/// All operations are non-panicking and cannot fail.
///
/// # Examples
///
/// Using tuples preserves the same order as normal indexing:
///
/// ```
/// use hexe_core::prelude::{Extract, Square};
///
/// let mut table: [[u8; 64]; 64] = [
///     /* ... */
/// # [0; 64]; 64
/// ];
///
/// let s1 = Square::B5;
/// let s2 = Square::C8;
///
/// *(s1, s2).extract_mut(&mut table) = 20;
///
/// assert_eq!(table[s1 as usize][s2 as usize], 20);
/// ```
pub trait Extract<T: ?Sized> {
    /// The output type.
    type Output: ?Sized;

    /// Extracts a reference to the value for `self` within `buf`.
    fn extract(self, buf: &T) -> &Self::Output;

    /// Extracts a mutable reference to the value for `self` within `buf`.
    fn extract_mut(self, buf: &mut T) -> &mut Self::Output;
}

impl<T, A: 'static> Extract<T> for (A, A)
    where
        A: Extract<T>,
        A: Extract<<A as Extract<T>>::Output>,
{
    type Output = <A as Extract<<A as Extract<T>>::Output>>::Output;

    #[inline]
    fn extract(self, table: &T) -> &Self::Output {
        self.1.extract(self.0.extract(table))
    }

    #[inline]
    fn extract_mut(self, table: &mut T) -> &mut Self::Output {
        self.1.extract_mut(self.0.extract_mut(table))
    }
}
