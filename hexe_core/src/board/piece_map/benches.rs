use super::{Piece, PieceMap};

use test::{Bencher, black_box};
use rand;

use castle_rights::CastleRight;
use iter::AllIterable;
use square::Square;

macro_rules! piece_map {
    ($($s:expr => $p:expr),* $(,)*) => {
        {
            #[allow(unused_mut)]
            let mut map = PieceMap::new();
            $(map.insert($s, $p);)*
            map
        }
    }
}

fn find_naive_impl(piece: Piece, map: &PieceMap) -> Option<Square> {
    for (i, &slot) in map.as_bytes().iter().enumerate() {
        if slot == piece as u8 {
            return Some(i.into())
        }
    }
    None
}

fn contains(piece: Piece, map: &PieceMap) -> bool {
    map.as_bytes().contains(&(piece as u8))
}

#[bench]
fn contains_piece(b: &mut Bencher) {
    let piece = Piece::BlackKing;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(black_box(&map).contains(black_box(piece)));
    });
}

#[bench]
fn contains_piece_naive(b: &mut Bencher) {
    let piece = Piece::BlackKing;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(contains(black_box(piece), black_box(&map)));
    });
}

#[bench]
fn find(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(black_box(&map).find(black_box(piece)));
    });
}

#[bench]
fn find_naive(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::H8 => piece };

    b.iter(|| {
        black_box(find_naive_impl(black_box(piece), black_box(&map)));
    });
}

#[bench]
fn rfind(b: &mut Bencher) {
    let piece = Piece::WhiteRook;
    let map = piece_map! { Square::A1 => piece };

    b.iter(|| {
        black_box(black_box(&map).rfind(black_box(piece)));
    });
}

#[bench]
fn iter_len(b: &mut Bencher) {
    let map = PieceMap::STANDARD;
    let mut iter = map.iter();

    iter.next();
    iter.next_back();

    b.iter(|| {
        black_box(black_box(&iter).len());
    });
}

#[bench]
fn len(b: &mut Bencher) {
    let mut map = PieceMap::STANDARD;
    map.shuffle(&mut rand::thread_rng());
    b.iter(|| {
        black_box(black_box(&map).len());
    });
}

#[bench]
fn len_naive(b: &mut Bencher) {
    fn len(map: &PieceMap) -> usize {
        map.as_bytes().iter().fold(64, |len, &pc| {
            len - (pc == 12) as usize
        })
    }

    let mut map = PieceMap::STANDARD;
    map.shuffle(&mut rand::thread_rng());

    assert_eq!(map.len(), len(&map));

    b.iter(|| {
        black_box(len(black_box(&map)));
    });
}

#[bench]
fn is_empty(b: &mut Bencher) {
    let map = piece_map!();
    b.iter(|| {
        black_box(black_box(&map).is_empty());
    });
}

#[bench]
fn eq(b: &mut Bencher) {
    let x = piece_map!();
    let y = piece_map!();
    b.iter(|| {
        black_box(black_box(&x) == black_box(&y));
    });
}

#[bench]
fn fen(b: &mut Bencher) {
    let map = PieceMap::STANDARD;
    b.iter(|| {
        black_box(&map).map_str(|s| {
            black_box(s);
        });
    });
}

#[bench]
fn castle(b: &mut Bencher) {
    b.iter(|| {
        let mut map = PieceMap([super::NONE; 64]);
        for right in CastleRight::ALL {
            black_box(&mut map).castle(right);
        }
        black_box(&mut map);
    });
}