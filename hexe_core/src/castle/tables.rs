use super::*;
use board::piece_map::NONE;
use mv;
use piece::Piece::*;
use square::Square::*;

macro_rules! quad {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($d as u32) << 24) |
        (($c as u32) << 16) |
        (($b as u32) << 8)  |
        ( $a as u32)
    }
}

#[repr(align(64))]
pub struct Tables {
    pub mb_masks: [(u64, u64); 4],
    pub moves: [u16; 4],
    pub chars: [u8; 4],
    pub pm_value: [u32; 4],
    pub pm_pairs: [(Square, Square); 4],
}

pub static TABLES: Tables = Tables {
    mb_masks: [ // (King, Rook)
        (squares!(E1, G1), squares!(H1, F1)),
        (squares!(E1, C1), squares!(A1, D1)),
        (squares!(E8, G8), squares!(H8, F8)),
        (squares!(E8, C8), squares!(A8, D8)),
    ],
    chars: *b"KQkq",
    moves: [
        mv::kind::WK | mv::KIND_CASTLE | mv::META_WK,
        mv::kind::WQ | mv::KIND_CASTLE | mv::META_WQ,
        mv::kind::BK | mv::KIND_CASTLE | mv::META_BK,
        mv::kind::BQ | mv::KIND_CASTLE | mv::META_BQ,
    ],
    pm_value: [
        quad!(NONE, WhiteRook, WhiteKing, NONE),
        quad!(NONE, NONE,      WhiteKing, WhiteRook),
        quad!(NONE, BlackRook, BlackKing, NONE),
        quad!(NONE, NONE,      BlackKing, BlackRook),
    ],
    pm_pairs: [(E1, E1), (E1, A1), (E8, E8), (E8, A8)],
};
