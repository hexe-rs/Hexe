//! A bitmap chess board representation.

use core::fmt;

/// A mapping of sixty-four bits to squares of a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

macro_rules! forward_fmt_impl {
    ($($f:ident)+) => {
        $(impl fmt::$f for Bitboard {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::$f::fmt(&self.0, f)
            }
        })+
    }
}

forward_fmt_impl! { Binary Octal LowerHex UpperHex }

impl fmt::Debug for Bitboard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2 for "0x" + 16 for number
        write!(f, "Bitboard({:#018X})", self)
    }
}
