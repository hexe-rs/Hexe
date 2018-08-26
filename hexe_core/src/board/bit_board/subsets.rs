use super::*;
use core::{fmt, u64};

/// An iterator over all subsets of a [`BitBoard`].
///
/// This implementation uses the [Carry-Rippler algorithm][impl].
///
/// # Examples
///
/// ```
/// # use hexe_core::prelude::*;
/// let bit_board = BitBoard::FULL;
///
/// for subset in bit_board.subsets() {
///     # break
///     /* ... */
/// }
/// ```
///
/// [impl]: https://chessprogramming.wikispaces.com/Traversing+Subsets+of+a+Set
/// [`BitBoard`]: struct.BitBoard.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Subsets {
    /// The current subset
    current: u64,
    /// The initial superset
    initial: u64,
    /// Whether or not this is the first iteration
    is_first: bool,
}

impl From<BitBoard> for Subsets {
    #[inline]
    fn from(bit_board: BitBoard) -> Subsets {
        Subsets { current: 0, initial: bit_board.0, is_first: true }
    }
}

impl Default for Subsets {
    #[inline]
    fn default() -> Subsets {
        BitBoard::FULL.into()
    }
}

impl Subsets {
    /// The initial superset from which `self` was constructed.
    #[inline]
    pub fn initial(&self) -> BitBoard {
        self.initial.into()
    }

    /// The current subset. This value will be returned by
    /// [`next`](#method.next) if all subsets have not yet been returned.
    #[inline]
    pub fn current(&self) -> BitBoard {
        self.current.into()
    }

    /// Returns whether `self` has exhausted all subsets.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.is_first && self.current == 0
    }

    /// Returns whether a call to [`next`](#method.next) will return the first
    /// subset.
    #[inline]
    pub fn is_first(&self) -> bool {
        self.is_first
    }
}

impl Iterator for Subsets {
    type Item = BitBoard;

    #[inline]
    fn next(&mut self) -> Option<BitBoard> {
        if self.is_empty() { None } else {
            self.is_first = false;
            let current = self.current;
            self.current = self.initial & self.current.wrapping_sub(self.initial);
            Some(current.into())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_first {
            let len = match self.initial {
                u64::MAX => self.initial,
                superset => 1 << superset.count_ones(),
            };
            let lower = len as usize;
            let upper = if lower as u64 == len {
                Some(lower)
            } else {
                None
            };
            (lower, upper)
        } else if self.current == 0 {
            (0, Some(0))
        } else {
            (0, None)
        }
    }

    #[inline]
    fn last(self) -> Option<BitBoard> {
        if self.is_empty() { None } else {
            // The last result is always the initial subset
            Some(self.initial.into())
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
        let mut iter = BitBoard(superset).subsets();
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
