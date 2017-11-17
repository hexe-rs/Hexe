//! Extension traits.

/// Bit twiddling operations.
pub trait Twiddling {
    /// Returns whether `self` contains a byte that equals zero.
    fn contains_zero_byte(self) -> bool;
}

macro_rules! impl_signed {
    ($($s:ty, $u:ty),+) => { $(
        impl Twiddling for $s {
            #[inline]
            fn contains_zero_byte(self) -> bool {
                (self as $u).contains_zero_byte()
            }
        }
    )+ }
}

impl_signed! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }

impl Twiddling for u8 {
    #[inline]
    fn contains_zero_byte(self) -> bool { self == 0 }
}

impl Twiddling for u16 {
    #[inline]
    fn contains_zero_byte(self) -> bool {
        (self >> 8 == 0) || (self & 0xFF == 0)
    }
}

/// From Matters Computational by J. Arndt (1.20)
///
/// "The idea is to subtract one from each of the bytes and then look for
/// bytes where the borrow propagated all the way to the most significant
/// bit."
macro_rules! contains_zero_byte {
    ($x:expr, $lo:expr, $hi:expr) => {
        $x.wrapping_sub($lo) & !$x & $hi != 0
    }
}

const LO: u64 = 0x0101010101010101;
const HI: u64 = 0x8080808080808080;

impl Twiddling for u32 {
    #[inline]
    fn contains_zero_byte(self) -> bool {
        contains_zero_byte!(self, LO as u32, HI as u32)
    }
}

impl Twiddling for u64 {
    #[inline]
    fn contains_zero_byte(self) -> bool {
        contains_zero_byte!(self, LO, HI)
    }
}

impl Twiddling for usize {
    #[inline]
    fn contains_zero_byte(self) -> bool {
        contains_zero_byte!(self, LO as usize, HI as usize)
    }
}
