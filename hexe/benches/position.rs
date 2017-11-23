#![feature(test)]
extern crate test;
extern crate hexe;

use test::{Bencher, black_box};
use hexe::position::Position;
use hexe::prelude::*;

#[bench]
fn position_eq(b: &mut Bencher) {
    let x = Position::default();
    let y = Position::default();
    b.iter(|| {
        black_box(black_box(&x) == black_box(&y));
    });
}
