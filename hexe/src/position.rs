//! A chess game state position.

use hexe_core::piece::map::PieceMap;

/// A representation of the current game state.
#[derive(PartialEq, Eq)]
pub struct Position {
    piece_map: PieceMap,
    pieces: [u64; 6],
    colors: [u64; 2],
}

impl Default for Position {
    fn default() -> Position {
        const PAWN:   u64 = 0x00FF00000000FF00;
        const KNIGHT: u64 = 0x4200000000000042;
        const BISHOP: u64 = 0x2400000000000024;
        const ROOK:   u64 = 0x8100000000000081;
        const QUEEN:  u64 = 0x0800000000000008;
        const KING:   u64 = 0x1000000000000010;
        const WHITE:  u64 = 0x000000000000FFFF;
        const BLACK:  u64 = 0xFFFF000000000000;

        Position {
            piece_map: PieceMap::STANDARD,
            pieces: [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING],
            colors: [WHITE, BLACK],
        }
    }
}
