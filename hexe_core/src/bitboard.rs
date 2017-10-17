//! A bitmap chess board representation.

use core::fmt;
use core::ops;

use color::Color;
use square::{Square, File, Rank};

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

macro_rules! forward_bit_ops_impl {
    ($($t1:ident $f1:ident $t2:ident $f2:ident)+) => {
        $(impl<T: Into<Bitboard>> ops::$t1<T> for Bitboard {
            type Output = Self;

            #[inline]
            fn $f1(self, other: T) -> Self {
                Bitboard((self.0).$f1(other.into().0))
            }
        }

        impl<T: Into<Bitboard>> ops::$t2<T> for Bitboard {
            #[inline]
            fn $f2(&mut self, other: T) {
                (self.0).$f2(other.into().0)
            }
        })+
    }
}

forward_bit_ops_impl! {
    BitAnd bitand BitAndAssign bitand_assign
    BitXor bitxor BitXorAssign bitxor_assign
    BitOr  bitor  BitOrAssign  bitor_assign
}

impl<T: Into<Bitboard>> ops::Sub<T> for Bitboard {
    type Output = Self;

    #[inline]
    fn sub(self, other: T) -> Self { Bitboard(self.0 & !other.into().0) }
}

impl<T: Into<Bitboard>> ops::SubAssign<T> for Bitboard {
    #[inline]
    fn sub_assign(&mut self, other: T) { self.0 &= !other.into().0 }
}

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

impl Bitboard {
    /// White board squares.
    pub const WHITE: Bitboard = Bitboard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: Bitboard = Bitboard(0xAA55AA55AA55AA55);
}

/// Bitboard masks for each file and rank.
pub mod masks {
    use super::*;

    macro_rules! impl_consts {
        ($base:expr, $shift:expr; $cur:ident, $($next:ident),+ $(,)*) => {
            pub const $cur: Bitboard = Bitboard($base);
            impl_consts!($shift; $cur, $($next),+);
        };
        ($shift:expr; $prev:ident, $cur:ident) => {
            pub const $cur: Bitboard = Bitboard($prev.0 << $shift);
        };
        ($shift:expr; $prev:ident, $cur:ident, $($next:ident),+) => {
            impl_consts!($shift; $prev, $cur);
            impl_consts!($shift; $cur, $($next),+);
        };
    }

    impl_consts! {
        0x0101010101010101, 1;
        FILE_A, FILE_B, FILE_C, FILE_D,
        FILE_E, FILE_F, FILE_G, FILE_H,
    }

    impl_consts! {
        0xFF, 8;
        RANK_1, RANK_2, RANK_3, RANK_4,
        RANK_5, RANK_6, RANK_7, RANK_8,
    }
}
