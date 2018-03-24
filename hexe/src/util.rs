use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};

/// A type like Cow with mutability and without the `Clone` restriction.
///
/// Whether the value is owned or mutably borrowed, the reference is stored at
/// the same memory offset in both variants. This is because `&mut T` and
/// `Box<T>` have the same memory layout.
pub enum MutRef<'a, T: ?Sized + 'a> {
    Borrowed(&'a mut T),
    Owned(Box<T>),
}

impl<'a, T> Deref for MutRef<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        match *self {
            MutRef::Borrowed(ref x) => x,
            MutRef::Owned(ref x)    => x,
        }
    }
}

impl<'a, T> DerefMut for MutRef<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        match *self {
            MutRef::Borrowed(ref mut x) => x,
            MutRef::Owned(ref mut x)    => x,
        }
    }
}

impl<'a, T> AsRef<T> for MutRef<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T { self }
}

impl<'a, T> AsMut<T> for MutRef<'a, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T { self }
}

#[inline]
pub unsafe fn zero<T: ?Sized>(val: &mut T) {
    let len = mem::size_of_val(val);
    let ptr = val as *mut T as *mut u8;
    ptr::write_bytes(ptr, 0, len);
}
