use core::usize;
use consts::*;

#[cfg(feature = "simd")]
use _simd::u8x16;

/// A type that represents multiple bytes.
///
/// Method implementations are taken from the `bytecount` crate.
trait Bytes {
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

/// A type that can efficiently return the count of a given value within itself.
pub trait Count<T> {
    /// The number of occurrences of `value` within `self`.
    fn count_of(self, value: T) -> usize;
}

macro_rules! impl_count {
    ($($N:expr)+) => { $(
        impl<'a> Count<u8> for &'a [u8; $N] {
            #[inline]
            fn count_of(self, needle: u8) -> usize {
                #[cfg(feature = "simd")]
                type B = u8x16;

                #[cfg(feature = "simd")]
                let chunks = (0..($N / 16)).map(|i| u8x16::load(self, i * 16));

                #[cfg(not(feature = "simd"))]
                type B = usize;

                #[cfg(not(feature = "simd"))]
                let chunks: &[usize; $N / PTR_SIZE] = unsafe {
                    use uncon::*;
                    self.into_unchecked()
                };

                let splat = B::splat(needle);

                chunks.into_iter().fold(B::splat(0), |sums, chunk| {
                    sums.increment(chunk.bytes_equal(splat))
                }).sum()
            }
        }
    )+ }
}

impl_count! { 64 }
