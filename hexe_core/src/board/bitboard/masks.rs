//! Bitboard masks for each file and rank.

#![allow(missing_docs)]

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
