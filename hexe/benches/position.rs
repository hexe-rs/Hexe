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

#[bench]
fn position_color_64(b: &mut Bencher) {
    let pos = Position::default();
    b.iter(|| {
        for sq in Square::all().map(black_box) {
            black_box(black_box(&pos).color_at(sq));
        }
    });
}
