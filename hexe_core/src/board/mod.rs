//! Various board representations.

pub mod bitboard;
pub mod multi_board;
pub mod piece_map;

#[doc(inline)] pub use self::bitboard::Bitboard;
#[doc(inline)] pub use self::multi_board::MultiBoard;
#[doc(inline)] pub use self::piece_map::PieceMap;
