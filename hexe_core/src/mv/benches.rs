use super::*;
use test::{Bencher, black_box};

fn gen_squares() -> [(Square, Square); 1000] {
    ::util::rand_pairs()
}

#[bench]
fn en_passant_try_new_1000(b: &mut Bencher) {
    let squares = gen_squares();
    b.iter(|| {
        for &(s1, s2) in squares.iter().map(black_box) {
            if let Some(mv) = kind::EnPassant::try_new(s1, s2) {
                black_box(mv);
            }
        }
    });
}

#[bench]
fn castle_try_new_1000(b: &mut Bencher) {
    let squares = gen_squares();
    b.iter(|| {
        for &(s1, s2) in squares.iter().map(black_box) {
            if let Some(mv) = kind::Castle::try_new(s1, s2) {
                black_box(mv);
            }
        }
    });
}
