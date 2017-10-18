use super::*;
use core::{fmt, iter, ops};
use prelude::*;

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

impl_set_ops! { Bitboard }

impl ops::Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self { Bitboard(!self.0) }
}

impl From<u64> for Bitboard {
    #[inline(always)]
    fn from(bits: u64) -> Self { Bitboard(bits) }
}

impl AsRef<u64> for Bitboard {
    #[inline(always)]
    fn as_ref(&self) -> &u64 { &self.0 }
}

impl AsMut<u64> for Bitboard {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut u64 { &mut self.0 }
}

impl From<Bitboard> for u64 {
    #[inline(always)]
    fn from(bb: Bitboard) -> Self { bb.0 }
}

impl AsRef<Bitboard> for u64 {
    #[inline(always)]
    fn as_ref(&self) -> &Bitboard {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsMut<Bitboard> for u64 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Bitboard {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl From<Square> for Bitboard {
    #[inline]
    fn from(square: Square) -> Self {
        Bitboard(1 << square as usize)
    }
}

impl From<File> for Bitboard {
    #[inline]
    fn from(file: File) -> Self {
        masks::FILE_A << file as usize
    }
}

impl From<Rank> for Bitboard {
    #[inline]
    fn from(rank: Rank) -> Self {
        masks::RANK_1 << ((rank as usize) << 3)
    }
}

impl From<Color> for Bitboard {
    #[inline]
    fn from(color: Color) -> Self {
        match color {
            Color::White => Bitboard::WHITE,
            Color::Black => Bitboard::BLACK,
        }
    }
}

impl<A: Into<Bitboard>> iter::FromIterator<A> for Bitboard {
    #[inline]
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        iter.into_iter().fold(Bitboard::EMPTY, ops::BitOr::bitor)
    }
}

impl<A: Into<Bitboard>> Extend<A> for Bitboard {
    #[inline]
    fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T) {
        *self |= iter.into_iter().collect::<Bitboard>();
    }
}
