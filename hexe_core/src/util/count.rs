use core::usize;
use consts::*;
use uncon::*;

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

/// A type that can efficiently return the count of a given value within itself.
pub trait Count<T> {
    /// The number of occurrences of `value` within `self`.
    fn count_of(self, value: T) -> usize;
}

const SIXTY_FOUR: usize = 64;

impl<'a> Count<u8> for &'a [u8; SIXTY_FOUR] {
    #[inline]
    fn count_of(self, needle: u8) -> usize {
        let splat = usize::splat(needle);

        let chunks: &[usize; SIXTY_FOUR / PTR_SIZE] = unsafe {
            self.into_unchecked()
        };

        chunks.iter().fold(0, |sums, &chunk| {
            sums.increment(chunk.bytes_equal(splat))
        }).sum()
    }
}
