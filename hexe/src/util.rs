use std::mem;
use std::ptr;

const LOWER_BIT: u8 = 32;

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
