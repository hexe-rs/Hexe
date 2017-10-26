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
    current:  Bitboard,
    bitboard: Bitboard,
    is_first: bool,
}

impl CarryRippler {
    /// Creates an instance for `bitboard`.
    #[inline]
    pub fn new(bitboard: Bitboard) -> CarryRippler {
        CarryRippler {
            current:  Bitboard::EMPTY,
            bitboard: bitboard,
            is_first: true,
        }
    }
}

impl Iterator for CarryRippler {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Bitboard> {
        if self.is_first || !self.current.is_empty() {
            self.is_first = false;
            let current  = self.current;
            self.current = self.bitboard
                         & self.current.0.wrapping_sub(self.bitboard.0);
            Some(current)
        } else {
            None
        }
    }
}
