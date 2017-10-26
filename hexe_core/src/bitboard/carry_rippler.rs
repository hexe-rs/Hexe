use super::*;

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
pub struct CarryRippler {
    /// The current subset
    sub: u64,
    /// The initial set
    set: u64,
    /// Whether or not this is the first iteration
    is_first: bool,
}

impl CarryRippler {
    /// Creates an instance for `bitboard`.
    #[inline]
    pub fn new(bitboard: Bitboard) -> CarryRippler {
        CarryRippler { sub: 0, set: bitboard.0, is_first: true }
    }
}

impl Iterator for CarryRippler {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Bitboard> {
        if self.is_first || self.sub != 0 {
            self.is_first = false;
            let sub = self.sub;
            self.sub = self.set & self.sub.wrapping_sub(self.set);
            Some(sub.into())
        } else {
            None
        }
    }

    #[inline]
    fn last(self) -> Option<Bitboard> {
        if self.is_first || self.sub != 0 {
            // The last result is always the initial set
            Some(self.set.into())
        } else {
            None
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

        for (a, b) in iter.by_ref().zip(SUBSETS.iter()) {
            assert_eq!(a, b.into());
        }

        assert_eq!(iter.next(), None);
    }
}
