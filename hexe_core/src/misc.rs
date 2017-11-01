//! Miscellaneous traits and types.

/// A type whose instance may be contained in some value.
pub trait Contained<T: ?Sized> {
    /// Returns whether `self` is contained in `other`.
    fn contained_in(self, other: &T) -> bool;
}
