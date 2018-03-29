use super::MultiBoard;
use castle::Right;
use color::Color;
use iter::AllIterable;
use square::Square;

use test::{Bencher, black_box};

#[inline(always)]
fn is_attacked(board: &MultiBoard, b: &mut Bencher) {
    let board = black_box(board);
    let color = Color::White;
    b.iter(|| {
        for sq in Square::ALL.map(black_box) {
            let board = black_box(board);
            let color = black_box(color);
            let value = board.is_attacked(sq, color);
            black_box(value);
        }
    });
}

#[bench]
fn is_attacked_empty_64(b: &mut Bencher) {
    is_attacked(&MultiBoard::default(), b);
}

#[bench]
fn is_attacked_standard_64(b: &mut Bencher) {
    is_attacked(&MultiBoard::STANDARD, b);
}

#[bench]
fn from_piece_map(b: &mut Bencher) {
    let map = ::board::PieceMap::STANDARD;
    b.iter(|| {
        let board = MultiBoard::from(black_box(&map));
        black_box(&board);
    });
}

#[bench]
fn remove_all_10(b: &mut Bencher) {
    let squares = [
        Square::B1, Square::C2, Square::H8, Square::A7, Square::A1,
        Square::B8, Square::C7, Square::H1, Square::A2, Square::A8,
    ];
    b.iter(|| {
        let mut board = MultiBoard::STANDARD;
        let squares = black_box(&squares[..]);
        for &square in squares {
            black_box(&mut board).remove_all(square);
        }
        black_box(&board);
    });
}

#[bench]
fn len_1000(b: &mut Bencher) {
    let board = MultiBoard::STANDARD;
    b.iter(|| {
        for _ in 0..1000 {
            black_box(black_box(&board).len());
        }
    });
}

#[bench]
fn is_empty_1000(b: &mut Bencher) {
    let board = MultiBoard::default();
    b.iter(|| {
        for _ in 0..1000 {
            black_box(black_box(&board).is_empty());
        }
    });
}

#[bench]
fn eq(b: &mut Bencher) {
    let x = MultiBoard::STANDARD;
    let y = MultiBoard::STANDARD;
    b.iter(|| {
        black_box(black_box(&x) == black_box(&y));
    });
}

#[bench]
fn castle_all(b: &mut Bencher) {
    b.iter(|| {
        let mut board = MultiBoard::default();
        for right in Right::ALL {
            black_box(&mut board).castle(right);
        }
        black_box(&mut board);
    });
}
