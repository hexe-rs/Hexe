//! Miscellaneous traits and types.

/// A type whose instance may be contained in some value.
pub trait Contained<T> {
    /// Returns whether `self` is contained in `other`.
    fn contained_in(self, other: T) -> bool;
}

/// A type whose instances may be used to extract references from buffers.
///
/// All operations are non-panicking and cannot fail.
pub trait Extract<T: ?Sized> {
    /// The output type.
    type Out: ?Sized;

    /// Extracts a reference to the value for `self` within `buf`.
    fn extract<'a>(self, buf: &'a T) -> &'a Self::Out;

    /// Extracts a mutable reference to the value for `self` within `buf`.
    fn extract_mut<'a>(self, buf: &'a mut T) -> &'a mut Self::Out;
}
