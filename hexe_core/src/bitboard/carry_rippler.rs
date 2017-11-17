use super::*;
use core::fmt;

/// An iterator over all subsets of a [`Bitboard`].
///
/// The original implementation can be found [here][impl].
///
/// # Examples
///
/// ```
/// # use hexe_core::prelude::*;
/// let bitboard = Bitboard::FULL;
///
/// for subset in bitboard.carry_rippler() {
///     # break
///     /* ... */
/// }
/// ```
///
/// [impl]: https://chessprogramming.wikispaces.com/Traversing+Subsets+of+a+Set
/// [`Bitboard`]: struct.Bitboard.html
#[derive(Copy, Clone)]
pub struct CarryRippler {
    /// The current subset
    sub: u64,
    /// The initial set
    set: u64,
    /// Whether or not this is the first iteration
    is_first: bool,
}

impl fmt::Debug for CarryRippler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CarryRippler")
            .field("superset", &self.superset())
            .field("subset",   &self.subset())
            .finish()
    }
}

impl Default for CarryRippler {
    #[inline]
    fn default() -> CarryRippler {
        CarryRippler::new(Bitboard::FULL)
    }
}

impl CarryRippler {
    /// Creates an instance for `bitboard`.
    #[inline]
    pub fn new(bitboard: Bitboard) -> CarryRippler {
        CarryRippler { sub: 0, set: bitboard.0, is_first: true }
    }

    /// The initial superset from which `self` was constructed.
    #[inline]
    pub fn superset(&self) -> Bitboard {
        self.set.into()
    }

    /// The current subset. This value will be returned next if `self` is not
    /// yet finished.
    #[inline]
    pub fn subset(&self) -> Bitboard {
        self.sub.into()
    }

    /// Returns whether `self` is finished iterating.
    #[inline]
    pub fn is_finished(&self) -> bool {
        !self.is_first && self.sub == 0
    }
}

impl Iterator for CarryRippler {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Bitboard> {
        if self.is_finished() { None } else {
            self.is_first = false;
            let sub = self.sub;
            self.sub = self.set & self.sub.wrapping_sub(self.set);
            Some(sub.into())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_first {
            const FULL: u64 = !0;
            let len = if self.set == FULL {
                FULL
            } else {
                1 << self.set.count_ones()
            };
            let lower = len as usize;
            let upper = if lower as u64 == len {
                Some(lower)
            } else {
                None
            };
            (lower, upper)
        } else if self.sub == 0 {
            (0, Some(0))
        } else {
            (0, None)
        }
    }

    #[inline]
    fn last(self) -> Option<Bitboard> {
        if self.is_finished() { None } else {
            // The last result is always the initial set
            Some(self.set.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        static SUBSETS: &[u64] = &[
            0b00000,
            0b00010,
            0b00100,
            0b00110,
            0b10000,
            0b10010,
            0b10100,
            0b10110,
        ];

        let superset = *SUBSETS.last().unwrap();
        let mut iter = Bitboard(superset).carry_rippler();
        let mut sum  = 0usize;

        assert_eq!(iter.size_hint().0, SUBSETS.len());

        for (a, &b) in iter.by_ref().zip(SUBSETS.iter()) {
            sum += 1;
            assert_eq!(a, b.into());
        }

        assert_eq!(sum, SUBSETS.len());
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }
}
