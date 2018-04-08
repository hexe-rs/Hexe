use super::*;
use mv;

pub struct Tables {
    pub mb_masks: [(u64, u64); 4],
    pub chars: [u8; 4],
    pub moves: [u16; 4],
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
};
