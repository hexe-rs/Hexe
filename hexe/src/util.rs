use std::mem;
use std::intrinsics::write_bytes;

#[inline]
pub unsafe fn zero<T: ?Sized>(val: &mut T) {
    let len = mem::size_of_val(val);
    let ptr = val as *mut T as *mut u8;
    write_bytes(ptr, 0, len);
}
