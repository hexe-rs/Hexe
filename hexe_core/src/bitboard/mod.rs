//! A bitmap chess board representation.

pub mod masks;
mod impls;

/// A mapping of sixty-four bits to squares of a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// White board squares.
    pub const WHITE: Bitboard = Bitboard(!Self::BLACK.0);

    /// Black board squares.
    pub const BLACK: Bitboard = Bitboard(0xAA55AA55AA55AA55);
}
