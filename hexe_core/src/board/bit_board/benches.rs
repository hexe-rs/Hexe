use test::{Bencher, black_box};

use super::*;
use util::rand_pairs;

macro_rules! impl_sliding_benches {
    ($($f:ident)+) => { $(
        #[bench]
        fn $f(b: &mut Bencher) {
            let pairs = rand_pairs::<BitBoard, BitBoard>();
            b.iter(|| {
                for &(bits, occ) in pairs.iter() {
                    black_box(black_box(bits).$f(!black_box(occ)));
                }
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }

#[bench]
fn iter(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(BitBoard::FULL) {
            black_box(sq);
        }
    });
}

#[bench]
fn iter_rev(b: &mut Bencher) {
    b.iter(|| {
        for sq in black_box(BitBoard::FULL).rev() {
            black_box(sq);
        }
    });
}
