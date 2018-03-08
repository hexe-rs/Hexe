//! Various board representations.

pub mod bitboard;
pub mod multi_board;
pub mod piece_map;

pub use self::bitboard::Bitboard;
pub use self::multi_board::MultiBoard;
pub use self::piece_map::PieceMap;
