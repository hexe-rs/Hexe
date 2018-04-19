use test::{Bencher, black_box};

use super::*;
use misc::Direction;
use util::rand_pairs;

macro_rules! impl_sliding_benches {
    ($($f:ident)+) => { $(
        #[bench]
        fn $f(b: &mut Bencher) {
            let pairs = rand_pairs::<Square, Bitboard>();
            b.iter(|| {
                for &(sq, occ) in pairs.iter() {
                    black_box(black_box(sq).$f(black_box(occ)));
                }
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }

#[bench]
fn shift_1000(b: &mut Bencher) {
    let pairs = rand_pairs::<Square, Direction>();
    b.iter(|| {
        for &(s, d) in pairs.iter() {
            black_box(black_box(s).shift(black_box(d)));
        }
    });
}

#[bench]
fn wrapping_shift_1000(b: &mut Bencher) {
    let pairs = rand_pairs::<Square, Direction>();
    b.iter(|| {
        for &(s, d) in pairs.iter() {
            black_box(black_box(s).wrapping_shift(black_box(d)));
        }
    });
}

#[bench]
fn iter(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(Square::ALL) {
            black_box(sq);
        }
    });
}

#[bench]
fn iter_rev(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(Square::ALL).rev() {
            black_box(sq);
        }
    });
}

#[bench]
fn color(b: &mut Bencher) {
    b.iter(|| {
        for sq in Square::ALL {
            black_box(black_box(sq).color());
        }
    })
}

#[bench]
fn distance_1000(b: &mut Bencher) {
    let squares = rand_pairs::<Square, Square>();
    b.iter(|| {
        for &(s1, s2) in squares.iter() {
            black_box(black_box(s1).distance(black_box(s2)));
        }
    });
}

#[bench]
fn distance_normal_1000(b: &mut Bencher) {
    fn distance(s1: Square, s2: Square) -> usize {
        use core::cmp::max;
        max(s1.file().distance(s2.file()), s1.rank().distance(s2.rank()))
    }
    let squares = rand_pairs::<Square, Square>();
    b.iter(|| {
        for &(s1, s2) in squares.iter() {
            black_box(distance(black_box(s1), black_box(s2)));
        }
    });
}
