use test::{Bencher, black_box};

use super::*;
use util::rand_pairs;

macro_rules! impl_sliding_benches {
    ($($f:ident)+) => { $(
        #[bench]
        fn $f(b: &mut Bencher) {
            let pairs = rand_pairs::<Bitboard, Bitboard>();
            b.iter(|| {
                for &(bits, occ) in pairs.iter() {
                    black_box(black_box(bits).$f(!black_box(occ)));
                }
            });
        }
    )+ }
}

impl_sliding_benches! { rook_attacks bishop_attacks queen_attacks }
