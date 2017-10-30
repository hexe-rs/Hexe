#![feature(test)]
extern crate test;
extern crate hexe_core;

use test::{Bencher, black_box};
use hexe_core::piece::*;
use hexe_core::square::Square;

macro_rules! piece_map {
    ($($s:expr => $p:expr),* $(,)*) => {
        {
            #[allow(unused_mut)]
            let mut map = map::PieceMap::new();
            $(map.insert($s, $p);)*
            map
        }
    }
}

fn find(piece: Piece, map: &map::PieceMap) -> Option<Square> {
    for (i, &slot) in map.as_bytes().iter().enumerate() {
        if slot == piece as u8 {
            return Some(i.into())
        }
    }
    None
}

fn contains(piece: Piece, map: &map::PieceMap) -> bool {
    map.as_bytes().contains(&(piece as u8))
}

#[bench]
fn map_contains_piece(b: &mut Bencher) {
    let piece = Piece::BlackKing;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(black_box(&map).contains(black_box(piece)));
    });
}

#[bench]
fn map_contains_piece_naive(b: &mut Bencher) {
    let piece = Piece::BlackKing;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(contains(black_box(piece), black_box(&map)));
    });
}

#[bench]
fn map_find(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(black_box(&map).find(black_box(piece)));
    });
}

#[bench]
fn map_find_naive(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(find(black_box(piece), black_box(&map)));
    });
}

#[bench]
fn map_rfind(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::A1 => piece };

    b.iter(|| {
        black_box(black_box(&map).rfind(black_box(piece)));
    });
}

#[bench]
fn map_iter_len(b: &mut Bencher) {
    let map = map::PieceMap::STANDARD;
    let mut iter = map.iter();

    iter.next();
    iter.next_back();

    b.iter(|| {
        black_box(black_box(&iter).len());
    });
}

#[bench]
fn map_len(b: &mut Bencher) {
    let map = piece_map!();
    b.iter(|| {
        black_box(black_box(&map).len());
    });
}

#[bench]
fn map_is_empty(b: &mut Bencher) {
    let map = piece_map!();
    b.iter(|| {
        black_box(black_box(&map).is_empty());
    });
}

#[bench]
fn map_eq(b: &mut Bencher) {
    let x = piece_map!();
    let y = piece_map!();
    b.iter(|| {
        black_box(black_box(&x) == black_box(&y));
    });
}
