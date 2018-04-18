//! Non-public utility traits.

use core::mem;
use core::ptr;

mod bytes;
mod count;

pub use self::bytes::*;
pub use self::count::*;

pub type Usize64 = bytes64!(usize);

#[inline]
pub unsafe fn zero<T: ?Sized>(val: &mut T) {
    let len = mem::size_of_val(val);
    let ptr = val as *mut T as *mut u8;
    ptr::write_bytes(ptr, 0, len);
}

#[cfg(any(test, feature = "rand"))]
pub fn rand_pairs<T, U>() -> [(T, U); 1000]
    where T: ::rand::Rand,
          U: ::rand::Rand,
{
    let mut pairs: [(T, U); 1000] = unsafe { mem::uninitialized() };
    for &mut (ref mut a, ref mut b) in pairs.iter_mut() {
        unsafe {
            ptr::write(a, ::rand::random());
            ptr::write(b, ::rand::random());
        }
    }
    pairs
}
