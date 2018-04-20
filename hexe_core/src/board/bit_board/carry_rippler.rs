use super::*;
use core::{fmt, u64};

/// An iterator over all subsets of a [`BitBoard`].
///
/// A reference implementation can be found [here][impl].
///
/// # Examples
///
/// ```
/// # use hexe_core::prelude::*;
/// let bit_board = BitBoard::FULL;
///
/// for subset in bit_board.carry_rippler() {
///     # break
///     /* ... */
/// }
/// ```
///
/// [impl]: https://chessprogramming.wikispaces.com/Traversing+Subsets+of+a+Set
/// [`BitBoard`]: struct.BitBoard.html
#[derive(Copy, Clone, PartialEq, Eq)]
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
            .field("is_first", &self.is_first)
            .finish()
    }
}

impl From<BitBoard> for CarryRippler {
    #[inline]
    fn from(bit_board: BitBoard) -> CarryRippler {
        CarryRippler { sub: 0, set: bit_board.0, is_first: true }
    }
}

impl Default for CarryRippler {
    #[inline]
    fn default() -> CarryRippler {
        BitBoard::FULL.into()
    }
}

impl CarryRippler {
    /// The initial superset from which `self` was constructed.
    #[inline]
    pub fn superset(&self) -> BitBoard {
        self.set.into()
    }

    /// The current subset. This value will be returned next if `self` is not
    /// yet finished.
    #[inline]
    pub fn subset(&self) -> BitBoard {
        self.sub.into()
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.is_first && self.sub == 0
    }

    /// Returns whether a call to `next` will return the first value.
    #[inline]
    pub fn is_first(&self) -> bool {
        self.is_first
    }
}

impl Iterator for CarryRippler {
    type Item = BitBoard;

    #[inline]
    fn next(&mut self) -> Option<BitBoard> {
        if self.is_empty() { None } else {
            self.is_first = false;
            let sub = self.sub;
            self.sub = self.set & self.sub.wrapping_sub(self.set);
            Some(sub.into())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_first {
            let len = match self.set {
                u64::MAX => self.set,
                superset => 1 << superset.count_ones(),
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
    fn last(self) -> Option<BitBoard> {
        if self.is_empty() { None } else {
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
        let mut iter = BitBoard(superset).carry_rippler();
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
