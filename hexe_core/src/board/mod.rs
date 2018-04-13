//! Various board types.
//!
//! ## Board Representation
//!
//! There are three chess board representations provided. They each have various
//! advantages and disadvantages, which are outlined below:
//!
//! ### [`Bitboard`]
//!
//! **Mapping:** bit-to-[`Square`]
//!
//! **Advantages:**
//!
//! - Throughput—excellent for performing parallel operations on the board:
//!
//!     - Checking whether a file is empty
//!
//!     - Generating moves for all pieces
//!
//! **Disadvantages:**
//!
//! - Size—larger overall memory cost:
//!
//!     - A common compact way of representing all pieces with bitboards is to
//!       have 6 × [`Role`] bitboards and 2 × [`Color`] bitboards. This
//!       results in (2 + 6) × 8 = 64 bytes used to represent all pieces.
//!
//!       This is how [`MultiBoard`] works.
//!
//!     - Using 12 × [`Piece`](../piece/enum.Piece.html) bitboards is another
//!       representation of the entire chess board. This results in 12 × 8 = 96
//!       bytes used to represent all pieces.
//!
//!     - Operations are often done using 64-bit (8 byte) integers.
//!
//! ### [`MultiBoard`]
//!
//! **Mapping:** [`Color`]/[`Piece`]/[`Role`] to [`Bitboard`]
//!
//! **Advantages:**
//!
//! - Lookup—_very fast_ square retrieval:
//!
//!   ```
//!   # use hexe_core::board::MultiBoard;
//!   # use hexe_core::prelude::*;
//!   let board = MultiBoard::STANDARD;
//!
//!   let king = board.first(Piece::WhiteKing).unwrap();
//!   println!("White king found at {}", king);
//!
//!   for sq in board.bitboard(Color::White) {
//!       println!("A white piece at {}", sq);
//!   }
//!   ```
//!
//! **Disadvantages:**
//!
//! - Checking—slow to find the piece at a square
//!
//! ### [`PieceMap`]
//!
//! **Mapping:** [`Piece`] to [`Square`]
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
//!
//! [`Bitboard`]: bitboard/struct.Bitboard.html
//! [`MultiBoard`]: multi_board/struct.MultiBoard.html
//! [`PieceMap`]: piece_map/struct.PieceMap.html
//!
//! [`Color`]: ../color/enum.Color.html
//! [`Piece`]: ../piece/enum.Piece.html
//! [`Role`]: ../piece/enum.Role.html
//! [`Square`]: ../square/enum.Square.html

pub mod bitboard;
pub mod multi_board;
pub mod piece_map;

#[doc(inline)] pub use self::bitboard::Bitboard;
#[doc(inline)] pub use self::multi_board::MultiBoard;
#[doc(inline)] pub use self::piece_map::PieceMap;

/// Chess variants that Hexe supports (or plans to support).
#[derive(Copy, Clone, Debug)]
pub enum Variant {
    /// Standard vanilla chess.
    Standard,
    /// [Chess960](https://en.wikipedia.org/wiki/Chess960), where players' ranks
    /// are randomized prior to starting.
    ///
    /// This variant may also be called Fischer Random Chess.
    Chess960,
    #[doc(hidden)]
    // Here be dragons and nasal demons.
    // TODO: https://github.com/rust-lang/rust/issues/44109
    __NonExhaustive,
}

impl Default for Variant {
    #[inline]
    fn default() -> Variant { Variant::Standard }
}
