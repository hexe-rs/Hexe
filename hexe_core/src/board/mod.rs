//! Various board types.
//!
//! ## Board Representation
//!
//! There are two primary chess board representations provided. Both have
//! various advantages and disadvantages, which are outlined below:
//!
//! ### [`Bitboard`](bitboard/struct.Bitboard.html)
//!
//! **Mapping:** bit-to-square
//!
//! **Advantages:**
//!
//! - Throughput—excellent for performing parallel operations on the board
//!
//!     - Checking whether a file is empty
//!
//!     - Generating moves for all pieces
//!
//! **Disadvantages:**
//!
//! - Size—larger overall memory cost
//!
//!     - A common compact way of representing all pieces with bitboards is to
//!       have 6 × [`PieceKind`](../piece/enum.PieceKind.html) bitboards and 2 ×
//!       [`Color`](color/enum.Color.html) bitboards. This results in
//!       (2 + 6) × 8 = 64 bytes used to represent all pieces.
//!
//!     - Using 12 × [`Piece`](../piece/enum.Piece.html) bitboards is another
//!       representation of the entire chess board. This results in 12 × 8 = 96
//!       bytes used to represent all pieces.
//!
//!     - Operations are often done using 64-bit (8 byte) integers
//!
//! ### [`PieceMap`](piece_map/struct.PieceMap.html)
//!
//! **Mapping:** byte-to-square
//!
//! **Advantages:**
//!
//! - Latency—great for performing instantaneous operations on the board
//!
//!     - Finding whether a square is empty or otherwise what piece sits on it
//!
//! - Size—lower overall memory cost
//!
//!     - Operations usually done with a few bytes
//!
//! **Disadvantages:**
//!
//! - Size—larger upfront memory cost
//!
//!     - Uses exactly 64 bytes for each square on the board and its piece

pub mod bitboard;
pub mod multi_board;
pub mod piece_map;

#[doc(inline)] pub use self::bitboard::Bitboard;
#[doc(inline)] pub use self::multi_board::MultiBoard;
#[doc(inline)] pub use self::piece_map::PieceMap;
