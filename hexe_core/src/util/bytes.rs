use consts::*;
use core::usize;

#[cfg(feature = "simd")]
use simd::u8x16;

/// A type that represents a sequence of multiple bytes.
///
/// Method implementations are taken from the `bytecount` crate.
pub trait Bytes {
    /// Duplicates the byte across all bytes.
    fn splat(byte: u8) -> Self;

    /// Performs a byte-wise equality check against `other` and stores the
    /// individual results within each byte.
    fn bytes_equal(self, other: Self) -> Self;

    /// Increments each byte within `self`.
    fn increment(self, incr: Self) -> Self;

    /// Returns the sum of all bytes within `self`.
    fn sum(self) -> usize;
}

const LO: usize = usize::MAX / 0xFF;
const HI: usize = LO << 7;

impl Bytes for usize {
    #[inline]
    fn splat(byte: u8) -> usize {
        LO * byte as usize
    }

    #[inline]
    fn bytes_equal(self, other: usize) -> usize {
        let x = self ^ other;
        !((((x & !HI) + !HI) | x) >> 7) & LO
    }

    #[inline]
    fn increment(self, incr: usize) -> usize {
        self + incr
    }

    #[inline]
    fn sum(self) -> usize {
        const EVERY_OTHER_LO: usize = usize::MAX / 0xFFFF;
        const EVERY_OTHER: usize = EVERY_OTHER_LO * 0xFF;

        // Pairwise reduction to avoid overflow on next step
        let pair_sum = (self & EVERY_OTHER) + ((self >> 8) & EVERY_OTHER);

        // Multiplication results in top two bytes holding sum
        pair_sum.wrapping_mul(EVERY_OTHER_LO) >> ((PTR_SIZE - 2) * 8)
    }
}

#[cfg(feature = "simd")]
impl Bytes for u8x16 {
    #[inline]
    fn splat(byte: u8) -> Self {
        Self::splat(byte)
    }

    #[inline]
    fn bytes_equal(self, other: Self) -> Self {
        self.eq(other).to_repr().to_u8()
    }

    #[inline]
    fn increment(self, incr: Self) -> Self {
        // incr on -1
        self - incr
    }

    #[inline]
    fn sum(self) -> usize {
        (0..16).fold(0, |s, i| s + self.extract(i) as usize)
    }
}
