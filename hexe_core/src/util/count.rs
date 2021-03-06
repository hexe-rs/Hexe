#[cfg(feature = "simd")]
use packed_simd::{FromBits, u8x64};

/// A type that can efficiently return the count of a given value within itself.
pub trait Count<T> {
    /// The number of occurrences of `value` within `self`.
    fn count(self, value: T) -> usize;
}

#[cfg(feature = "simd")]
impl Count<u8> for u8x64 {
    #[inline]
    fn count(self, needle: u8) -> usize {
        let zero = u8x64::splat(0);
        let val  = u8x64::splat(needle);
        (zero - u8x64::from_bits(self.eq(val))).wrapping_sum() as usize
    }
}

impl<'a> Count<u8> for &'a [u8; 64] {
    #[inline]
    #[cfg(feature = "simd")]
    fn count(self, needle: u8) -> usize {
        u8x64::from_slice_unaligned(self).count(needle)
    }

    #[inline]
    #[cfg(not(feature = "simd"))]
    fn count(self, needle: u8) -> usize {
        use uncon::*;
        use util::bytes::Bytes;

        let chunks: &super::Usize64 = unsafe { self.into_unchecked() };
        let splat = usize::splat(needle);

        chunks.into_iter().fold(0usize, |sums, chunk| {
            sums.increment(chunk.bytes_eq(splat))
        }).sum()
    }
}
