use board::PieceMap;
use super::MultiBoard;
use prelude::*;

#[test]
fn is_attacked() {
    let board = MultiBoard::STANDARD;

    let test = |cond, sq, player| assert_eq!(
        cond, board.is_attacked(sq, player), "{:?} {:?}", sq, player
    );

    // Piece for A cannot be attacked by B when A is at a corner of B
    for color in Color::ALL {
        let rank = Rank::last(color);
        for &file in &[File::A, File::H] {
            test(false, Square::new(file, rank), color);
        }
    }

    for sq in Square::ALL.take(40) {
        test(false, sq, Color::White);
    }

    for sq in Square::ALL.skip(8).take(16) {
        test(true, sq, Color::Black);
    }

    for sq in Square::ALL.rev().take(40) {
        test(false, sq, Color::Black);
    }

    for sq in Square::ALL.rev().skip(8).take(16) {
        test(true, sq, Color::White);
    }
}

#[test]
fn from_piece_map() {
    let pieces = PieceMap::STANDARD;
    let board  = MultiBoard::from(&pieces);
    assert!(board == MultiBoard::STANDARD);
}
