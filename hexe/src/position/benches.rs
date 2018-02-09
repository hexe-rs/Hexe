use super::*;
use test::{Bencher, black_box};

#[bench]
fn eq(b: &mut Bencher) {
    let x = Position::default();
    let y = Position::default();
    b.iter(|| {
        black_box(black_box(&x) == black_box(&y));
    });
}
