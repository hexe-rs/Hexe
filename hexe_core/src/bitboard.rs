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

macro_rules! forward_sh_impl {
    ($($t1:ident $f1:ident $t2:ident $f2:ident)+) => {
        $(impl<T> ops::$t1<T> for Bitboard where u64: ops::$t1<T, Output=u64> {
            type Output = Self;

            #[inline]
            fn $f1(self, shift: T) -> Self { Bitboard((self.0).$f1(shift)) }
        }

        impl<T> ops::$t2<T> for Bitboard where u64: ops::$t2<T> {
            #[inline]
            fn $f2(&mut self, shift: T) { (self.0).$f2(shift) }
        })+
    }
}

forward_sh_impl! {
    Shl shl ShlAssign shl_assign
    Shr shr ShrAssign shr_assign
}

impl From<u64> for Bitboard {
    #[inline(always)]
    fn from(bits: u64) -> Self { Bitboard(bits) }
}

impl From<Bitboard> for u64 {
    #[inline(always)]
    fn from(bb: Bitboard) -> Self { bb.0 }
}

impl AsRef<u64> for Bitboard {
    #[inline(always)]
    fn as_ref(&self) -> &u64 { &self.0 }
}

impl AsMut<u64> for Bitboard {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut u64 { &mut self.0 }
}
