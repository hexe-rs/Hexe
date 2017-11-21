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
            let sq  = Square::from(rand::random::<u8>());
            b.iter(|| {
                black_box(black_box(sq).$f(black_box(occ)));
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }

#[bench]
fn squares_iter(b: &mut test::Bencher) {
    b.iter(|| {
        for sq in black_box(Square::all()) {
            black_box(sq);
        }
    });
}

#[bench]
fn squares_iter_rev(b: &mut test::Bencher) {
    b.iter(|| {
        for sq in black_box(Square::all()).rev() {
            black_box(sq);
        }
    });
}
