//! [SIMD](https://en.wikipedia.org/wiki/Single_Instruction_Multiple_Data)
//! parallelism.

use core::ops::BitOr;

#[cfg(feature = "simd")]
use core::simd::{u64x2, u64x4, u64x8};

use board::BitBoard;
use sealed::Sealed;
use square::Square;

/// The minimum level (1).
pub type LMin = L1;

/// The maximum level (8).
#[cfg(feature = "simd")]
pub type LMax = L8;

/// The maximum level (1).
#[cfg(not(feature = "simd"))]
pub type LMax = L1;

/// The level of parallelism to use in operations.
pub trait Level: Sealed {
    /// The `BitBoard` type.
    type BitBoard: Copy + BitOr<Output=Self::BitBoard>;

    /// The `Square` type.
    type Square: Copy;

    /// An integral value for the level used. This is always a power of two.
    const LEVEL: usize;

    /// Returns the bishop attacks for each square and each occupied board.
    fn bishop_attacks(sq: Self::Square, occupied: Self::BitBoard) -> Self::BitBoard;

    /// Returns the rook attacks for each square and each occupied board.
    fn rook_attacks(sq: Self::Square, occupied: Self::BitBoard) -> Self::BitBoard;

    /// Returns the queen attacks for each square and each occupied board.
    #[inline]
    fn queen_attacks(sq: Self::Square, occupied: Self::BitBoard) -> Self::BitBoard {
        Self::bishop_attacks(sq, occupied) | Self::rook_attacks(sq, occupied)
    }
}

/// Only one of each type will be used. No parallelism is used.
#[derive(Copy, Clone, Debug)]
pub struct L1;

impl Sealed for L1 {}

impl Level for L1 {
    type BitBoard = BitBoard;
    type Square = Square;

    const LEVEL: usize = 1;

    #[inline]
    fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
        ::magic::bishop_attacks(sq, occupied)
    }

    #[inline]
    fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
        ::magic::rook_attacks(sq, occupied)
    }
}

#[cfg(feature = "simd")]
macro_rules! levels {
    ($($d:expr, $l:ident, $n:expr, $bb:ty, $($tmp:ident),+;)+) => { $(
        #[doc = $d]
        #[derive(Copy, Clone, Debug)]
        pub struct $l;

        impl Sealed for $l {}

        impl Level for $l {
            type BitBoard = $bb;
            type Square = [Square; $n];

            const LEVEL: usize = $n;

            #[inline]
            fn bishop_attacks(sq: Self::Square, occupied: Self::BitBoard) -> Self::BitBoard {
                ::magic::simd::$l::bishop_attacks(sq, occupied)
            }

            #[inline]
            fn rook_attacks(sq: Self::Square, occupied: Self::BitBoard) -> Self::BitBoard {
                ::magic::simd::$l::rook_attacks(sq, occupied)
            }
        }
    )+ }
}

#[cfg(feature = "simd")]
levels! {
    "Two of each type will be used.",   L2, 2, u64x2, a, b;
    "Four of each type will be used.",  L4, 4, u64x4, a, b, c, d;
    "Eight of each type will be used.", L8, 8, u64x8, a, b, c, d, e, f, g, h;
}
