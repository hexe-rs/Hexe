use std::mem;
use std::ops;
use std::ptr::{self, NonNull};
use std::slice;

use libc;

const LOWER_BIT: u8 = 32;

/// A wrapper that can be sent across thread boundaries.
///
/// This is _very unsafe_ to use since it allows any type to be Send, bypassing
/// Rust's built-in thread safety.
pub struct AnySend<T>(T);

unsafe impl<T> Send for AnySend<T> {}

impl<T> AnySend<T> {
    #[inline]
    pub fn new(val: T) -> Self { AnySend(val) }

    #[inline]
    pub unsafe fn get(self) -> T { self.0 }
}

/// A buffer that, when allocated, starts as all zeroes.
pub struct ZeroBuffer<T> {
    /// The start of the `calloc`ed buffer.
    start: *mut libc::c_void,
    /// A pointer offset to the correct alignment of `T`.
    align: NonNull<T>,
    /// The size of the buffer by number of `T`.
    len: usize,
}

unsafe impl<T: Send> Send for ZeroBuffer<T> {}
unsafe impl<T: Sync> Sync for ZeroBuffer<T> {}

impl<T> Default for ZeroBuffer<T> {
    #[inline]
    fn default() -> ZeroBuffer<T> {
        ZeroBuffer {
            start: ptr::null_mut(),
            align: NonNull::dangling(),
            len: 0,
        }
    }
}

impl<T> Drop for ZeroBuffer<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.dealloc() };
    }
}

impl<T> ops::Deref for ZeroBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts(ptr, self.len) }
    }
}

impl<T> ops::DerefMut for ZeroBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts_mut(ptr, self.len) }
    }
}

impl<T> AsRef<[T]> for ZeroBuffer<T> {
    #[inline]
    fn as_ref(&self) -> &[T] { self }
}

impl<T> AsMut<[T]> for ZeroBuffer<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] { self }
}

impl<T> ZeroBuffer<T> {
    #[inline]
    unsafe fn dealloc(&mut self) {
        if !self.start.is_null() {
            libc::free(self.start);
        }
    }

    #[cfg(test)]
    pub fn is_aligned(&self) -> bool {
        self.align.as_ptr() as usize % mem::align_of::<T>() == 0
    }

    #[inline]
    pub fn resize_exact(&mut self, len: usize) {
        if len == self.len {
            return;
        }

        let size  = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let mask  = !(align - 1);

        unsafe { self.dealloc() };

        let calloc = unsafe { libc::calloc(len + 1, size) };
        self.start = calloc;
        self.len   = len;

        self.align = unsafe {
            let val = calloc.offset(align as _) as usize;
            NonNull::new_unchecked((val & mask) as *mut T)
        };
    }
}

#[inline]
pub unsafe fn zero<T: ?Sized>(val: &mut T) {
    let len = mem::size_of_val(val);
    let ptr = val as *mut T as *mut u8;
    ptr::write_bytes(ptr, 0, len);
}

/// Performs a case-insensitive check against `input` assuming `check` is
/// encoded as an ASCII alphabetical lowercase string.
pub fn matches_lower_alpha(check: &[u8], input: &[u8]) -> bool {
    if check.len() != input.len() {
        return false;
    }
    for (&check, &input) in check.iter().zip(input.iter()) {
        // Sets the lowercase bit in the input byte
        if input | LOWER_BIT != check {
            return false;
        }
    }
    true
}
