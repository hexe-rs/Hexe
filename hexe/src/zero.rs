use std::cell::UnsafeCell;
use std::mem;
use std::ops;
use std::ptr::{self, NonNull};
use std::slice;

use libc;

/// A type whose instances can safely be all zeroes.
pub unsafe trait Zero {
    /// Safely zeroes out `self`.
    #[inline]
    fn zero(&mut self) {
        unsafe { ::util::zero(self) };
    }
}

macro_rules! impl_zero {
    ($($t:ty)+) => { $(
        unsafe impl Zero for $t {}
    )+ }
}

impl_zero! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}

unsafe impl<T: Zero> Zero for [T] {}

unsafe impl<T: Zero> Zero for UnsafeCell<T> {}

/// A buffer that, when allocated, starts as all zeroes.
pub struct ZeroBuffer<T: Zero> {
    /// The start of the `calloc`ed buffer.
    start: *mut libc::c_void,
    /// A pointer offset to the correct alignment of `T`.
    align: NonNull<T>,
    /// The size of the buffer by number of `T`.
    len: usize,
}

unsafe impl<T: Send + Zero> Send for ZeroBuffer<T> {}
unsafe impl<T: Sync + Zero> Sync for ZeroBuffer<T> {}

impl<T: Zero> Default for ZeroBuffer<T> {
    #[inline]
    fn default() -> ZeroBuffer<T> {
        ZeroBuffer {
            start: ptr::null_mut(),
            align: NonNull::dangling(),
            len: 0,
        }
    }
}

impl<T: Zero> Drop for ZeroBuffer<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.dealloc() };
    }
}

impl<T: Zero> ops::Deref for ZeroBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts(ptr, self.len) }
    }
}

impl<T: Zero> ops::DerefMut for ZeroBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts_mut(ptr, self.len) }
    }
}

impl<T: Zero> AsRef<[T]> for ZeroBuffer<T> {
    #[inline]
    fn as_ref(&self) -> &[T] { self }
}

impl<T: Zero> AsMut<[T]> for ZeroBuffer<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] { self }
}

impl<T: Zero> ZeroBuffer<T> {
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
