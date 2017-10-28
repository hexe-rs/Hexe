#![feature(test)]
extern crate test;
extern crate hexe_core;

use test::{Bencher, black_box};
use hexe_core::piece::*;
use hexe_core::square::Square;

#[bench]
fn contains_piece(b: &mut Bencher) {
    let piece = Piece::BlackKing;
    let mut map = map::PieceMap::new();
    map.insert(Square::H8, piece);
    b.iter(|| {
        black_box(black_box(map).contains(black_box(piece)))
    });
}
