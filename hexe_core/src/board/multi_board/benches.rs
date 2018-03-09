use super::MultiBoard;
use castle_rights::CastleRight;
use iter::AllIterable;
use square::Square;

use test::{Bencher, black_box};

#[bench]
fn remove_squares_10(b: &mut Bencher) {
    let squares = [
        Square::B1, Square::C2, Square::H8, Square::A7, Square::A1,
        Square::B8, Square::C7, Square::H1, Square::A2, Square::A8,
    ];
    b.iter(|| {
        let mut board = MultiBoard::STANDARD;
        let squares = black_box(&squares[..]);
        black_box(&mut board).remove_squares(squares.iter().cloned());
        black_box(&board);
    });
}

#[bench]
fn castle_all(b: &mut Bencher) {
    b.iter(|| {
        let mut board = MultiBoard::default();
        for right in CastleRight::ALL {
            black_box(&mut board).castle(right);
        }
        black_box(&mut board);
    });
}
