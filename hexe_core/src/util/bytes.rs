use core::{mem, ops, u64};

/// A type that represents a sequence of multiple bytes.
///
/// Method implementations are taken from the `bytecount` crate.
pub trait Bytes: Sized {
    /// Duplicates the byte across all bytes.
    fn splat(byte: u8) -> Self;

    /// Performs a byte-wise equality check against `other` and stores the
    /// individual results within each byte.
    fn bytes_eq(self, other: Self) -> Self;

    /// Increments each byte within `self`.
    fn increment(self, incr: Self) -> Self;

    /// Returns the sum of all bytes within `self`.
    fn sum(self) -> usize;

    /// Returns whether `self` contains a byte that equals zero.
    fn contains_zero_byte(self) -> bool;

    /// Returns whether `self` contains the byte.
    #[inline]
    fn contains_byte(self, byte: u8) -> bool
        where Self: ops::BitXor, Self::Output: Bytes
    {
        (self ^ Self::splat(byte)).contains_zero_byte()
    }
}

const LO: u64 = u64::MAX / 0xFF;
const HI: u64 = LO << 7;

macro_rules! impl_bytes {
    ($($t:ty),+) => { $(
        impl Bytes for $t {
            #[inline]
            fn splat(byte: u8) -> Self {
                LO as Self * byte as Self
            }

            #[inline]
            fn bytes_eq(self, other: Self) -> Self {
                const H: $t = HI as $t;
                const L: $t = LO as $t;

                let x = self ^ other;
                !(((x & !H).wrapping_add(!H) | x) >> 7) & L
            }

            #[inline]
            fn increment(self, incr: Self) -> Self {
                self + incr
            }

            #[inline]
            fn sum(self) -> usize {
                const EVERY_OTHER_LO: $t = u64::MAX as $t / 0xFFFF;
                const EVERY_OTHER: $t = EVERY_OTHER_LO * 0xFF;

                // Pairwise reduction to avoid overflow on next step
                let pair = (self & EVERY_OTHER) + ((self >> 8) & EVERY_OTHER);

                // Multiplication results in top two bytes holding sum
                let size = mem::size_of::<$t>();
                (pair.wrapping_mul(EVERY_OTHER_LO) >> ((size - 2) * 8)) as usize
            }

            // From Matters Computational by J. Arndt (1.20)
            //
            // "The idea is to subtract one from each of the bytes and then look
            // for bytes where the borrow propagated all the way to the most
            // significant bit."
            #[inline]
            fn contains_zero_byte(self) -> bool {
                self.wrapping_sub(LO as Self) & !self & HI as Self != 0
            }
        }
    )+ }
}

impl_bytes! { usize, u64, u32 }
