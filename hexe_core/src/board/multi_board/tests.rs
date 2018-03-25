use board::PieceMap;
use super::MultiBoard;

#[test]
fn from_piece_map() {
    let pieces = PieceMap::STANDARD;
    let board  = MultiBoard::from(&pieces);
    assert!(board == MultiBoard::STANDARD);
}
