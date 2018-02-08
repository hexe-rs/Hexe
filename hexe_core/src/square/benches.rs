use super::*;
use test::{Bencher, black_box};
use rand::{Rand, self};

macro_rules! impl_sliding_benches {
    ($($f:ident)+) => { $(
        #[bench]
        fn $f(b: &mut Bencher) {
            let pairs = rand_pairs::<Square, Bitboard>(1000);
            b.iter(|| {
                for &(sq, occ) in &pairs {
                    black_box(black_box(sq).$f(black_box(occ)));
                }
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }

fn rand_pairs<T: Rand, U: Rand>(n: usize) -> Vec<(T, U)> {
    (0..n).map(|_| (rand::random(), rand::random())).collect()
}

#[bench]
fn squares_iter(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(Square::ALL) {
            black_box(sq);
        }
    });
}

#[bench]
fn squares_iter_rev(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(Square::ALL).rev() {
            black_box(sq);
        }
    });
}

#[bench]
fn square_color(b: &mut Bencher) {
    b.iter(|| {
        for sq in Square::ALL {
            black_box(black_box(sq).color());
        }
    })
}

#[bench]
fn square_distance_1000(b: &mut Bencher) {
    let squares = rand_pairs::<Square, Square>(1000);
    b.iter(|| {
        for &(s1, s2) in &squares {
            black_box(black_box(s1).distance(black_box(s2)));
        }
    });
}

#[bench]
fn square_distance_normal_1000(b: &mut Bencher) {
    fn distance(s1: Square, s2: Square) -> usize {
        use std::cmp::max;
        max(s1.file().distance(s2.file()), s1.rank().distance(s2.rank()))
    }
    let squares = rand_pairs::<Square, Square>(1000);
    b.iter(|| {
        for &(s1, s2) in &squares {
            black_box(distance(black_box(s1), black_box(s2)));
        }
    });
}