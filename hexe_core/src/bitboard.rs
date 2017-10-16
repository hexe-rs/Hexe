//! A bitmap chess board representation.

use core::fmt;
use core::ops;

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

impl ops::Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self { Bitboard(!self.0) }
}

impl<T> ops::Shl<T> for Bitboard where u64: ops::Shl<T, Output=u64> {
    type Output = Self;

    #[inline]
    fn shl(self, shift: T) -> Self { Bitboard(self.0.shl(shift)) }
}

impl<T> ops::Shr<T> for Bitboard where u64: ops::Shr<T, Output=u64> {
    type Output = Self;

    #[inline]
    fn shr(self, shift: T) -> Self { Bitboard(self.0.shr(shift)) }
}

impl<T> ops::ShlAssign<T> for Bitboard where u64: ops::ShlAssign<T> {
    #[inline]
    fn shl_assign(&mut self, shift: T) { self.0.shl_assign(shift) }
}

impl<T> ops::ShrAssign<T> for Bitboard where u64: ops::ShrAssign<T> {
    #[inline]
    fn shr_assign(&mut self, shift: T) { self.0.shr_assign(shift) }
}
