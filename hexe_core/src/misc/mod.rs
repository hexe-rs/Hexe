//! Miscellaneous traits and types.

mod dir;
pub use self::dir::Direction;

mod extract;
pub use self::extract::Extract;

/// A type whose instance may be contained in some value.
pub trait Contained<T> {
    /// Returns whether `self` is contained in `other`.
    fn contained_in(self, other: T) -> bool;
}
