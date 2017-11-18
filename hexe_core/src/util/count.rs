use super::bytes::Bytes;
use consts::*;

#[cfg(feature = "simd")]
use simd::u8x16;

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
