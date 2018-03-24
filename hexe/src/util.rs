use std::mem;
use std::ptr;

/// A type like Cow with mutability and without the `Clone` restriction.
///
/// Whether the value is owned or mutably borrowed, the reference is stored at
/// the same memory offset in both variants. This is because `&mut T` and
/// `Box<T>` have the same memory layout.
pub enum MutRef<'a, T: ?Sized + 'a> {
    Borrowed(&'a mut T),
    Owned(Box<T>),
}

#[inline]
pub unsafe fn zero<T: ?Sized>(val: &mut T) {
    let len = mem::size_of_val(val);
    let ptr = val as *mut T as *mut u8;
    ptr::write_bytes(ptr, 0, len);
}
