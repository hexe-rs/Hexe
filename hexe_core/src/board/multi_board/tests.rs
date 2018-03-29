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

    macro_rules! iter {
        ($color:expr, $iter:expr) => {
            for sq in $iter.take(40) {
                test(false, sq, $color);
            }
            for sq in $iter.skip(8).take(16) {
                test(true, sq, !$color);
            }
        }
    }

    iter!(Color::White, Square::ALL);
    iter!(Color::Black, Square::ALL.rev());
}

#[test]
fn from_piece_map() {
    let pieces = PieceMap::STANDARD;
    let board  = MultiBoard::from(&pieces);
    assert!(board == MultiBoard::STANDARD);
}
