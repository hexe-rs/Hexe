#![feature(test)]
extern crate test;
extern crate rand;
extern crate hexe_core;

use test::{Bencher, black_box};
use hexe_core::prelude::*;

macro_rules! impl_sliding_benches {
    ($($f:ident)+) => { $(
        #[bench]
        fn $f(b: &mut Bencher) {
            let occ = Bitboard(rand::random());
            b.iter(|| {
                for s in Square::all() {
                    black_box(black_box(s).$f(black_box(occ)));
                }
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }
